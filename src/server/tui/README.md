# Module: tui

This module provides a terminal user interface (TUI) for controlling power supply instances.

## Functional Requirements

*Instance Selection*

- The TUI must be started for a specific instance from the CLI.
- The user can provide an optional instance_name, by default the tui take the first instance available.
- If there is not interface instance available, the application must stop.

*Instance Control*

- The user can see the power state, the voltage and current values.
- The user can toggle the power state by clicking on it.

## Technical Requirements

- Integrates with the main application logic and device drivers.
- Uses Rust TUI libraries (e.g., `ratatui`, `crossterm`) for rendering and input handling.
- May interact with async runtimes and device state.
- The TUI must create a mqtt client `pza_power_supply_lib::PowerSupplyClient` and use it to interact with the power supply.

## Manual Testing Scenarios

- [ ] No interface must lead to an application stop
    - Remove all instance from the server config
    - Start the application `panduza`
    - Check application stop with an error message


