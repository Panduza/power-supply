# Module: server/config

## Functional Requirements

- Handles server configuration loading and parsing for the power supply application.
- Provides types and functions to read, validate, and expose configuration options to the server.
- Supports configuration from file.

## Technical Requirements

- Uses `serde` for deserialization and `toml` or `yaml` for config file parsing.

## Auto Testing Scenarios

- Test loading a valid configuration file and verifying all fields are parsed correctly.

