# 05 - Terminal User Interface (TUI) — Requirements

Primary language: English. All documents and generated code for this feature MUST be written in English unless a specific exception is documented.

## Functional Requirements

- FR-001: Provide a CLI TUI exposing the core controls: `power on|off`, `set voltage <V>`, `get voltage`, `set current <A>`, `get current`, `status`.
- FR-002: Validate numeric formats and bounds before calling device APIs.
- FR-003: Support machine-readable output via the `--json` option for all measurement/state commands.
- FR-004: Persist user-friendly command logs for auditing (text format) without exposing sensitive information.
- FR-005: Surface clear error codes and messages for hardware communication failures.
- FR-006: Include an ISO8601 timestamp (`last_updated`) for measurements and state changes.

## Platform / Cross-cutting

- FR-PLATFORMS: Support Linux, macOS, and Windows. Document any platform-specific limitations.
- FR-CLI: All user-facing functionality exposed by the TUI must be available via the CLI and listed in `--help`.
- FR-MCP: If exposing automation/integrations, provide an MCP-compatible contract and contract tests (specify if required).
- FR-TEST-FIRST: Tests must be specified here and implemented before implementation (unit/integration CLI tests).

## Tests to Implement (required by PR-002)
- `power_on_off`: run `power on` then `power off` and validate `status --json`.
- `set_voltage_bounds`: verify invalid values are rejected and valid values applied.
- `get_voltage_json_schema`: ensure `get voltage --json` conforms to the schema.
- `status_json_schema`: validate the stability of the JSON format returned by `status --json`.

## Project Rules (reminder - must follow)
- PR-001 (Docs First): Every command must have an entry in `docs/` and appear in `--help`.
- PR-002 (Test Coverage): Each story must have at least one automated CLI test.
- PR-004 (Error Handling): No unhandled panics; user-friendly errors + non-zero exit codes.
- PR-007 (Formatting & Linting): `cargo fmt` and `cargo clippy` must pass.
- PR-008 (CLI Machine-Readable Output): Measurement commands must offer `--json`.

## Key Entities
- `DeviceState`: `{ power: "on"|"off", voltage_set: f64, current_set: f64, voltage_measured: f64, current_measured: f64, last_updated: iso8601 }`.
- `Command`: Representation of a parsed CLI command, its arguments, and validation result.
- `Measurement`: `{ value: f64, unit: "V"|"A", timestamp: iso8601 }`.

## Success Criteria (measurable)
- SC-001: Users can toggle power and view updated state via CLI with < 1 second response for local devices.
- SC-002: `set voltage` and `set current` return measured values and timestamps in > 95% of integration runs.
- SC-003: `status --json` output conforms to a stable schema usable by automation scripts.

## Implementation Notes
- Exact device bounds (min/max voltage/current) should be read from the driver configuration (`drivers::...`) or documented here if hardware imposes limits.
- Audit logs may be managed by the `server::services` component or equivalent; avoid writing secrets to logs.
