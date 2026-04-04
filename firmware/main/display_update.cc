#include "display_update.h"

#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <sys/stat.h>

#include "GUI_BMPfile.h"
#include "GUI_Paint.h"
#include "epaper_port.h"
#include "esp_crt_bundle.h"
#include "esp_heap_caps.h"
#include "esp_http_client.h"
#include "esp_log.h"

namespace {

constexpr const char* kTag = "fw_display";
constexpr int kHttpTimeoutMs = 30000;
constexpr uint8_t kBinaryMagic[] = {'P', 'P', 'B', 'F'};
constexpr uint8_t kBinaryVersion = 1;
constexpr uint16_t kBinaryHeaderLength = 20;
constexpr size_t kBinaryPayloadLength = ((EXAMPLE_LCD_WIDTH % 2 == 0) ? (EXAMPLE_LCD_WIDTH / 2) : (EXAMPLE_LCD_WIDTH / 2 + 1)) *
                                        EXAMPLE_LCD_HEIGHT;

uint8_t* s_epaper_image = nullptr;
size_t s_http_bytes_written = 0;
FILE* s_http_output = nullptr;

enum class HttpTransferMode {
    kNone,
    kFile,
    kBinary,
};

struct BinaryTransferState {
    uint8_t header[kBinaryHeaderLength];
    size_t header_received;
    size_t payload_received;
    size_t expected_payload_length;
    uint32_t expected_checksum;
    uint32_t actual_checksum;
    bool header_validated;
};

HttpTransferMode s_http_transfer_mode = HttpTransferMode::kNone;
BinaryTransferState s_binary_transfer = {};

void SetErrorDetail(char* buffer, size_t buffer_size, const char* message) {
    if (buffer == nullptr || buffer_size == 0) {
        return;
    }
    snprintf(buffer, buffer_size, "%s", message);
}

bool IsHttpsUrl(const char* url) {
    return url != nullptr && strncmp(url, "https://", 8) == 0;
}

esp_err_t HttpEventHandler(esp_http_client_event_t* evt) {
    switch (evt->event_id) {
        case HTTP_EVENT_ON_DATA:
            if (evt->data_len <= 0) {
                break;
            }
            if (s_http_transfer_mode == HttpTransferMode::kFile) {
                if (s_http_output != nullptr) {
                    size_t written = fwrite(evt->data, 1, evt->data_len, s_http_output);
                    s_http_bytes_written += written;
                    if (written != static_cast<size_t>(evt->data_len)) {
                        return ESP_FAIL;
                    }
                }
            } else if (s_http_transfer_mode == HttpTransferMode::kBinary) {
                const uint8_t* cursor = static_cast<const uint8_t*>(evt->data);
                size_t remaining = static_cast<size_t>(evt->data_len);
                while (remaining > 0) {
                    if (!s_binary_transfer.header_validated) {
                        size_t needed = kBinaryHeaderLength - s_binary_transfer.header_received;
                        size_t chunk = remaining < needed ? remaining : needed;
                        memcpy(s_binary_transfer.header + s_binary_transfer.header_received, cursor, chunk);
                        s_binary_transfer.header_received += chunk;
                        cursor += chunk;
                        remaining -= chunk;
                        if (s_binary_transfer.header_received == kBinaryHeaderLength) {
                            uint16_t header_length =
                                static_cast<uint16_t>(s_binary_transfer.header[6]) |
                                (static_cast<uint16_t>(s_binary_transfer.header[7]) << 8);
                            uint16_t width = static_cast<uint16_t>(s_binary_transfer.header[8]) |
                                             (static_cast<uint16_t>(s_binary_transfer.header[9]) << 8);
                            uint16_t height = static_cast<uint16_t>(s_binary_transfer.header[10]) |
                                              (static_cast<uint16_t>(s_binary_transfer.header[11]) << 8);
                            uint32_t payload_length = static_cast<uint32_t>(s_binary_transfer.header[12]) |
                                                      (static_cast<uint32_t>(s_binary_transfer.header[13]) << 8) |
                                                      (static_cast<uint32_t>(s_binary_transfer.header[14]) << 16) |
                                                      (static_cast<uint32_t>(s_binary_transfer.header[15]) << 24);
                            uint32_t checksum = static_cast<uint32_t>(s_binary_transfer.header[16]) |
                                                (static_cast<uint32_t>(s_binary_transfer.header[17]) << 8) |
                                                (static_cast<uint32_t>(s_binary_transfer.header[18]) << 16) |
                                                (static_cast<uint32_t>(s_binary_transfer.header[19]) << 24);
                            if (memcmp(s_binary_transfer.header, kBinaryMagic, sizeof(kBinaryMagic)) != 0 ||
                                s_binary_transfer.header[4] != kBinaryVersion || s_binary_transfer.header[5] != 0 ||
                                header_length != kBinaryHeaderLength || width != EXAMPLE_LCD_WIDTH ||
                                height != EXAMPLE_LCD_HEIGHT || payload_length != kBinaryPayloadLength) {
                                return ESP_ERR_INVALID_RESPONSE;
                            }
                            s_binary_transfer.expected_payload_length = payload_length;
                            s_binary_transfer.expected_checksum = checksum;
                            s_binary_transfer.header_validated = true;
                        }
                        continue;
                    }

                    size_t capacity =
                        s_binary_transfer.expected_payload_length - s_binary_transfer.payload_received;
                    size_t chunk = remaining < capacity ? remaining : capacity;
                    if (chunk == 0) {
                        return ESP_ERR_INVALID_SIZE;
                    }
                    memcpy(s_epaper_image + s_binary_transfer.payload_received, cursor, chunk);
                    for (size_t i = 0; i < chunk; ++i) {
                        s_binary_transfer.actual_checksum += cursor[i];
                    }
                    s_binary_transfer.payload_received += chunk;
                    cursor += chunk;
                    remaining -= chunk;
                }
                s_http_bytes_written = s_binary_transfer.payload_received;
            }
            break;
        default:
            break;
    }
    return ESP_OK;
}

void ResetBinaryTransferState() {
    memset(&s_binary_transfer, 0, sizeof(s_binary_transfer));
}

esp_err_t ValidateBinaryTransfer(char* error_detail, size_t error_detail_size) {
    if (!s_binary_transfer.header_validated) {
        SetErrorDetail(error_detail, error_detail_size, "binary frame header is incomplete");
        return ESP_ERR_INVALID_RESPONSE;
    }
    if (s_binary_transfer.payload_received != s_binary_transfer.expected_payload_length) {
        SetErrorDetail(error_detail, error_detail_size, "binary frame payload length is incomplete");
        return ESP_ERR_INVALID_SIZE;
    }
    if (s_binary_transfer.actual_checksum != s_binary_transfer.expected_checksum) {
        SetErrorDetail(error_detail, error_detail_size, "binary frame checksum mismatch");
        return ESP_ERR_INVALID_CRC;
    }
    return ESP_OK;
}

esp_err_t PerformHttpGet(const char* url, const char* bearer_token, bool insecure, char* error_detail,
                        size_t error_detail_size) {
    esp_http_client_config_t config = {};
    config.url = url;
    config.timeout_ms = kHttpTimeoutMs;
    config.event_handler = HttpEventHandler;
    config.buffer_size = 4096;
    config.disable_auto_redirect = false;
    if (IsHttpsUrl(url) && !insecure) {
        config.crt_bundle_attach = esp_crt_bundle_attach;
    }

    esp_http_client_handle_t client = esp_http_client_init(&config);
    if (client == nullptr) {
        SetErrorDetail(error_detail, error_detail_size, "failed to initialize HTTP client");
        return ESP_FAIL;
    }

    esp_http_client_set_header(client, "Connection", "close");
    if (bearer_token != nullptr && bearer_token[0] != '\0') {
        char auth_header[540];
        snprintf(auth_header, sizeof(auth_header), "Bearer %s", bearer_token);
        esp_http_client_set_header(client, "Authorization", auth_header);
    }
    esp_err_t err = esp_http_client_perform(client);
    int status_code = esp_http_client_get_status_code(client);
    esp_http_client_cleanup(client);

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

    return ESP_OK;
}

esp_err_t ValidateDisplayPath(char* error_detail, size_t error_detail_size) {
    if (s_epaper_image == nullptr) {
        esp_err_t init_err = InitializeDisplayPipeline();
        if (init_err != ESP_OK) {
            SetErrorDetail(error_detail, error_detail_size, "display pipeline initialization failed");
            return init_err;
        }
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

esp_err_t DownloadImageToSdCard(const char* url, const char* bearer_token, bool insecure, const char* output_path,
                                char* error_detail, size_t error_detail_size) {
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
    s_http_transfer_mode = HttpTransferMode::kFile;
    esp_err_t err = PerformHttpGet(url, bearer_token, insecure, error_detail, error_detail_size);

    fclose(s_http_output);
    s_http_output = nullptr;
    s_http_transfer_mode = HttpTransferMode::kNone;

    if (err != ESP_OK) {
        return err;
    }

    ESP_LOGI(kTag, "Downloaded %u bytes to %s", static_cast<unsigned>(s_http_bytes_written), output_path);
    return ESP_OK;
}

esp_err_t DownloadBinaryFrameToDisplay(const char* url, const char* bearer_token, bool insecure, char* error_detail,
                                       size_t error_detail_size) {
    if (url == nullptr) {
        SetErrorDetail(error_detail, error_detail_size, "invalid binary download arguments");
        return ESP_ERR_INVALID_ARG;
    }

    esp_err_t init_err = ValidateDisplayPath(error_detail, error_detail_size);
    if (init_err != ESP_OK) {
        return init_err;
    }

    Paint_SelectImage(s_epaper_image);
    Paint_Clear(EPD_7IN3E_WHITE);
    s_http_bytes_written = 0;
    s_http_transfer_mode = HttpTransferMode::kBinary;
    ResetBinaryTransferState();

    esp_err_t err = PerformHttpGet(url, bearer_token, insecure, error_detail, error_detail_size);
    s_http_transfer_mode = HttpTransferMode::kNone;
    if (err != ESP_OK) {
        return err;
    }

    err = ValidateBinaryTransfer(error_detail, error_detail_size);
    if (err != ESP_OK) {
        return err;
    }

    ESP_LOGI(kTag, "Displaying binary frame (%u bytes)", static_cast<unsigned>(s_binary_transfer.payload_received));
    epaper_port_display(s_epaper_image);
    return ESP_OK;
}

esp_err_t RenderBmpFromSdCard(const char* path, char* error_detail, size_t error_detail_size) {
    esp_err_t init_err = ValidateDisplayPath(error_detail, error_detail_size);
    if (init_err != ESP_OK) {
        return init_err;
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
