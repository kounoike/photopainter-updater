#include "display_update.h"

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

#include "GUI_BMPfile.h"
#include "GUI_Paint.h"
#include "epaper_port.h"
#include "esp_heap_caps.h"
#include "esp_http_client.h"
#include "esp_log.h"

namespace {

constexpr const char* kTag = "fw_display";
constexpr int kHttpTimeoutMs = 30000;

uint8_t* s_epaper_image = nullptr;
size_t s_http_bytes_written = 0;
FILE* s_http_output = nullptr;

void SetErrorDetail(char* buffer, size_t buffer_size, const char* message) {
    if (buffer == nullptr || buffer_size == 0) {
        return;
    }
    snprintf(buffer, buffer_size, "%s", message);
}

esp_err_t HttpEventHandler(esp_http_client_event_t* evt) {
    switch (evt->event_id) {
        case HTTP_EVENT_ON_DATA:
            if (evt->data_len > 0 && s_http_output != nullptr) {
                size_t written = fwrite(evt->data, 1, evt->data_len, s_http_output);
                s_http_bytes_written += written;
                if (written != static_cast<size_t>(evt->data_len)) {
                    return ESP_FAIL;
                }
            }
            break;
        default:
            break;
    }
    return ESP_OK;
}

esp_err_t ValidateBmpHeader(const char* path, char* error_detail, size_t error_detail_size) {
    FILE* fp = fopen(path, "rb");
    if (fp == nullptr) {
        SetErrorDetail(error_detail, error_detail_size, "downloaded BMP could not be reopened");
        return ESP_FAIL;
    }

    BMPFILEHEADER file_header = {};
    BMPINFOHEADER info_header = {};
    bool header_ok = fread(&file_header, sizeof(file_header), 1, fp) == 1 &&
                     fread(&info_header, sizeof(info_header), 1, fp) == 1;
    fclose(fp);

    if (!header_ok) {
        SetErrorDetail(error_detail, error_detail_size, "downloaded image header is incomplete");
        return ESP_ERR_INVALID_RESPONSE;
    }
    if (file_header.bType != 0x4D42) {
        SetErrorDetail(error_detail, error_detail_size, "downloaded file is not a BMP image");
        return ESP_ERR_INVALID_RESPONSE;
    }
    if (info_header.biBitCount != 24) {
        SetErrorDetail(error_detail, error_detail_size, "BMP must be 24-bit for GUI_ReadBmp_RGB_6Color");
        return ESP_ERR_INVALID_ARG;
    }
    if (info_header.biWidth <= 0 || info_header.biHeight <= 0 ||
        info_header.biWidth > EXAMPLE_LCD_WIDTH || info_header.biHeight > EXAMPLE_LCD_HEIGHT) {
        SetErrorDetail(error_detail, error_detail_size, "BMP size is outside e-paper display bounds");
        return ESP_ERR_INVALID_SIZE;
    }
    return ESP_OK;
}

}  // namespace

esp_err_t InitializeDisplayPipeline() {
    if (s_epaper_image != nullptr) {
        return ESP_OK;
    }

    size_t image_size = ((EXAMPLE_LCD_WIDTH % 2 == 0) ? (EXAMPLE_LCD_WIDTH / 2) : (EXAMPLE_LCD_WIDTH / 2 + 1)) *
                        EXAMPLE_LCD_HEIGHT;
    s_epaper_image = static_cast<uint8_t*>(heap_caps_malloc(image_size, MALLOC_CAP_SPIRAM));
    if (s_epaper_image == nullptr) {
        return ESP_ERR_NO_MEM;
    }

    Paint_NewImage(s_epaper_image, EXAMPLE_LCD_WIDTH, EXAMPLE_LCD_HEIGHT, 0, EPD_7IN3E_WHITE);
    Paint_SetScale(6);
    Paint_SelectImage(s_epaper_image);
    Paint_SetRotate(180);
    Paint_Clear(EPD_7IN3E_WHITE);
    return ESP_OK;
}

esp_err_t DownloadImageToSdCard(const char* url, const char* output_path, char* error_detail, size_t error_detail_size) {
    if (url == nullptr || output_path == nullptr) {
        SetErrorDetail(error_detail, error_detail_size, "invalid download arguments");
        return ESP_ERR_INVALID_ARG;
    }

    s_http_output = fopen(output_path, "wb");
    if (s_http_output == nullptr) {
        SetErrorDetail(error_detail, error_detail_size, "failed to open output BMP on SD card");
        return ESP_FAIL;
    }

    s_http_bytes_written = 0;
    esp_http_client_config_t config = {};
    config.url = url;
    config.timeout_ms = kHttpTimeoutMs;
    config.event_handler = HttpEventHandler;
    config.buffer_size = 4096;
    config.disable_auto_redirect = false;

    esp_http_client_handle_t client = esp_http_client_init(&config);
    esp_err_t err = esp_http_client_perform(client);
    int status_code = esp_http_client_get_status_code(client);
    esp_http_client_cleanup(client);

    fclose(s_http_output);
    s_http_output = nullptr;

    if (err != ESP_OK) {
        char message[128];
        snprintf(message, sizeof(message), "HTTP transfer failed: %s", esp_err_to_name(err));
        SetErrorDetail(error_detail, error_detail_size, message);
        return err;
    }
    if (status_code < 200 || status_code >= 300) {
        char message[96];
        snprintf(message, sizeof(message), "HTTP status code %d", status_code);
        SetErrorDetail(error_detail, error_detail_size, message);
        return ESP_ERR_INVALID_RESPONSE;
    }
    if (s_http_bytes_written == 0) {
        SetErrorDetail(error_detail, error_detail_size, "HTTP response body is empty");
        return ESP_ERR_INVALID_SIZE;
    }

    ESP_LOGI(kTag, "Downloaded %u bytes to %s", static_cast<unsigned>(s_http_bytes_written), output_path);
    return ESP_OK;
}

esp_err_t RenderBmpFromSdCard(const char* path, char* error_detail, size_t error_detail_size) {
    if (s_epaper_image == nullptr) {
        esp_err_t init_err = InitializeDisplayPipeline();
        if (init_err != ESP_OK) {
            SetErrorDetail(error_detail, error_detail_size, "display pipeline initialization failed");
            return init_err;
        }
    }

    struct stat st = {};
    if (stat(path, &st) != 0) {
        SetErrorDetail(error_detail, error_detail_size, "downloaded BMP does not exist");
        return ESP_ERR_NOT_FOUND;
    }

    esp_err_t validate_err = ValidateBmpHeader(path, error_detail, error_detail_size);
    if (validate_err != ESP_OK) {
        return validate_err;
    }

    Paint_SelectImage(s_epaper_image);
    Paint_Clear(EPD_7IN3E_WHITE);
    if (GUI_ReadBmp_RGB_6Color(path, 0, 0) != 0) {
        SetErrorDetail(error_detail, error_detail_size, "GUI_ReadBmp_RGB_6Color rejected the BMP");
        return ESP_ERR_INVALID_RESPONSE;
    }

    ESP_LOGI(kTag, "Displaying BMP frame");
    epaper_port_display(s_epaper_image);
    return ESP_OK;
}
