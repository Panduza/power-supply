# Feature Specification: Terminal User Interface (TUI)

**Feature Branch**: `63-move-to-ratatui-to-ease-diffusion-through-cargo-tools`  
**Created**: 2025-11-20  
**Status**: Draft  
**Input**: User description: "This server must propose a Terminal UI"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Power Control (Priority: P1)

As a user, I want to switch the device power ON and OFF from the terminal so I can control the supply without a GUI.

**Why this priority**: Basic operational control is the primary use-case for a headless interface.

**Independent Test**: Invoking the `power on` and `power off` CLI commands toggles the device state and returns a confirmation.

**Acceptance Scenarios**:

1. **Given** the device is OFF, **When** the user issues `power on`, **Then** the device transitions to ON and the CLI prints the new state and timestamp.
2. **Given** the device is ON, **When** the user issues `power off`, **Then** the device transitions to OFF and the CLI prints the new state and timestamp.

---

### User Story 2 - Set Output Voltage (Priority: P2)

As a user, I want to set the output voltage so I can configure the device to the required voltage.

**Why this priority**: Essential for configuring the supply but secondary to basic power control.

**Independent Test**: Issuing `set voltage 5.00` stores the requested setpoint and triggers the hardware/API call; the CLI returns requested and measured values.

**Acceptance Scenarios**:

1. **Given** the device is ON, **When** the user issues `set voltage 5.00`, **Then** the device accepts the setpoint and the CLI displays "Requested: 5.00 V / Measured: X.XX V" with timestamp.

---

### User Story 3 - Read Output Voltage (Priority: P2)

As a user, I want to read the current output voltage to verify the applied setpoint.

**Why this priority**: Verification step needed for safe operation and debugging.

**Independent Test**: Issuing `get voltage` returns a measured voltage and timestamp.

**Acceptance Scenarios**:

1. **Given** the device is connected, **When** the user issues `get voltage`, **Then** the CLI prints the current measured voltage and timestamp.

---

### User Story 4 - Set Current Limit (Priority: P2)

As a user, I want to set the current limit so I can protect the load and configure device behavior.

**Why this priority**: Protecting loads and preventing damage is critical but aligned with voltage control.

**Independent Test**: Issuing `set current 0.500` stores the limit and the CLI displays the applied setpoint.

**Acceptance Scenarios**:

1. **Given** the device is ON, **When** the user issues `set current 0.500`, **Then** the device accepts the limit and the CLI displays "Requested: 0.500 A" with timestamp.

---

### User Story 5 - Read Output Current (Priority: P2)

As a user, I want to read the output current to monitor consumption.

**Why this priority**: Observability of current is needed for safety and measurement.

**Independent Test**: Issuing `get current` returns the current measurement and timestamp.

**Acceptance Scenarios**:

1. **Given** the device is connected, **When** the user issues `get current`, **Then** the CLI prints the measured current and timestamp.

---

### User Story 6 - Summary Status (Priority: P1)

As a user, I want a concise summary of power, set/measured voltage and set/measured current to get a quick status overview.

**Why this priority**: Rapid situational awareness is critical during experiments or troubleshooting.

**Independent Test**: Issuing `status` prints power state, voltage setpoint/measured, current setpoint/measured, and timestamps.

**Acceptance Scenarios**:

1. **Given** the device is connected, **When** the user issues `status`, **Then** the CLI prints: Power (ON/OFF), Voltage (set/measured), Current (set/measured) with timestamps.

---

### Edge Cases

- What happens when the device is disconnected or unreachable? The CLI should return an error code and a clear message.
- How does system handle values outside device limits? The CLI must validate inputs and reject out-of-range setpoints with a helpful message.
- Concurrent commands: ensure serialized access to the device API to avoid race conditions.
- Hardware faults: present readable error codes and fall back to a safe state (e.g., power off) when required.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a CLI-based Terminal UI exposing the core device controls (power on/off, set/get voltage, set/get current, status).
- **FR-002**: CLI input MUST validate numeric formats and bounds (voltage and current) before calling device APIs.
- **FR-003**: CLI MUST return structured, machine-readable output when requested (e.g., `--json`) for automation.
- **FR-004**: System MUST persist user-friendly logs of commands and device responses for auditing.
- **FR-005**: System MUST surface clear error codes and messages for hardware communication failures.
- **FR-006**: CLI commands MUST include timestamps for measurements and state changes.

<!-- Project Rules -->

### Project Rules

- **PR-001 (Docs First)**: All CLI commands implemented MUST have a documentation entry in `docs/` and an entry in the CLI `--help` output.
- **PR-002 (Test Coverage)**: Each user story MUST have at least one automated test (unit or integration) validating the public CLI behavior (`--json` output where applicable).
- **PR-003 (Backward Compatibility)**: CLI argument changes MUST be backwards-compatible; when incompatible changes are necessary, bump major version and document migration steps.
- **PR-004 (Error Handling)**: No unhandled panics; all errors must be returned with user-friendly messages and non-zero exit codes.
- **PR-005 (Small, Reviewable PRs)**: Implement features in small PRs that each map to one or two user stories and include tests.
- **PR-006 (Security)**: Do not expose sensitive data (e.g., network credentials) in logs or CLI outputs.
- **PR-007 (Formatting & Linting)**: Code MUST pass repository linters/formatters before merge (run `cargo fmt` and `cargo clippy` as part of CI).
- **PR-008 (CLI Machine-Readable Output)**: All commands that return measurements MUST offer a `--json` flag for scripting.

### Platform / Cross-cutting Requirements

- **FR-PLATFORMS**: The feature MUST support Linux, macOS, and Windows (document any platform-specific limitations).
- **FR-CLI**: User-facing functionality MUST be available via CLI with machine-readable output (e.g., `--json`).
- **FR-MCP**: If the feature exposes automation or integrations, it MUST provide an MCP-compatible API contract and contract tests.
- **FR-TEST-FIRST**: Tests (unit/integration/contract) MUST be specified in the spec and written before implementation begins.

### Key Entities

- **DeviceState**: Represents power state (`on`/`off`), voltage setpoint, current setpoint, measured voltage, measured current, last_updated timestamp.
- **Command**: Represents a parsed CLI command with arguments, validation result, and execution metadata.
- **Measurement**: Represents a single sensor reading with value, unit, and timestamp.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can toggle power and view updated state via CLI with < 1 second response for local devices.
- **SC-002**: `set voltage` and `set current` commands return measured values and timestamps in > 95% of integration runs.
- **SC-003**: `status --json` output conforms to a stable schema and is usable by automated scripts.
- **SC-004**: Reduce reported CLI-related support issues by 50% after delivering clear docs and help output.

---

If any requirement above is unclear, mark it as [NEEDS CLARIFICATION] and propose options in the implementation ticket.


