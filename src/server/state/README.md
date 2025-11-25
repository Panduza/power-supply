# Module: server/state


## Functional Requirements

- Provide a global state structure (`ServerState`) for sharing data between background services and the TUI.
- Manage:
	- `factory`: Arc<Mutex<Factory>> — the device driver factory
	- `server_config`: Arc<Mutex<ServerMainConfig>> — the server configuration
	- `instances`: Arc<Mutex<HashMap<String, MqttRunnerHandler>>> — running MQTT instances keyed by name
- Allow starting and stopping background runtime services via async methods:
	- `start_services(&self) -> anyhow::Result<()>` — instantiate and register all configured devices
- Expose available instance names for use by other modules (e.g., TUI) via:
	- `instances_names(&self) -> Vec<String>`

_Observability_
The state must be be fully loaded before any other component can start.
At the end of the service initialization sequence, the state must emit a signal.

## Technical Requirements

- Uses `Arc<Mutex<...>>` for thread-safe shared state.
- Integrates with the server's factory, configuration, and MQTT runner modules.
- Exposes async methods for runtime management and instance enumeration.
- Implements `PartialEq` for pointer-based equality of state fields.
- Use `tokio::sync::watch` for the observability signal.
