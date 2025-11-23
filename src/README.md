# Module: MAIN

## Functional Requirements

- The main operation sequence of the application is:
    - Configure tracing first to be able to generate logs.
    - Parse CLI arguments.
    - Parse server main config file.
    - Start server services in a separated task.
    - Start TUI at the end if requested by user.

## Technical Requirements

- Uses Rust standard library.
