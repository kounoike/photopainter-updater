#pragma once

#include <stddef.h>

#include "esp_err.h"

struct FirmwareConfig {
    char wifi_ssid[33];
    char wifi_password[65];
    char image_url[257];
};

esp_err_t LoadConfigFromSdCard(const char* path, FirmwareConfig* out_config, char* error_detail, size_t error_detail_size);
bool IsBinaryImageUrl(const char* image_url);
