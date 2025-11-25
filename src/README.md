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

- lib.rs must handle client library functionality.
- Client library provides PowerSupplyClient for external applications to interact with power supplies via MQTT.

## Sub-Module Specifications

- Server Module: `server/README.md` - Main server logic and orchestration
- Client Module: Client library implementation (see `client/` directory)

## Technical Requirements

- Uses Rust standard library.
