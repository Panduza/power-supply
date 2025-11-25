# Module: server/config

## Functional Requirements

- Handles server configuration loading and parsing for the power supply application.
- Provides types and functions to read, validate, and expose configuration options to the server.
- Supports configuration from file.

Structure of the configuration file:
- Main
    - 

## Technical Requirements

- Uses `serde` for serialization & deserialization.
- Each config struct must have its file and the main config must be located in mod.rs.

## Auto Testing Scenarios

- Test loading a valid configuration file and verifying all fields are parsed correctly.
