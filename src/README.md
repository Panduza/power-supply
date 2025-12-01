# Module: MAIN

## Functional Requirements

### MAIN

- The main function **must** be a tokio async main.
- Main **must** manage the server application, see `server/README.md`.

### LIB

- lib.rs must handle client library functionality.
- Client library provides PowerSupplyClient for external applications to interact with power supplies via MQTT.

## Technical Requirements

- Uses Rust standard library.
