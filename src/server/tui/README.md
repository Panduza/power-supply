# Module: tui

## Functional Requirements

- Provides a terminal user interface (TUI) for controlling power supply instances.
- Supports launching the TUI for a specific instance from the CLI.
- The user can see the power state, the voltage and current values.
- The user can toggle the power state by clicking on it.

## Technical Requirements

- Integrates with the main application logic and device drivers.
- Uses Rust TUI libraries (e.g., `ratatui`, `crossterm`) for rendering and input handling.
- May interact with async runtimes and device state.
- The TUI must create a mqtt client `pza_power_supply_lib::PowerSupplyClient` and use it to interact with the power supply.
