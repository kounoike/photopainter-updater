#pragma once

#include "esp_err.h"

enum class UpdateTrigger {
    kStartup,
    kBootButton,
};

enum class FailureCategory {
    kNone,
    kConfigError,
    kWifiError,
    kHttpError,
    kImageError,
};

esp_err_t InitializeFailureStateStorage();
void ClearLastFailureState();
void RecordFailureState(UpdateTrigger trigger, FailureCategory category, const char* detail);
void WriteFailureReportToSdCard(UpdateTrigger trigger, FailureCategory category, const char* detail);
void ClearFailureReportFromSdCard();
const char* UpdateTriggerToString(UpdateTrigger trigger);
const char* FailureCategoryToString(FailureCategory category);
