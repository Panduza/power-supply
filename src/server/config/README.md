# Module: server/config

## Functional Requirements

- Handles server configuration loading and parsing for the power supply application.
- Provides types and functions to read, validate, and expose configuration options to the server.
- Supports configuration from file.

Structure of the configuration file:
- Main
    - TUI
        - `power_toggle_key`: The user key to toggle the power.
    - MCP
        - `enable`: Enable or disable the MCP server (bool).
        - `host`: Host address for the MCP server (string).
        - `port`: Port number for the MCP server (integer).
    - Power Supply
        - `model`: Model identifier for the power supply (string).
        - `description`: Optional description of the power supply (string, optional).
        - `security_min_voltage`: Minimum allowed voltage (float, optional).
        - `security_max_voltage`: Maximum allowed voltage (float, optional).
        - `security_min_current`: Minimum allowed current (float, optional).
        - `security_max_current`: Maximum allowed current (float, optional).

## Technical Requirements

- Uses `serde` for serialization & deserialization.
- Each config struct must have its file and the main config must be located in mod.rs.

## Auto Testing Scenarios

- Test loading a valid configuration file and verifying all fields are parsed correctly.
