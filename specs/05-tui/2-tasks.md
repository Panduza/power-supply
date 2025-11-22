# Tasks: TUI (replanned)

Purpose: provide a small, testable, docs-first plan to implement the TUI Control Box. Task 01 is blocking: finish requirements from `specs/05-tui/0-stories.md` before implementation.

Note: the task planner MUST consult `specs/0-general/1-reqs.md` for global constraints (toolchain, CI/linting/testing) when producing implementation tasks.

1. [ ] Task 01 — Complete spec: Stories → Requirements
   - spec: `specs/05-tui/1-reqs.md`
   - description: Finalize `1-reqs.md` by converting the `0-stories.md` content into concrete Functional Requirements (FRs), Non-Functional Requirements, Acceptance Criteria and Testable Outcomes. If details are missing, record explicit `NEED CLARIFICATION` questions and wait for answers.
   - acceptance: `specs/05-tui/1-reqs.md` lists FRs covering the Control Box story, each FR has at least one Acceptance Criteria and one Testable Outcome; any open questions appear under `NEED CLARIFICATION`.
   - tests: manual review of the requirements file.
   - estimate: 1-2 hours
   - prerequisites: `specs/05-tui/0-stories.md`
   - blocking: true

2. [ ] Task 02 — Static prototype: Control Box rendering
   - spec: `specs/05-tui/1-reqs.md`
   - description: Implement a minimal static TUI view that renders the Control Box with labeled placeholders for Power state, Voltage, and Current. No backend wiring yet.
   - acceptance: running the app shows the Control Box UI with correct labels and placeholder values; layout is stable at the minimum supported terminal size defined in the spec.
   - tests: manual smoke test; include a small test that spawns the binary and confirms expected labels in output when run in a TTY (or a trimmed integration harness).
   - estimate: 4-8 hours
   - prerequisites: Task 01

3. [ ] Task 03 — Live updates (emulator integration)
   - spec: `specs/05-tui/1-reqs.md`
   - description: Connect the static prototype to the emulator driver so the Control Box displays live values and responds to emulator state changes.
   - acceptance: changes in the emulator (power on/off, voltage/current updates) are reflected in the TUI within the refresh interval specified by the spec; documented manual verification steps exist.
   - tests: integration test using the emulator fixture to toggle values and verify UI updates (or an integration script for manual verification).
   - estimate: 1 day
   - prerequisites: Task 02, `drivers/emulator.rs`

4. [ ] Task 04 — Interaction: keyboard navigation & primary actions
   - spec: `specs/05-tui/1-reqs.md`
   - description: Add keyboard controls for primary actions (e.g., toggle Power, adjust setpoints if in-scope). Implement clear focus and basic accessibility considerations per spec.
   - acceptance: primary actions are reachable via keyboard, focus is visible, and actions trigger expected UI/backend behaviour.
   - tests: unit tests for input handling where possible; manual verification steps.
   - estimate: 8-12 hours
   - prerequisites: Task 02, Task 03

5. [ ] Task 05 — Tests, CI, docs
   - spec: `specs/05-tui/1-reqs.md`
   - description: Add unit tests, component tests for rendering logic, CI checks for `cargo fmt` and `cargo clippy`, and update `docs/` or add `specs/05-tui/README.md` with run & test instructions.
   - acceptance: `cargo test` covers new modules; `cargo fmt -- --check` and `cargo clippy` pass locally; docs include run and manual test instructions.
   - tests: automated tests + CI checks.
   - estimate: 4-8 hours
   - prerequisites: Tasks 02-04

Guidance for implementers:
- Keep each task small and produce one PR per task.
- Include `spec:` and `prerequisites:` in PR descriptions for traceability.
- If any global constraint from `specs/0-general/1-reqs.md` conflicts with implementation choices, surface it as `NEED CLARIFICATION` in `1-reqs.md`.

