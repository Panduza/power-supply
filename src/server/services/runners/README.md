# Module: Runners

## Functional Requirements

- Start and manage runner tasks for configured power-supply devices.
- For each configured runner name:
  - Instantiate the appropriate device driver via the provided `DriverFactory`.
  - Create and start a `Runner` instance that manages device communication (MQTT, protocols, etc.).
  - Register each runner task with a `TaskMonitor` so lifecycle events (panic, stop, errors) can be observed and handled.
- Provide a single async entrypoint (`RunnersService::start`) that initializes monitoring and spawns the necessary background tasks.
- Surface runtime errors via `anyhow::Error` and ensure the caller can observe or await the spawned monitor task.
- Keep task-monitor-related resources alive for the lifetime of the service so that task events are not dropped.
- Graceful shutdown: respond to shutdown signals and allow in-flight runner tasks to stop cleanly.

- Automatic reboot on crash: when a runner task crashes or panics the system must attempt to reboot it using the `TaskMonitor` events. Restart attempts should follow a configurable policy (e.g. exponential backoff with configurable max retries) to avoid tight crash-restart loops.

## Technical Requirements

- Uses the `tokio` runtime for async concurrency and task spawning.
- Uses `pza_toolkit::task_monitor::TaskMonitor` to track runner tasks and receive lifecycle events.
- Uses `tracing`/`tracing::info`/`tracing::error` for structured logging.
- Accepts a `ServerConfig` (application configuration) and an `Arc<Mutex<DriverFactory>>` to instantiate per-runner drivers.
- Exposes `pub async fn start(server_config: ServerConfig, drivers_factory: Arc<Mutex<DriverFactory>>)
  -> anyhow::Result<(Self, JoinHandle<Result<(), anyhow::Error>>)>` which returns the service instance and a handle for the monitor task.
- Concurrency and synchronization:
  - Driver factory is shared behind `Arc<Mutex<...>>` to allow safe instantiation from the async context.
  - The TaskMonitor handle is stored inside the `RunnersService` (inside an `Arc<Mutex<Option<TaskMonitor>>>`) to keep it alive.
- Error handling: propagate initialization errors (driver instantiation, runner creation) using `anyhow` and log runtime TaskMonitor events.

- Restart-on-crash behavior: implement a handler for `TaskMonitor` events (e.g. `TaskPanicOMG` / `TaskStopWithPain`) that can:
  - identify the affected runner by name;
  - re-instantiate the driver via the `DriverFactory` (or reuse cached factory data);
  - call `Runner::start` again to spawn a fresh runner task;
  - register the replacement task with the same `TaskMonitor` so monitoring continues;
  - apply a restart policy (backoff, max attempts) and record restart attempts in metrics/logs.

Note: to support safe restarts, `Runner::start` and driver instantiation should be idempotent / test-friendly so they can be invoked multiple times for the same named runner.
