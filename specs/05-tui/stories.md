# 05 - Terminal User Interface (TUI) — Stories

Primary language: English. All documents and generated code for this feature MUST be written in English unless a specific exception is documented.

## Context
The TUI feature provides command-line control of the power supply while preserving auditability, machine-readable output, and input validation. The stories below capture the essential user scenarios.

---

## User Story 1 — Power Control (P1)
As a user, I want to turn the device power ON and OFF from the terminal so I can control the supply without a GUI.

Acceptance Criteria:
- `power on` transitions the device to ON, prints the new state and an ISO8601 timestamp.
- `power off` transitions the device to OFF, prints the new state and an ISO8601 timestamp.
- Commands return a non-zero exit code when a hardware error occurs.

Independent Test:
- Run `power on` then `status --json` and verify the power state and timestamp.

---

## User Story 2 — Set Output Voltage (P2)
As a user, I want to set the output voltage so I can configure the device to the required voltage.

Acceptance Criteria:
- `set voltage 5.00` validates the input (format and bounds), applies the setpoint and prints: `Requested: 5.00 V / Measured: X.XX V` with timestamp.
- `--json` returns a structured JSON object containing `requested`, `measured`, `unit`, and `timestamp`.

Independent Test:
- Run `set voltage 5.00 --json` and validate the JSON schema.

---

## User Story 3 — Read Output Voltage (P2)
As a user, I want to read the current output voltage to verify the applied setpoint.

Acceptance Criteria:
- `get voltage` prints the measured voltage and timestamp.
- `get voltage --json` returns a JSON object compatible with `set voltage --json`.

---

## User Story 4 — Set Current Limit (P2)
As a user, I want to set the current limit to protect the load and configure device behavior.

Acceptance Criteria:
- `set current 0.500` validates and applies the limit, prints `Requested: 0.500 A` with timestamp.
- `--json` returns the applied setpoint and timestamp.

---

## User Story 5 — Read Output Current (P2)
As a user, I want to read the output current to monitor consumption.

Acceptance Criteria:
- `get current` prints the measured current and timestamp.
- `get current --json` returns a structured JSON object.

---

## User Story 6 — Summary Status (P1)
As a user, I want a concise summary of power, set/measured voltage and set/measured current to get a quick status overview.

Acceptance Criteria:
- `status` prints: Power (ON/OFF), Voltage (set/measured), Current (set/measured) with timestamps.
- `status --json` returns a stable schema usable by automation scripts.

---

## Edge Cases
- Device disconnected: commands must fail cleanly with a readable error message and non-zero exit code.
- Values outside device limits: CLI must validate and reject out-of-range setpoints with a helpful message.
- Concurrent commands: device API access must be serialized to avoid race conditions.
- Hardware faults: present readable error codes and fall back to a safe state (e.g., power off) when required.
