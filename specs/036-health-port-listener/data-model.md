# Data Model: Health Port Listener

## 1. PortConfiguration

- Purpose:
  - server 起動時の port 設定を表す。
- Fields:
  - `port`
  - `health_port`
- Validation:
  - `port` は必須
  - `health_port` は任意

## 2. HealthListenerMode

- Purpose:
  - health listener の起動形態を表す。
- Variants:
  - `Disabled`
  - `SharedWithMain`
  - `Dedicated`
- Rules:
  - `health_port` 未指定なら `Disabled`
  - `health_port == port` なら `SharedWithMain`
  - `health_port != port` なら `Dedicated`

## 3. ListenerBindingPlan

- Purpose:
  - 実際に bind する listener 一覧を表す。
- Fields:
  - `main_address`
  - `health_address`
  - `health_mode`
- Validation:
  - `Dedicated` のときだけ `health_address` を持つ
