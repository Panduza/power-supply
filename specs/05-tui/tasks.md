# 05 - Terminal User Interface (TUI) — Tasks

Primary language: English. All documents and generated code for this feature MUST be written in English unless a specific exception is documented.

Purpose: break the work into small, PR-friendly tasks mapped to the stories and tests.

1. [ ] Task 01 — Spec: Stories & Requirements
	- spec: `specs/05-tui/stories.md`, `specs/05-tui/requirements.md`
	- description: Review and complete the feature stories and requirements. Ensure each story has clear acceptance criteria and list required tests as specified by project rules.
	- acceptance: `stories.md` and `requirements.md` exist, include acceptance criteria for each user story, and list tests where required; ready for task generation.
	- tests: N/A (docs-only) — reviewers must verify presence of acceptance criteria.
	- estimate: 0.25 day
	- dependencies: none
	- prerequisites: none
	- blocking: true

2. [ ] Task 02 — CLI skeleton & parsing
	- spec: `specs/05-tui/requirements.md`
	- description: Add a `cli` module (or extend `main.rs`) that implements parsing for commands: `power`, `set voltage`, `get voltage`, `set current`, `get current`, `status`. Use `clap` or the project's standard parser. Provide `--help` and a global `--json` flag.
	- acceptance: CLI binary accepts commands and arguments; `--help` prints usage; unit tests cover parser for valid/invalid inputs.
	- tests: unit tests for parsing, validation of numeric formats and bounds.
	- estimate: 1 day
	- dependencies: Task 01
	- prerequisites: Task 01 completed
	- blocking: false

3. [ ] Task 03 — Implement `power on|off`
	- spec: `specs/05-tui/stories.md`, `specs/05-tui/requirements.md`
	- description: Implement the `power on` and `power off` commands by calling the device driver API. Ensure operations are serialized and log results.
	- acceptance: `power on` and `power off` toggle device state, print human-friendly output, and `--json` returns a structured object with state and timestamp. CLI returns non-zero on hardware error.
	- tests: integration tests using `drivers::emulator` to simulate device responses; unit tests to validate error handling.
	- estimate: 1 day
	- dependencies: Task 02
	- prerequisites: Task 02
	- blocking: false

4. [ ] Task 04 — Implement `set/get voltage`
	- spec: `specs/05-tui/stories.md`, `specs/05-tui/requirements.md`
	- description: Implement `set voltage <V>` and `get voltage`. Validate numeric format and safe bounds, call the driver to set/read voltage, and return measured/readback values.
	- acceptance: `set voltage 5.00` applies the setpoint and prints `Requested: 5.00 V / Measured: X.XX V` with timestamp; `--json` returns `requested`, `measured`, `unit`, `timestamp`.
	- tests: unit tests for validation + integration tests with `emulator` verifying readback; JSON schema test for `--json` output.
	- estimate: 1.5 days
	- dependencies: Task 02
	- prerequisites: Task 02
	- blocking: false

5. [ ] Task 05 — Implement `set/get current`
	- spec: `specs/05-tui/stories.md`, `specs/05-tui/requirements.md`
	- description: Implement `set current <A>` and `get current` analogous to voltage commands. Validate bounds and format specific to current.
	- acceptance: `set current 0.500` applies the limit and prints `Requested: 0.500 A` with timestamp; `get current` prints measured current; `--json` returns structured measurement.
	- tests: unit validation tests; integration tests with `emulator`.
	- estimate: 1 day
	- dependencies: Task 02
	- prerequisites: Task 02
	- blocking: false

6. [ ] Task 06 — Implement `status` summary
	- spec: `specs/05-tui/stories.md`, `specs/05-tui/requirements.md`
	- description: Implement `status` command that aggregates `DeviceState` (power, set/measured voltage/current) and prints a concise human-readable summary and a `--json` machine-readable schema.
	- acceptance: `status` prints power state, set/measured voltage/current with timestamps; `status --json` conforms to the agreed schema in `requirements.md`.
	- tests: integration test that calls `status --json` and validates schema stability and presence of timestamps.
	- estimate: 0.75 day
	- dependencies: Tasks 03, 04, 05
	- prerequisites: Tasks 03–05
	- blocking: false

7. [ ] Task 07 — Logging & Audit
	- spec: `specs/05-tui/requirements.md`
	- description: Add persistent audit logs for commands and device responses. Use existing logging infrastructure if present; otherwise add a simple rotating file logger. Ensure no secrets are written.
	- acceptance: each command invocation is recorded with timestamp, command arguments (without secrets), result, and optional driver response; logs rotate or are bounded.
	- tests: unit/integration test that verifies a log entry is created for `power on` and `set voltage` commands.
	- estimate: 0.5 day
	- dependencies: Tasks 02–06 (can be implemented in parallel but should capture events from those tasks)
	- prerequisites: none
	- blocking: false

8. [ ] Task 08 — Error handling and edge cases
	- spec: `specs/05-tui/requirements.md`
	- description: Implement robust error handling for unreachable devices, out-of-range setpoints, and hardware faults. Ensure CLI returns non-zero exit codes and user-friendly messages.
	- acceptance: documented behaviors for disconnected device, out-of-range values, and hardware faults; tests simulate emulator failures and validate messages & exit codes.
	- tests: integration tests simulating device disconnect and driver errors; unit tests for validation logic.
	- estimate: 1 day
	- dependencies: Tasks 03–06
	- prerequisites: Tasks 03–06
	- blocking: false

9. [ ] Task 09 — Docs & CI
	- spec: `docs/` and `specs/05-tui/requirements.md`
	- description: Add user-facing docs for each CLI command into `docs/`, and add CI steps to run CLI tests, `cargo fmt`, and `cargo clippy`.
	- acceptance: `docs/` contains entries for each command; CI workflow runs tests and linters successfully for PRs touching CLI code.
	- tests: CI must run unit and integration tests for the CLI; ensure linters pass.
	- estimate: 0.5 day
	- dependencies: Tasks 02–06
	- prerequisites: Tasks 02–06
	- blocking: false

10. [ ] Task 10 — Release notes / Migration
	- spec: `specs/05-tui/requirements.md`
	- description: Prepare release notes and migration steps if the CLI introduces breaking changes. Document version bump and migration instructions.
	- acceptance: release notes drafted and added to `docs/` or release artifacts; version bumped if changes are breaking.
	- tests: N/A (documentation)
	- estimate: 0.25 day
	- dependencies: Tasks 02–09
	- prerequisites: Tasks 02–09
	- blocking: false

Notes on PR strategy:
- Prefer separate PRs for Tasks 02–06 (small surface & tests included).
- Tasks 03–06 should provide `--json` from the first iteration to satisfy PR-008.

References:
- Origin spec: `specs/05-tui/stories.md` and `specs/05-tui/requirements.md`.
