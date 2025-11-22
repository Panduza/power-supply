# Requirements: TUI

## Summary

Terminal User Interface (TUI) for real-time power supply control and monitoring, providing essential data visibility (power state, voltage, current) through a CLI flag `--tui` as specified in `specs/05-tui/0-stories.md`.

## Functional Requirements

- 5-FR1: CLI Integration
  - The application MUST accept a `--tui` command-line flag to launch the TUI mode
  - **Acceptance Criteria**: Running `pza-power-supply --tui` starts the TUI; without flag, normal behavior unchanged
  - **Testable Outcome**: CLI help shows `--tui` option; process spawned with flag enters TUI event loop

- 5-FR2: Control Box Display
  - The TUI MUST display a Control Box showing power state (ON/OFF), voltage value, and current value
  - **Acceptance Criteria**: All three data points visible with clear labels and units; layout stable at minimum terminal size
  - **Testable Outcome**: Visual verification captures expected labels; resizing maintains readability

- 5-FR3: Live Data Updates
  - The Control Box MUST reflect backend state changes within 500ms refresh interval
  - **Acceptance Criteria**: Emulator value changes appear in TUI within specified interval; no visual artifacts
  - **Testable Outcome**: Integration test modifies emulator state and verifies UI update timing

- 5-FR4: Basic Interaction
  - The TUI MUST support power toggle (Space key) and clean exit (Q/Esc keys)
  - **Acceptance Criteria**: Keypress toggles power state in backend; exit keys perform graceful shutdown
  - **Testable Outcome**: Unit test for key mapping; integration test verifies state changes

- 5-FR5: Terminal Compatibility
  - The TUI MUST handle minimum terminal size (80x24) and display warning if smaller
  - **Acceptance Criteria**: Below minimum shows clear message; normal operation at/above minimum
  - **Testable Outcome**: Automated test with controlled terminal dimensions

- 5-FR6: Error Resilience
  - The TUI MUST handle backend disconnection gracefully with status indication
  - **Acceptance Criteria**: Lost connection shows non-blocking status panel; auto-retry on recovery
  - **Testable Outcome**: Mock adapter failure test; UI remains responsive during error state

## Non-Functional Requirements

- 5-NFR1: Performance
  - Frame rate MUST not exceed 30 FPS; CPU usage SHOULD remain under 5% during idle
  - **Acceptance Criteria**: Measured frame interval ≥33ms; CPU monitoring shows acceptable usage
  - **Testable Outcome**: Performance benchmark captures metrics within thresholds

- 5-NFR2: Accessibility
  - Focus indicators MUST be visible; keyboard navigation MUST be intuitive
  - **Acceptance Criteria**: Clear focus highlighting; help overlay available (? key)
  - **Testable Outcome**: Manual accessibility review; help system functional test

- 5-NFR3: Code Quality (per 1-TR2, 1-TR3)
  - Module structure MUST follow `src/tui/*.rs` layout; functions MUST be focused (≤80 lines)
  - **Acceptance Criteria**: No `mod.rs` files; individual modules for concerns; CI passes lint checks
  - **Testable Outcome**: `cargo clippy` and `cargo fmt --check` pass; code review confirms structure

## Technical Requirements

- 5-TR1: The crate ratatui must be used to code the TUI.
- 5-TR2: The TUI source code must be located in `src/tui` directory

## NEED CLARIFICATION

- Should voltage/current setpoints be adjustable via TUI, or display-only?
- What specific error messages should appear for different backend failure modes?
- Are there color/theme preferences for the Control Box display?
