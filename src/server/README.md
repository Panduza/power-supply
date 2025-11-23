# Module: server

## Functional Requirements

- Provides the main server logic and orchestration for the power supply application.

- The main operation sequence of the server application is:
    - Configure tracing first to be able to generate logs.
    - Parse CLI arguments.
    - Parse server main config file.
    - Start server services in a separated task.
    - Start TUI at the end if requested by user.

## Technical Requirements

- Written in Rust, organized as a module with submodules for each server component.
- Relies on internal modules for GUI, state, and various interfaces.
- Integrates with the rest of the application via public use statements.
