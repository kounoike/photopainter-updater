#include <stdio.h>
#include "axp_prot.h"
#include "button_bsp.h"
#include "display_update.h"
#include "epaper_port.h"
#include "esp_event.h"
#include "esp_log.h"
#include "esp_netif.h"
#include "esp_sleep.h"
#include "failure_state.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "i2c_bsp.h"
#include "nvs_flash.h"
#include "sdcard_bsp.h"
#include "update_job.h"

namespace {

constexpr const char* kTag = "fw_main";

[[noreturn]] void HoldForDevelopment(const char* reason) {
    for (;;) {
        ESP_LOGW(kTag, "Development hold active: %s", reason == nullptr ? "" : reason);
        vTaskDelay(pdMS_TO_TICKS(5000));
    }
}

esp_err_t InitializeNvs() {
    esp_err_t err = nvs_flash_init();
    if (err == ESP_ERR_NVS_NO_FREE_PAGES || err == ESP_ERR_NVS_NEW_VERSION_FOUND) {
        ESP_ERROR_CHECK(nvs_flash_erase());
        err = nvs_flash_init();
    }
    return err;
}

}  // namespace

extern "C" void app_main(void) {
    esp_err_t err = esp_netif_init();
    if (err != ESP_OK) {
        ESP_LOGE(kTag, "esp_netif_init failed: %s", esp_err_to_name(err));
        HoldForDevelopment("esp_netif_init failed");
    }

    err = esp_event_loop_create_default();
    if (err != ESP_OK && err != ESP_ERR_INVALID_STATE) {
        ESP_LOGE(kTag, "esp_event_loop_create_default failed: %s", esp_err_to_name(err));
        HoldForDevelopment("esp_event_loop_create_default failed");
    }

    err = InitializeNvs();
    if (err != ESP_OK) {
        ESP_LOGE(kTag, "InitializeNvs failed: %s", esp_err_to_name(err));
        HoldForDevelopment("InitializeNvs failed");
    }

    err = InitializeFailureStateStorage();
    if (err != ESP_OK) {
        ESP_LOGE(kTag, "InitializeFailureStateStorage failed: %s", esp_err_to_name(err));
        HoldForDevelopment("InitializeFailureStateStorage failed");
    }

    i2c_master_Init();
    axp_i2c_prot_init();
    axp_cmd_init();
    epaper_port_init();
    if (_sdcard_init() == 0) {
        RecordFailureState(UpdateTrigger::kStartup, FailureCategory::kConfigError, "SD card initialization failed");
        ESP_LOGE(kTag, "SD card initialization failed; sleep/shutdown is disabled in the current development build");
        HoldForDevelopment("SD card initialization failed");
    }

    button_Init();

    err = InitializeDisplayPipeline();
    if (err != ESP_OK) {
        RecordFailureState(UpdateTrigger::kStartup, FailureCategory::kImageError, "Display pipeline initialization failed");
        ESP_LOGE(kTag, "InitializeDisplayPipeline failed: %s", esp_err_to_name(err));
        HoldForDevelopment("InitializeDisplayPipeline failed");
    }

    err = InitializeUpdateJobSystem();
    if (err != ESP_OK) {
        RecordFailureState(UpdateTrigger::kStartup, FailureCategory::kConfigError, "Update job system initialization failed");
        ESP_LOGE(kTag, "InitializeUpdateJobSystem failed: %s", esp_err_to_name(err));
        HoldForDevelopment("InitializeUpdateJobSystem failed");
    }

    err = EnqueueUpdate(UpdateTrigger::kStartup);
    if (err != ESP_OK) {
        RecordFailureState(UpdateTrigger::kStartup, FailureCategory::kConfigError, "Startup update queueing failed");
        ESP_LOGE(kTag, "EnqueueUpdate failed: %s", esp_err_to_name(err));
        HoldForDevelopment("EnqueueUpdate failed");
    }

    ESP_LOGI(kTag, "Startup update queued");
}
