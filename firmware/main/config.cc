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

bool CopyOptionalJsonString(cJSON* root, const char* key, char* destination, size_t destination_size, char* error_detail,
                            size_t error_detail_size) {
    cJSON* item = cJSON_GetObjectItemCaseSensitive(root, key);
    if (item == nullptr) {
        destination[0] = '\0';
        return true;
    }
    if (!cJSON_IsString(item) || item->valuestring == nullptr || item->valuestring[0] == '\0') {
        char message[112];
        snprintf(message, sizeof(message), "config.txt JSON invalid or empty optional string: %s", key);
        SetErrorDetail(error_detail, error_detail_size, message);
        return false;
    }

    snprintf(destination, destination_size, "%s", item->valuestring);
    return true;
}

bool CopyOptionalJsonBool(cJSON* root, const char* key, bool* destination, char* error_detail, size_t error_detail_size) {
    cJSON* item = cJSON_GetObjectItemCaseSensitive(root, key);
    if (item == nullptr) {
        *destination = false;
        return true;
    }
    if (!cJSON_IsBool(item)) {
        char message[104];
        snprintf(message, sizeof(message), "config.txt JSON invalid boolean: %s", key);
        SetErrorDetail(error_detail, error_detail_size, message);
        return false;
    }

    *destination = cJSON_IsTrue(item);
    return true;
}

bool IsHttpOrHttpsUrl(const char* value) {
    if (value == nullptr) {
        return false;
    }
    return strncmp(value, "http://", 7) == 0 || strncmp(value, "https://", 8) == 0;
}

bool HasBmpSuffix(const char* value) {
    if (value == nullptr) {
        return false;
    }
    size_t length = strlen(value);
    return length >= 4 && strcmp(value + length - 4, ".bmp") == 0;
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
                             error_detail_size) &&
              CopyOptionalJsonString(root, "bearer_token", out_config->bearer_token, sizeof(out_config->bearer_token),
                                     error_detail, error_detail_size) &&
              CopyOptionalJsonBool(root, "insecure", &out_config->insecure, error_detail, error_detail_size);

    if (ok) {
        if (!IsHttpOrHttpsUrl(out_config->image_url)) {
            ok = false;
            SetErrorDetail(error_detail, error_detail_size, "image_url must start with http:// or https://");
        }
    }

    cJSON_Delete(root);
    if (!ok) {
        return ESP_ERR_INVALID_ARG;
    }

    ESP_LOGI(kTag, "Loaded config.txt JSON for SSID '%s' (auth=%s, insecure=%s)", out_config->wifi_ssid,
             out_config->bearer_token[0] == '\0' ? "none" : "bearer", out_config->insecure ? "true" : "false");
    return ESP_OK;
}

bool IsBinaryImageUrl(const char* image_url) {
    // Default to binary format; only use BMP path when URL explicitly ends with .bmp
    return !HasBmpSuffix(image_url);
}
