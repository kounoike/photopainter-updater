#include "update_job.h"

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

#include "button_bsp.h"
#include "config.h"
#include "display_update.h"
#include "driver/gpio.h"
#include "driver/rtc_io.h"
#include "esp_event.h"
#include "esp_log.h"
#include "esp_netif.h"
#include "esp_sleep.h"
#include "esp_wifi.h"
#include "freertos/event_groups.h"
#include "freertos/queue.h"
#include "freertos/task.h"
#include "led_bsp.h"
#include "sdcard_bsp.h"

namespace {

constexpr const char* kTag = "fw_update";
constexpr char kConfigPath[] = "/sdcard/config.txt";
constexpr char kImageCachePath[] = "/sdcard/download.bmp";
constexpr int kWifiConnectedBit = BIT0;
constexpr int kWifiFailedBit = BIT1;
constexpr gpio_num_t kBootWakeupPin = GPIO_NUM_0;
constexpr int kWifiTimeoutMs = 30000;
constexpr uint8_t kActivityLedPin = LED_PIN_Green;
constexpr TickType_t kActivityLedBlinkInterval = pdMS_TO_TICKS(500);
constexpr TickType_t kActivityLedIdlePollInterval = pdMS_TO_TICKS(50);
constexpr char kDebugSleepBypassPath[] = "/sdcard/debug.txt";

StaticQueue_t s_queue_storage;
uint8_t s_queue_buffer[sizeof(UpdateTrigger)];
QueueHandle_t s_update_queue = nullptr;
portMUX_TYPE s_update_lock = portMUX_INITIALIZER_UNLOCKED;
bool s_update_pending = false;
TaskHandle_t s_activity_led_task = nullptr;
portMUX_TYPE s_activity_led_lock = portMUX_INITIALIZER_UNLOCKED;
bool s_activity_led_blinking = false;
bool s_activity_led_initialized = false;
UpdateTrigger s_activity_led_owner = UpdateTrigger::kStartup;
bool s_wifi_initialized = false;
EventGroupHandle_t s_wifi_event_group = nullptr;
int s_wifi_retry_count = 0;
esp_event_handler_instance_t s_wifi_handler_any_id;
esp_event_handler_instance_t s_ip_handler_got_ip;

void SetErrorDetail(char* buffer, size_t buffer_size, const char* message) {
    if (buffer == nullptr || buffer_size == 0) {
        return;
    }
    snprintf(buffer, buffer_size, "%s", message);
}

void ReleasePendingSlot() {
    portENTER_CRITICAL(&s_update_lock);
    s_update_pending = false;
    portEXIT_CRITICAL(&s_update_lock);
}

bool ShouldBypassDeepSleep() {
    struct stat st = {};
    bool file_bypass = stat(kDebugSleepBypassPath, &st) == 0;
    bool developer_mode = IsDeveloperModeEnabled();
    ESP_LOGI(kTag, "Deep sleep bypass file %s: %s, developer mode: %s", kDebugSleepBypassPath,
             file_bypass ? "present" : "absent", developer_mode ? "enabled" : "disabled");
    return file_bypass || developer_mode;
}

void SetActivityLedLevel(bool on) {
    if (!s_activity_led_initialized) {
        return;
    }
    gpio_set_level(static_cast<gpio_num_t>(kActivityLedPin), on ? LED_ON : LED_OFF);
}

void StopActivityLedLocked() {
    s_activity_led_blinking = false;
}

void StartActivityLed(UpdateTrigger trigger) {
    if (IsDeveloperModeEnabled()) {
        ESP_LOGW(kTag, "Developer mode active, skipping activity LED enable");
        return;
    }
    portENTER_CRITICAL(&s_activity_led_lock);
    s_activity_led_owner = trigger;
    s_activity_led_blinking = true;
    portEXIT_CRITICAL(&s_activity_led_lock);
    ESP_LOGI(kTag, "Activity LED blinking started for trigger=%s on GPIO %u", UpdateTriggerToString(trigger),
             static_cast<unsigned>(kActivityLedPin));
}

void ActivityLedTask(void* arg) {
    bool led_on = false;
    for (;;) {
        bool blinking = false;
        portENTER_CRITICAL(&s_activity_led_lock);
        blinking = s_activity_led_blinking;
        portEXIT_CRITICAL(&s_activity_led_lock);

        if (!blinking) {
            if (led_on) {
                SetActivityLedLevel(false);
                led_on = false;
            }
            vTaskDelay(kActivityLedIdlePollInterval);
            continue;
        }

        led_on = !led_on;
        SetActivityLedLevel(led_on);
        vTaskDelay(kActivityLedBlinkInterval);
    }
}

[[noreturn]] void EnterDeepSleep(const char* reason) {
    if (ShouldBypassDeepSleep()) {
        ESP_LOGW(kTag, "Skipping deep sleep because %s exists", kDebugSleepBypassPath);
        for (;;) {
            vTaskDelay(portMAX_DELAY);
        }
    }

    ESP_LOGI(kTag, "Entering deep sleep: %s", reason == nullptr ? "" : reason);
    ESP_ERROR_CHECK_WITHOUT_ABORT(esp_sleep_disable_wakeup_source(ESP_SLEEP_WAKEUP_ALL));
    ESP_ERROR_CHECK_WITHOUT_ABORT(rtc_gpio_pullup_en(kBootWakeupPin));
    ESP_ERROR_CHECK_WITHOUT_ABORT(rtc_gpio_pulldown_dis(kBootWakeupPin));
    ESP_ERROR_CHECK_WITHOUT_ABORT(esp_sleep_enable_ext0_wakeup(kBootWakeupPin, 0));
    vTaskDelay(pdMS_TO_TICKS(500));
    esp_deep_sleep_start();
    for (;;) {
        vTaskDelay(portMAX_DELAY);
    }
}

void HandleFailure(UpdateTrigger trigger, FailureCategory category, const char* detail) {
    RecordFailureState(trigger, category, detail);
    StopActivityLed();
    ReleasePendingSlot();
    EnterDeepSleep(detail);
}

void WifiEventHandler(void* arg, esp_event_base_t event_base, int32_t event_id, void* event_data) {
    if (event_base == WIFI_EVENT && event_id == WIFI_EVENT_STA_START) {
        esp_wifi_connect();
    } else if (event_base == WIFI_EVENT && event_id == WIFI_EVENT_STA_DISCONNECTED) {
        if (s_wifi_retry_count < 1) {
            ++s_wifi_retry_count;
            esp_wifi_connect();
        } else {
            xEventGroupSetBits(s_wifi_event_group, kWifiFailedBit);
        }
    } else if (event_base == IP_EVENT && event_id == IP_EVENT_STA_GOT_IP) {
        s_wifi_retry_count = 0;
        xEventGroupSetBits(s_wifi_event_group, kWifiConnectedBit);
    }
}

esp_err_t EnsureWifiInitialized() {
    if (s_wifi_initialized) {
        return ESP_OK;
    }

    s_wifi_event_group = xEventGroupCreate();
    if (s_wifi_event_group == nullptr) {
        return ESP_ERR_NO_MEM;
    }

    esp_netif_t* wifi_netif = esp_netif_create_default_wifi_sta();
    if (wifi_netif == nullptr) {
        return ESP_FAIL;
    }

    wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
    esp_err_t err = esp_wifi_init(&cfg);
    if (err != ESP_OK) {
        return err;
    }
    err = esp_event_handler_instance_register(WIFI_EVENT, ESP_EVENT_ANY_ID, &WifiEventHandler, nullptr,
                                              &s_wifi_handler_any_id);
    if (err != ESP_OK) {
        return err;
    }
    err = esp_event_handler_instance_register(IP_EVENT, IP_EVENT_STA_GOT_IP, &WifiEventHandler, nullptr,
                                              &s_ip_handler_got_ip);
    if (err != ESP_OK) {
        return err;
    }
    err = esp_wifi_set_mode(WIFI_MODE_STA);
    if (err != ESP_OK) {
        return err;
    }
    err = esp_wifi_start();
    if (err != ESP_OK) {
        return err;
    }
    s_wifi_initialized = true;
    return ESP_OK;
}

esp_err_t ConnectWifi(const FirmwareConfig& config, char* error_detail, size_t error_detail_size) {
    esp_err_t init_err = EnsureWifiInitialized();
    if (init_err != ESP_OK) {
        SetErrorDetail(error_detail, error_detail_size, "WiFi subsystem initialization failed");
        return init_err;
    }

    wifi_config_t wifi_config = {};
    strlcpy(reinterpret_cast<char*>(wifi_config.sta.ssid), config.wifi_ssid, sizeof(wifi_config.sta.ssid));
    strlcpy(reinterpret_cast<char*>(wifi_config.sta.password), config.wifi_password, sizeof(wifi_config.sta.password));
    wifi_config.sta.threshold.authmode = WIFI_AUTH_WPA2_PSK;
    wifi_config.sta.pmf_cfg.capable = true;
    wifi_config.sta.pmf_cfg.required = false;

    s_wifi_retry_count = 0;
    xEventGroupClearBits(s_wifi_event_group, kWifiConnectedBit | kWifiFailedBit);
    esp_wifi_disconnect();
    esp_err_t err = esp_wifi_set_config(WIFI_IF_STA, &wifi_config);
    if (err != ESP_OK) {
        SetErrorDetail(error_detail, error_detail_size, "failed to apply WiFi configuration");
        return err;
    }
    err = esp_wifi_connect();
    if (err != ESP_OK) {
        SetErrorDetail(error_detail, error_detail_size, "failed to start WiFi connection");
        return err;
    }

    EventBits_t bits = xEventGroupWaitBits(s_wifi_event_group, kWifiConnectedBit | kWifiFailedBit, pdTRUE, pdFALSE,
                                           pdMS_TO_TICKS(kWifiTimeoutMs));
    if (bits & kWifiConnectedBit) {
        return ESP_OK;
    }

    SetErrorDetail(error_detail, error_detail_size, "failed to connect to configured WiFi");
    return ESP_ERR_TIMEOUT;
}

esp_err_t RunUpdate(UpdateTrigger trigger) {
    char detail[160] = {};
    FirmwareConfig config = {};
    ESP_LOGI(kTag, "Update job started: trigger=%s", UpdateTriggerToString(trigger));
    StartActivityLed(trigger);

    esp_err_t err = LoadConfigFromSdCard(kConfigPath, &config, detail, sizeof(detail));
    if (err != ESP_OK) {
        HandleFailure(trigger, FailureCategory::kConfigError, detail);
        return err;
    }

    err = ConnectWifi(config, detail, sizeof(detail));
    if (err != ESP_OK) {
        HandleFailure(trigger, FailureCategory::kWifiError, detail);
        return err;
    }
    bool use_binary_path = IsBinaryImageUrl(config.image_url);
    ESP_LOGI(kTag, "WiFi connected, starting HTTP update for trigger=%s route=%s", UpdateTriggerToString(trigger),
             use_binary_path ? "binary" : "bmp");

    if (use_binary_path) {
        err = DownloadBinaryFrameToDisplay(config.image_url, detail, sizeof(detail));
        if (err != ESP_OK) {
            FailureCategory category =
                (err == ESP_ERR_INVALID_RESPONSE || err == ESP_ERR_INVALID_SIZE || err == ESP_ERR_INVALID_CRC)
                    ? FailureCategory::kImageError
                    : FailureCategory::kHttpError;
            HandleFailure(trigger, category, detail);
            return err;
        }
        ESP_LOGI(kTag, "Binary frame render completed");
    } else {
        err = DownloadImageToSdCard(config.image_url, kImageCachePath, detail, sizeof(detail));
        if (err != ESP_OK) {
            HandleFailure(trigger, FailureCategory::kHttpError, detail);
            return err;
        }
        ESP_LOGI(kTag, "HTTP download completed, starting BMP render");

        err = RenderBmpFromSdCard(kImageCachePath, detail, sizeof(detail));
        if (err != ESP_OK) {
            HandleFailure(trigger, FailureCategory::kImageError, detail);
            return err;
        }
        ESP_LOGI(kTag, "BMP render completed");
    }

    ClearLastFailureState();
    StopActivityLed();
    ESP_LOGI(kTag, "Update finished successfully for trigger=%s", UpdateTriggerToString(trigger));
    ReleasePendingSlot();
    EnterDeepSleep("update finished successfully");
}

void UpdateWorkerTask(void* arg) {
    for (;;) {
        UpdateTrigger trigger = UpdateTrigger::kStartup;
        if (xQueueReceive(s_update_queue, &trigger, portMAX_DELAY) != pdTRUE) {
            continue;
        }

        RunUpdate(trigger);
    }
}

void BootButtonTask(void* arg) {
    for (;;) {
        EventBits_t bits =
            xEventGroupWaitBits(boot_groups, set_bit_button(0) | set_bit_button(1), pdTRUE, pdFALSE, portMAX_DELAY);
        if (bits & set_bit_button(1)) {
            bool enabled = false;
            esp_err_t err = ToggleDeveloperMode(&enabled);
            if (err != ESP_OK) {
                ESP_LOGE(kTag, "Failed to toggle developer mode: %s", esp_err_to_name(err));
            } else {
                StopActivityLed();
                ESP_LOGW(kTag, "Developer mode %s via BOOT long press", enabled ? "enabled" : "disabled");
            }
        }
        if (bits & set_bit_button(0)) {
            esp_err_t err = EnqueueUpdate(UpdateTrigger::kBootButton);
            if (err != ESP_OK) {
                ESP_LOGW(kTag, "Ignored BOOT update request because another update is already pending");
            }
        }
    }
}

}  // namespace

esp_err_t InitializeActivityLedControl() {
    if (IsDeveloperModeEnabled()) {
        ESP_LOGW(kTag, "Developer mode active, skipping activity LED initialization");
        return ESP_OK;
    }

    if (s_activity_led_task != nullptr) {
        StopActivityLed();
        return ESP_OK;
    }

    gpio_config_t gpio_conf = {};
    gpio_conf.intr_type = GPIO_INTR_DISABLE;
    gpio_conf.mode = GPIO_MODE_OUTPUT;
    gpio_conf.pin_bit_mask = (1ULL << kActivityLedPin);
    gpio_conf.pull_down_en = GPIO_PULLDOWN_DISABLE;
    gpio_conf.pull_up_en = GPIO_PULLUP_ENABLE;
    esp_err_t gpio_err = gpio_config(&gpio_conf);
    if (gpio_err != ESP_OK) {
        return gpio_err;
    }

    s_activity_led_initialized = true;
    SetActivityLedLevel(false);

    BaseType_t task_created = xTaskCreate(ActivityLedTask, "fw_activity_led", 4096, nullptr, 3, &s_activity_led_task);
    if (task_created != pdPASS) {
        s_activity_led_initialized = false;
        s_activity_led_task = nullptr;
        return ESP_ERR_NO_MEM;
    }

    ESP_LOGI(kTag, "Activity LED control initialized on green LED GPIO %u", static_cast<unsigned>(kActivityLedPin));
    return ESP_OK;
}

esp_err_t InitializeUpdateJobSystem() {
    if (s_update_queue == nullptr) {
        s_update_queue = xQueueCreateStatic(1, sizeof(UpdateTrigger), s_queue_buffer, &s_queue_storage);
        if (s_update_queue == nullptr) {
            return ESP_ERR_NO_MEM;
        }
    }

    BaseType_t worker_created = xTaskCreate(UpdateWorkerTask, "fw_update_worker", 8192, nullptr, 5, nullptr);
    if (worker_created != pdPASS) {
        return ESP_ERR_NO_MEM;
    }

    BaseType_t boot_created = xTaskCreate(BootButtonTask, "fw_boot_button", 4096, nullptr, 4, nullptr);
    if (boot_created != pdPASS) {
        return ESP_ERR_NO_MEM;
    }

    return ESP_OK;
}

void StopActivityLed() {
    bool was_blinking = false;
    UpdateTrigger owner = UpdateTrigger::kStartup;
    portENTER_CRITICAL(&s_activity_led_lock);
    was_blinking = s_activity_led_blinking;
    owner = s_activity_led_owner;
    StopActivityLedLocked();
    portEXIT_CRITICAL(&s_activity_led_lock);

    SetActivityLedLevel(false);
    if (was_blinking) {
        ESP_LOGI(kTag, "Activity LED blinking stopped for trigger=%s", UpdateTriggerToString(owner));
    }
}

esp_err_t EnqueueUpdate(UpdateTrigger trigger) {
    bool can_enqueue = false;

    portENTER_CRITICAL(&s_update_lock);
    if (!s_update_pending) {
        s_update_pending = true;
        can_enqueue = true;
    }
    portEXIT_CRITICAL(&s_update_lock);

    if (!can_enqueue) {
        return ESP_ERR_INVALID_STATE;
    }

    if (xQueueSend(s_update_queue, &trigger, 0) != pdTRUE) {
        ReleasePendingSlot();
        return ESP_FAIL;
    }
    return ESP_OK;
}
