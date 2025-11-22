```markdown
# Tasks: TUI

**Purpose**: Break the `TUI` feature into small, focused tasks suitable for individual PRs. Each task includes `spec:` with the originating spec and an inline checkbox so implementers can mark progress.

Notes:
- The agent must consult `specs/0-general/1-reqs.md` for global constraints (toolchain, CI, linting, testing requirements) before emitting implementation tasks.

1. [ ] Task 01 — Spec: Stories & Requirements
	- spec: `specs/05-tui/1-reqs.md`
	- description: Convert `specs/05-tui/0-stories.md` into concrete functional requirements (FRs) with explicit acceptance criteria and testable outcomes. Add any missing non-functional requirements (terminal sizes, accessibility, keyboard navigation).
	- acceptance: `specs/05-tui/1-reqs.md` contains FRs for each story, each FR has at least one Acceptance Criteria entry and one Testable Outcome.
	- tests: manual review; examples in the file.
	- estimate: 1-2 hours
	- prerequisites: `specs/05-tui/0-stories.md`
	- blocking: true

2. [ ] Task 02 — Minimal TUI prototype (render control box)
	- spec: `specs/05-tui/1-reqs.md`
	- description: Implement a minimal TUI that renders the Control Box screen showing Power state (ON/OFF), Voltage value, and Current value with placeholder/static data.
	- acceptance: running the app in a TTY shows the Control Box with the three values and basic keyboard focus handling; no crashes on common terminal sizes (80x24+).
	- tests: manual verification; add a small smoke test that spawns the binary and asserts the initial output contains the expected labels.
	- estimate: 1 day
	- prerequisites: Task 01

3. [ ] Task 03 — Backend wiring & live updates
	- spec: `specs/05-tui/1-reqs.md`
	- description: Wire the TUI prototype to the emulator/driver (use existing `drivers/emulator.rs`) to read live power state, voltage, and current and display updates.
	- acceptance: values in the TUI update when the emulator state changes; manual steps to reproduce documented.
	- tests: integration test using the emulator as a fixture that toggles values and asserts the TUI reflects them (or an integration script for manual verification).
	- estimate: 1 day
	- prerequisites: Task 02, `drivers/emulator.rs`

4. [ ] Task 04 — Unit tests and component tests
	- spec: `specs/05-tui/1-reqs.md`
	- description: Add unit tests for parsing/command handling and component-level tests for TUI rendering logic (where possible without a full terminal).
	- acceptance: `cargo test` covers new modules with meaningful unit tests; component tests validate render output for key states.
	- tests: automated unit tests under `src/` or `server/gui/` as appropriate.
	- estimate: 0.5-1 day
	- prerequisites: Task 02, Task 03

5. [ ] Task 05 — CI, linting, docs and release notes
	- spec: `specs/05-tui/1-reqs.md`
	- description: Ensure new code passes `cargo fmt` and `cargo clippy`, add docs/README updates describing how to run and test the TUI, and add short release notes or changelog entry.
	- acceptance: CI passes locally (format/lint/tests) and `docs/` or `specs/05-tui/README.md` contains run instructions.
	- tests: run `cargo fmt -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`.
	- estimate: 2-3 hours
	- prerequisites: Tasks 02-04

``` 
