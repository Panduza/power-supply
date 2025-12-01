mod runner;
use core::task;
use pza_toolkit::task_monitor::TaskMonitor;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration as TokioDuration};
use tracing::error;
use tracing::info;

use super::drivers::Factory as DriverFactory;
use crate::server::config::ServerConfig;
use runner::Runner;

pub struct RunnersService {
    /// Just to keep the monitor alive
    _task_monitor: Arc<Mutex<Option<TaskMonitor>>>,
}

impl RunnersService {
    /// Start the runners services
    pub async fn start(
        server_config: ServerConfig,
        drivers_factory: Arc<Mutex<DriverFactory>>,
    ) -> anyhow::Result<(Self, JoinHandle<Result<(), anyhow::Error>>)> {
        // Monitoring
        let (task_monitor, mut runner_tasks_event_receiver) = TaskMonitor::new("runners");

        // Start MQTT runners for each configured device
        let factory = drivers_factory.lock().await;
        info!("Starting server runtime services...");
        if let Some(devices) = &server_config.runners {
            for (name, device_config) in devices {
                info!("Starting runner for device '{}'", name);
                // Instanciate the driver
                let instance = factory.instanciate_driver(device_config.clone())?;

                // Start the runner
                let task_handle = Runner::start(name.clone(), instance).await?;

                // Register the task with the monitor
                task_monitor
                    .handle_sender()
                    .send((name.clone(), task_handle))
                    .await?;
            }
        }

        // Prepare data for monitor task (cloneable handles)
        let monitor_sender = task_monitor.handle_sender();
        let drivers_factory_clone = drivers_factory.clone();
        let monitor_config = server_config.clone();

        // Spawn a task to handle TaskMonitor events and perform restarts
        let handle = tokio::spawn(async move {
            // Per-runner restart attempt counter
            let mut restart_attempts: HashMap<String, usize> = HashMap::new();
            const MAX_RETRIES: usize = 5;
            const BASE_DELAY_MS: u64 = 1000;

            loop {
                let event_recv = runner_tasks_event_receiver.recv().await;
                match event_recv {
                    Some(event) => {
                        error!("TaskMonitor event: {:?}", event);

                        // Only handle panic/stop-with-error events for restart
                        match event {
                            pza_toolkit::task_monitor::Event::TaskPanicOMG(event_body)
                            | pza_toolkit::task_monitor::Event::TaskStopWithPain(event_body) => {
                                let task_name = event_body.task_name.clone();

                                // If the task corresponds to a configured runner, attempt restart
                                if let Some(runners_map) = &monitor_config.runners {
                                    if let Some(device_cfg) = runners_map.get(&task_name) {
                                        let attempts =
                                            restart_attempts.entry(task_name.clone()).or_insert(0);
                                        if *attempts >= MAX_RETRIES {
                                            error!(
                                                "Max restart attempts reached for '{}', giving up",
                                                task_name
                                            );
                                            continue;
                                        }

                                        let delay_ms = BASE_DELAY_MS
                                            .saturating_mul(2u64.pow(*attempts as u32));
                                        *attempts += 1;
                                        info!(
                                            "Scheduling restart for '{}' in {}ms (attempt {}/{})",
                                            task_name, delay_ms, *attempts, MAX_RETRIES
                                        );
                                        sleep(TokioDuration::from_millis(delay_ms)).await;

                                        // Try to re-instantiate the driver and start a new runner
                                        match drivers_factory_clone
                                            .lock()
                                            .await
                                            .instanciate_driver(device_cfg.clone())
                                        {
                                            Ok(instance) => {
                                                match Runner::start(task_name.clone(), instance)
                                                    .await
                                                {
                                                    Ok(task_handle) => {
                                                        // Register replacement task with the monitor
                                                        if let Err(e) = monitor_sender
                                                            .send((task_name.clone(), task_handle))
                                                            .await
                                                        {
                                                            error!("Failed to register restarted task '{}': {:?}", task_name, e);
                                                        } else {
                                                            info!("Successfully restarted runner '{}'", task_name);
                                                            // Reset attempts counter on success
                                                            restart_attempts
                                                                .insert(task_name.clone(), 0);
                                                        }
                                                    }
                                                    Err(e) => {
                                                        error!("Failed to start restarted runner '{}': {:?}", task_name, e);
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                error!("Failed to instantiate driver for '{}' during restart: {:?}", task_name, e);
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    None => {
                        error!("TaskMonitor pipe closed");
                        // End monitor task
                        return Ok(());
                    }
                }
            }
        });

        // Return the Runners instance
        Ok((
            Self {
                _task_monitor: Arc::new(Mutex::new(Some(task_monitor))),
            },
            handle,
        ))
    }
}
