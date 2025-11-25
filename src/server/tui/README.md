# Module: tui

This module provides a terminal user interface (TUI) for controlling all power supply instances simultaneously.

## Functional Requirements

*Instance Management*

- The TUI must display and manage all available power supply instances at the same time.
- Each instance is shown in its own widget.
- If there are no interface instances available, the application must stop.

*Instance Control*

- The user can see the power state, voltage, and current values for each instance.
- The user can toggle the power state of any instance by interacting with its widget.

## Technical Requirements

- Integrates with the main application logic and device drivers.
- Uses Rust TUI libraries (e.g., `ratatui`, `crossterm`) for rendering and input handling.
- May interact with async runtimes and device state.
- The TUI must create a mqtt client `pza_power_supply_lib::PowerSupplyClient` and use it to interact with the power supply.
- The TUI module must be splitted into clean widgets:
    - Power Supply Instance Widget

_Power Supply Instance Widget_

- Each power supply instance must be managed in a separate widget.
- The widget code must be located in `psi_widget.rs`.
- Each widget must be contained in a `Block` with:
    - The name of the instance as the block name
    - Rounded border type

## Manual Testing Scenarios

- [ ] No interface must lead to an application stop
    - Remove all instances from the server config
    - Start the application `panduza`
    - Check application TUI, it must display a block explaining that no instance are available.


