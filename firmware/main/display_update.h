#pragma once

#include <stddef.h>

#include "esp_err.h"

esp_err_t InitializeDisplayPipeline();
esp_err_t DownloadImageToSdCard(const char* url, const char* output_path, char* error_detail, size_t error_detail_size);
esp_err_t DownloadBinaryFrameToDisplay(const char* url, char* error_detail, size_t error_detail_size);
esp_err_t RenderBmpFromSdCard(const char* path, char* error_detail, size_t error_detail_size);
