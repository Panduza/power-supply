mod runner;
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

pub struct Runners {
    ///
    task_monitor: Arc<Mutex<Option<TaskMonitor>>>,
}

impl Runners {
    /// Start the runners services
    pub async fn start(
        server_config: Arc<Mutex<ServerConfig>>,
        drivers_factory: Arc<Mutex<DriverFactory>>,
    ) -> anyhow::Result<(Self, JoinHandle<()>)> {
        // Monitoring
        let (task_monitor, mut runner_tasks_event_receiver) = TaskMonitor::new("runners");

        // Start MQTT runners for each configured device
        let mut instances = Vec::new();
        let factory = drivers_factory.lock().await;
        info!("Starting server runtime services...");
        if let Some(devices) = &server_config.lock().await.runners {
            for (name, device_config) in devices {
                let instance = factory.instanciate_driver(device_config.clone())?;
                instances.push(name.clone());
                // MqttRunner::start(name.clone(), task_monitor.clone(), instance).await?;
            }
        }
        // .instances.lock().await = instances;

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
                        break;
                    }
                }
            }
        });

        //
        Ok((
            Self {
                task_monitor: Arc::new(Mutex::new(Some(task_monitor))),
            },
            handle,
        ))
    }
}
