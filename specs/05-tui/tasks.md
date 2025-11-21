# 05 - Terminal User Interface (TUI) — Tasks

Primary language: English. All documents and generated code for this feature MUST be written in English unless a specific exception is documented.

Purpose: break the work into small, PR-friendly tasks mapped to the stories and tests.

1. [ ] Task 01 — Spec: Stories & Requirements
   - Create `specs/05-tui/stories.md` and `requirements.md`. (PR: docs only)
   - Acceptance: files added and reviewed.

2. [ ] Task 02 — CLI skeleton & parsing
   - Add a `cli` submodule (or extend `main.rs`) to parse commands: `power`, `set voltage`, `get voltage`, `set current`, `get current`, `status`.
   - Use `clap` or an existing parser in the project.
   - Implement `--help` and a global `--json` flag.
   - Tests: unit tests for parsing and argument validation.

3. [ ] Task 03 — Implement `power on|off`
   - Implement calls to the device API (driver) to change power state.
   - Return text and JSON output depending on the flag.
   - Integration tests using the `emulator` driver.

4. [ ] Task 04 — Implement `set/get voltage`
   - Validate bounds and format (e.g., two decimal places for V).
   - Call the driver, read measured value, output text/JSON.
   - Tests: unit (validation), integration (`emulator`), JSON schema.

5. [ ] Task 05 — Implement `set/get current`
   - Same as voltage but for current.
   - Tests: unit + integration.

6. [ ] Task 06 — Implement `status` summary
   - Aggregate `DeviceState` and format text/JSON output.
   - Tests: validate timestamps are included and JSON schema stability.

7. [ ] Task 07 — Logging & Audit
   - Add persistence of commands and responses (rotating file or existing logging system).
   - Ensure logs do not expose secrets.
   - Tests: verify log entries for critical commands.

8. [ ] Task 08 — Error handling and edge cases
   - Tests for device disconnected, out-of-range values, hardware faults.
   - Document error behaviors in `docs/`.

9. [ ] Task 09 — Docs & CI
   - Document every command in `docs/` (PR-001 requirement).
   - Add CLI tests to CI workflows; ensure `cargo fmt` and `cargo clippy` pass.

10. [ ] Task 10 — Release notes / Migration
   - If CLI breaking changes occur: write migration notes and bump major version.

Notes on PR strategy:
- Prefer separate PRs for Tasks 02–06 (small surface & tests included).
- Tasks 03–06 should provide `--json` from the first iteration to satisfy PR-008.

References:
- Origin spec: `specs/5-tui.md` (migrated to this folder).
