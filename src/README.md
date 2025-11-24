# Module: MAIN

## Functional Requirements

### MAIN

Main manage the application binary.
The purpose of the application is to provide multiple network interface to control power-supplies through different ways:
- MCP interface
- MQTT interface

This application also provides a minimal TUI to provide a simple control and monitoring interface to the user.

Requirements:
- The main application must handle server application, see `server/README.md`.
- The main function is a tokio async main.

### LIB

- lib.rs must handle client.

## Technical Requirements

- Uses Rust standard library.
