#include "failure_state.h"

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

#include "esp_log.h"
#include "nvs.h"

namespace {

constexpr const char* kTag = "fw_failure";
constexpr const char* kNamespace = "firmware";
constexpr const char* kFailureReportPath = "/sdcard/last_error.txt";

void SaveString(nvs_handle_t handle, const char* key, const char* value) {
    if (value == nullptr) {
        value = "";
    }
    ESP_ERROR_CHECK_WITHOUT_ABORT(nvs_set_str(handle, key, value));
}

void WriteSdCardFailureReport(UpdateTrigger trigger, FailureCategory category, const char* detail) {
    FILE* fp = fopen(kFailureReportPath, "wb");
    if (fp == nullptr) {
        ESP_LOGW(kTag, "Failed to open %s for writing", kFailureReportPath);
        return;
    }

    const char* safe_detail = detail == nullptr ? "" : detail;
    fprintf(fp, "last_failure=%s\n", FailureCategoryToString(category));
    fprintf(fp, "last_trigger=%s\n", UpdateTriggerToString(trigger));
    fprintf(fp, "last_detail=%s\n", safe_detail);
    fclose(fp);

    ESP_LOGI(kTag, "Wrote failure report to %s", kFailureReportPath);
}

}  // namespace

esp_err_t InitializeFailureStateStorage() {
    nvs_handle_t handle = 0;
    esp_err_t err = nvs_open(kNamespace, NVS_READWRITE, &handle);
    if (err != ESP_OK) {
        return err;
    }
    nvs_close(handle);
    return ESP_OK;
}

void ClearLastFailureState() {
    nvs_handle_t handle = 0;
    if (nvs_open(kNamespace, NVS_READWRITE, &handle) != ESP_OK) {
        return;
    }

    SaveString(handle, "last_failure", "none");
    SaveString(handle, "last_trigger", "none");
    SaveString(handle, "last_detail", "");
    ESP_ERROR_CHECK_WITHOUT_ABORT(nvs_commit(handle));
    nvs_close(handle);

    ClearFailureReportFromSdCard();
}

void RecordFailureState(UpdateTrigger trigger, FailureCategory category, const char* detail) {
    ESP_LOGE(kTag, "Failure recorded: trigger=%s category=%s detail=%s", UpdateTriggerToString(trigger),
             FailureCategoryToString(category), detail == nullptr ? "" : detail);

    nvs_handle_t handle = 0;
    if (nvs_open(kNamespace, NVS_READWRITE, &handle) != ESP_OK) {
        return;
    }

    SaveString(handle, "last_failure", FailureCategoryToString(category));
    SaveString(handle, "last_trigger", UpdateTriggerToString(trigger));
    SaveString(handle, "last_detail", detail == nullptr ? "" : detail);
    ESP_ERROR_CHECK_WITHOUT_ABORT(nvs_commit(handle));
    nvs_close(handle);

    WriteFailureReportToSdCard(trigger, category, detail);
}

void WriteFailureReportToSdCard(UpdateTrigger trigger, FailureCategory category, const char* detail) {
    WriteSdCardFailureReport(trigger, category, detail);
}

void ClearFailureReportFromSdCard() {
    struct stat st = {};
    if (stat(kFailureReportPath, &st) != 0) {
        return;
    }

    if (remove(kFailureReportPath) != 0) {
        ESP_LOGW(kTag, "Failed to remove %s", kFailureReportPath);
    }
}

const char* UpdateTriggerToString(UpdateTrigger trigger) {
    switch (trigger) {
        case UpdateTrigger::kStartup:
            return "startup";
        case UpdateTrigger::kBootButton:
            return "boot_button";
    }
    return "unknown";
}

const char* FailureCategoryToString(FailureCategory category) {
    switch (category) {
        case FailureCategory::kNone:
            return "none";
        case FailureCategory::kConfigError:
            return "config_error";
        case FailureCategory::kWifiError:
            return "wifi_error";
        case FailureCategory::kHttpError:
            return "http_error";
        case FailureCategory::kImageError:
            return "image_error";
    }
    return "unknown";
}
