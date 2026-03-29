#pragma once

#include "esp_err.h"

#include "failure_state.h"

esp_err_t InitializeUpdateJobSystem();
esp_err_t InitializeActivityLedControl();
esp_err_t EnqueueUpdate(UpdateTrigger trigger);
void StopActivityLed();
