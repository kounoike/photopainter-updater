#include "update_job.h"

#include <stdio.h>
#include <string.h>

#include "button_bsp.h"
#include "config.h"
#include "display_update.h"
#include "driver/rtc_io.h"
#include "esp_event.h"
#include "esp_log.h"
#include "esp_netif.h"
#include "esp_sleep.h"
#include "esp_wifi.h"
#include "freertos/event_groups.h"
#include "freertos/queue.h"
#include "freertos/task.h"
#include "sdcard_bsp.h"

namespace {

constexpr const char* kTag = "fw_update";
constexpr char kConfigPath[] = "/sdcard/config.txt";
constexpr char kImageCachePath[] = "/sdcard/download.bmp";
constexpr int kWifiConnectedBit = BIT0;
constexpr int kWifiFailedBit = BIT1;
constexpr gpio_num_t kBootWakeupPin = GPIO_NUM_0;
constexpr int kWifiTimeoutMs = 30000;

StaticQueue_t s_queue_storage;
uint8_t s_queue_buffer[sizeof(UpdateTrigger)];
QueueHandle_t s_update_queue = nullptr;
portMUX_TYPE s_update_lock = portMUX_INITIALIZER_UNLOCKED;
bool s_update_pending = false;
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

[[noreturn]] void HoldForDevelopment(const char* reason) {
    for (;;) {
        ESP_LOGW(kTag, "Development hold active: %s", reason == nullptr ? "" : reason);
        vTaskDelay(pdMS_TO_TICKS(5000));
    }
}

void ReleasePendingSlot() {
    portENTER_CRITICAL(&s_update_lock);
    s_update_pending = false;
    portEXIT_CRITICAL(&s_update_lock);
}

void ShutdownAfterFailure() {
    ESP_LOGW(kTag, "Failure shutdown is disabled in the current development build");
}

void HandleFailure(UpdateTrigger trigger, FailureCategory category, const char* detail) {
    RecordFailureState(trigger, category, detail);
    ReleasePendingSlot();
    ShutdownAfterFailure();
    HoldForDevelopment(detail);
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

    err = DownloadImageToSdCard(config.image_url, kImageCachePath, detail, sizeof(detail));
    if (err != ESP_OK) {
        HandleFailure(trigger, FailureCategory::kHttpError, detail);
        return err;
    }

    err = RenderBmpFromSdCard(kImageCachePath, detail, sizeof(detail));
    if (err != ESP_OK) {
        HandleFailure(trigger, FailureCategory::kImageError, detail);
        return err;
    }

    ClearLastFailureState();
    ESP_LOGI(kTag, "Update finished successfully for trigger=%s", UpdateTriggerToString(trigger));
    return ESP_OK;
}

void UpdateWorkerTask(void* arg) {
    for (;;) {
        UpdateTrigger trigger = UpdateTrigger::kStartup;
        if (xQueueReceive(s_update_queue, &trigger, portMAX_DELAY) != pdTRUE) {
            continue;
        }

        esp_err_t err = RunUpdate(trigger);
        if (err == ESP_OK) {
            ReleasePendingSlot();
        }
    }
}

void BootButtonTask(void* arg) {
    for (;;) {
        EventBits_t bits = xEventGroupWaitBits(boot_groups, set_bit_button(0), pdTRUE, pdFALSE, portMAX_DELAY);
        if (bits & set_bit_button(0)) {
            esp_err_t err = EnqueueUpdate(UpdateTrigger::kBootButton);
            if (err != ESP_OK) {
                ESP_LOGW(kTag, "Ignored BOOT update request because another update is already pending");
            }
        }
    }
}

}  // namespace

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
