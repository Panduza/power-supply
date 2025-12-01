mod runner;
use core::task;
use pza_toolkit::task_monitor::TaskMonitor;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
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

        // Spawn a task to handle TaskMonitor events
        let handle = tokio::spawn(async move {
            loop {
                let event_recv = runner_tasks_event_receiver.recv().await;
                match event_recv {
                    Some(event) => {
                        error!("TaskMonitor event: {:?}", event);
                        // Handle the event as needed
                    }
                    None => {
                        error!("TaskMonitor pipe closed");
                        // Handle the error as needed
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
