# Tasks: TUI (ratatui-focused revision)

Purpose: deliver the TUI Control Box incrementally using `ratatui`, with tight, testable slices. Task 01 remains blocking until requirements are complete.

References:
- Global constraints: `specs/0-general/1-reqs.md`
- Stories: `specs/05-tui/0-stories.md`
- Requirements (to be finalized): `specs/05-tui/1-reqs.md`

Conventions:
- One PR per task; keep PR scope to the described acceptance.
- Each task must mention `spec:` and `prerequisites:` in PR description.
- If an acceptance criterion cannot be met due to missing clarity, add an item under `NEED CLARIFICATION` in `1-reqs.md` before proceeding.

0. [ ] Task 00 — Bootstrap & module skeleton
   - spec: `specs/05-tui/1-reqs.md`
   - description: Ensure `ratatui` & `crossterm` dependencies (already present) are acknowledged; create initial `src/tui` module skeleton: `layout.rs`, `render.rs`, `input.rs`, `state.rs` (empty stubs). Add a minimal event loop struct (`TuiApp`) without business logic.
   - acceptance: Compiles with new module files; running `--tui` (temporarily a no-op placeholder) enters and cleanly exits the loop with a log message.
   - tests: Simple unit test asserting `TuiApp::new()` returns default state; smoke run manual.
   - estimate: 1-2 hours
   - prerequisites: Cargo deps (present), stories file

1. [ ] Task 01 — Finalize requirements (blocking)
   - spec: `specs/05-tui/1-reqs.md`
   - description: Convert stories into Functional Requirements (FRs): CLI flag `--tui`, initial Control Box data points, minimum terminal size, refresh cadence, exit keys, error handling (lost backend), performance targets (frame interval), accessibility basics (focus visibility). Add Non-Functional (maintainability per 1-TR2 & 1-TR3). List Acceptance Criteria + Testable Outcomes for each FR. Record questions under `NEED CLARIFICATION` if gaps.
   - acceptance: Requirements file lists numbered FRs with AC + Testable Outcome; includes at least one performance constraint and terminal size; any ambiguous points appear under `NEED CLARIFICATION`.
   - tests: Manual review
   - estimate: 1-2 hours
   - prerequisites: Task 00, stories file
   - blocking: true

2. [ ] Task 02 — CLI flag integration (`--tui`)
   - spec: `specs/05-tui/1-reqs.md`
   - description: Wire `--tui` clap flag to invoke `TuiApp::run()`; ensure graceful fallback if terminal not interactive; add logging banner.
   - acceptance: Running binary with `--tui` starts event loop; without flag unchanged behaviour; help text lists flag.
   - tests: CLI integration test (spawn process with `--tui`, assert banner in stdout); unit test for clap config.
   - estimate: 1-2 hours
   - prerequisites: Task 01

3. [ ] Task 03 — Static Control Box layout
   - spec: `specs/05-tui/1-reqs.md`
   - description: Implement ratatui layout + widgets showing labels: Power: [placeholder], Voltage: [placeholder], Current: [placeholder]. No live data yet. Handle minimum terminal size check.
   - acceptance: Placeholders render correctly; resizing below minimum shows a concise warning panel; exit key works.
   - tests: Unit test for layout slice function; integration snapshot test capturing first frame text (if feasible) or regex-based assertions.
   - estimate: 3-5 hours
   - prerequisites: Task 02

4. [ ] Task 04 — State model & polling abstraction
   - spec: `specs/05-tui/1-reqs.md`
   - description: Add `state.rs` with struct holding power/voltage/current; implement a trait-based backend adapter (emulator first). Provide periodic tick update (Tokio interval). No user interaction yet.
   - acceptance: When emulator values change (manual or scripted), internal state updates (log trace) within refresh interval.
   - tests: Unit test for adapter trait mock; integration test simulating value change and asserting state.
   - estimate: 4-6 hours
   - prerequisites: Task 03, `drivers/emulator.rs`

5. [ ] Task 05 — Live rendering updates
   - spec: `specs/05-tui/1-reqs.md`
   - description: Bind state updates to UI redraw; replace placeholders with live values; implement basic formatting (units).
   - acceptance: Changing emulator values reflects on screen within defined interval; no flicker; CPU usage reasonable (< stated threshold if specified).
   - tests: Integration test capturing two frames with changed values; optional perf log assertion.
   - estimate: 4-6 hours
   - prerequisites: Task 04

6. [ ] Task 06 — Interaction: power toggle & exit/help keys
   - spec: `specs/05-tui/1-reqs.md`
   - description: Implement keyboard handling (e.g. Space to toggle power, Q/Esc to exit, ? for help overlay). Update backend through emulator adapter. Visible focus/feedback.
   - acceptance: Keypress toggles power (reflected in UI & emulator); help overlay appears/disappears; exit keys perform clean teardown.
   - tests: Unit tests for key mapping; integration script sending keystrokes (crossterm event injection) verifying state changes.
   - estimate: 5-8 hours
   - prerequisites: Task 05

7. [ ] Task 07 — Error handling & resilience
   - spec: `specs/05-tui/1-reqs.md`
   - description: Display non-blocking status panel for backend disconnect/timeouts; auto-retry strategy; graceful downgrade (keep last known values with stale indicator).
   - acceptance: Forced adapter error surfaces message; UI remains responsive; recovery clears indicator.
   - tests: Unit test using failing mock adapter; integration test injecting error.
   - estimate: 3-5 hours
   - prerequisites: Task 06

8. [ ] Task 08 — Tests, CI, docs consolidation
   - spec: `specs/05-tui/1-reqs.md`
   - description: Expand coverage (layout, adapter, interaction); ensure `cargo fmt -- --check` & `cargo clippy` pass; add `specs/05-tui/README.md` with run instructions, manual verification steps, troubleshooting. Update any performance notes.
   - acceptance: Test suite passes; coverage of key modules; README present; CI passes locally.
   - tests: All added unit/integration tests + tooling checks.
   - estimate: 4-6 hours
   - prerequisites: Tasks 03-07

9. [ ] Task 09 — Optional: Performance & polish
   - spec: `specs/05-tui/1-reqs.md`
   - description: Measure frame interval, CPU usage, memory footprint; minor UX refinements (colors, alignment) within spec boundaries.
   - acceptance: Metrics documented in README; no regressions in tests; visual enhancements approved.
   - tests: Manual metrics capture; optional benchmark harness.
   - estimate: 2-4 hours
   - prerequisites: Task 08

Guidance for implementers:
- Keep tasks narrow; avoid mixing rendering, state, and interaction in one PR.
- Respect general requirements: avoid large multi-purpose functions (1-TR3) and maintain module clarity (1-TR2).
- Surface any blocking ambiguity early by updating requirements with `NEED CLARIFICATION`.
- Prefer trait + mock for backend to keep tests fast and deterministic.

