#include "config.h"

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

#include "cJSON.h"
#include "esp_log.h"

namespace {

constexpr const char* kTag = "fw_config";

void SetErrorDetail(char* buffer, size_t buffer_size, const char* message) {
    if (buffer == nullptr || buffer_size == 0) {
        return;
    }
    snprintf(buffer, buffer_size, "%s", message);
}

bool CopyJsonString(cJSON* root, const char* key, char* destination, size_t destination_size, char* error_detail,
                    size_t error_detail_size) {
    cJSON* item = cJSON_GetObjectItemCaseSensitive(root, key);
    if (!cJSON_IsString(item) || item->valuestring == nullptr || item->valuestring[0] == '\0') {
        char message[96];
        snprintf(message, sizeof(message), "config.txt JSON missing or empty: %s", key);
        SetErrorDetail(error_detail, error_detail_size, message);
        return false;
    }

    snprintf(destination, destination_size, "%s", item->valuestring);
    return true;
}
}  // namespace

esp_err_t LoadConfigFromSdCard(const char* path, FirmwareConfig* out_config, char* error_detail, size_t error_detail_size) {
    if (path == nullptr || out_config == nullptr) {
        SetErrorDetail(error_detail, error_detail_size, "invalid config arguments");
        return ESP_ERR_INVALID_ARG;
    }

    struct stat st = {};
    if (stat(path, &st) != 0) {
        SetErrorDetail(error_detail, error_detail_size, "config.txt not found on SD card root");
        return ESP_ERR_NOT_FOUND;
    }

    FILE* fp = fopen(path, "rb");
    if (fp == nullptr) {
        SetErrorDetail(error_detail, error_detail_size, "failed to open config.txt");
        return ESP_FAIL;
    }

    char* json_buffer = new char[st.st_size + 1];
    size_t read_size = fread(json_buffer, 1, st.st_size, fp);
    fclose(fp);
    json_buffer[read_size] = '\0';

    if (read_size == 0) {
        delete[] json_buffer;
        SetErrorDetail(error_detail, error_detail_size, "config.txt is empty");
        return ESP_ERR_INVALID_SIZE;
    }

    cJSON* root = cJSON_Parse(json_buffer);
    delete[] json_buffer;
    if (root == nullptr) {
        SetErrorDetail(error_detail, error_detail_size, "config.txt JSON parse failed");
        return ESP_ERR_INVALID_RESPONSE;
    }

    memset(out_config, 0, sizeof(*out_config));
    bool ok = CopyJsonString(root, "wifi_ssid", out_config->wifi_ssid, sizeof(out_config->wifi_ssid), error_detail,
                             error_detail_size) &&
              CopyJsonString(root, "wifi_password", out_config->wifi_password, sizeof(out_config->wifi_password),
                             error_detail, error_detail_size) &&
              CopyJsonString(root, "image_url", out_config->image_url, sizeof(out_config->image_url), error_detail,
                             error_detail_size);

    if (ok) {
        if (strncmp(out_config->image_url, "http://", 7) != 0) {
            ok = false;
            SetErrorDetail(error_detail, error_detail_size, "image_url must start with http://");
        }
    }

    cJSON_Delete(root);
    if (!ok) {
        return ESP_ERR_INVALID_ARG;
    }

    ESP_LOGI(kTag, "Loaded config.txt JSON for SSID '%s'", out_config->wifi_ssid);
    return ESP_OK;
}
