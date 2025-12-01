mod mcp;
mod runners;
use crate::server::cli::Args as CliArgs;
use crate::server::config::ServerConfig;
use crate::server::services::runners::RunnersService;
// use crate::server::factory::Factory;
// use crate::server::mcp::McpServer;
// use crate::server::mqtt::MqttRunner;
use super::drivers;
use anyhow::Ok;
use pza_toolkit::rumqtt::broker::start_broker_in_thread;
use pza_toolkit::task_monitor::TaskMonitor;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::Mutex;
use tracing::error;
use tracing::info;

use mcp::McpService;

// Global state for sharing data between background services and GUI
#[derive(Clone)]
pub struct Services {
    /// Server configuration
    pub server_config: ServerConfig,

    /// Factory instance
    pub drivers_factory: Arc<Mutex<drivers::Factory>>,

    ///
    task_monitor: Arc<Mutex<Option<TaskMonitor>>>,

    /// Watch channel sender for ready signal
    ready_sender: Arc<Mutex<Option<watch::Sender<bool>>>>,

    /// Watch channel receiver for ready signal
    ready_receiver: watch::Receiver<bool>,
}

impl Debug for Services {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Services")
            .field("factory", &"Arc<Mutex<Factory>>")
            .field("server_config", &"Arc<Mutex<ServerConfig>>")
            .field("instances", &"Arc<Mutex<Vec<String>>>")
            .finish()
    }
}

impl PartialEq for Services {
    fn eq(&self, other: &Self) -> bool {
        // Arc::ptr_eq(&self.factory, &other.factory)
        // &&

        // && Arc::ptr_eq(&self.instances, &other.instances)
        Arc::ptr_eq(&self.ready_sender, &other.ready_sender)
    }
}

impl Services {
    /// Create a new Services instance
    pub fn new(server_config: ServerConfig, drivers_factory: Arc<Mutex<drivers::Factory>>) -> Self {
        let (ready_sender, ready_receiver) = watch::channel(false);
        Self {
            server_config,
            drivers_factory,
            task_monitor: Arc::new(Mutex::new(None)),

            ready_sender: Arc::new(Mutex::new(Some(ready_sender))),
            ready_receiver,
        }
    }

    // ------------------------------------------------------------------------------

    /// Get a receiver for the ready signal
    pub fn ready_receiver(&self) -> watch::Receiver<bool> {
        self.ready_receiver.clone()
    }

    // ------------------------------------------------------------------------------

    /// Start background runtime services
    pub async fn start(&self) -> anyhow::Result<()> {
        // Monitoring
        let (task_monitor, mut runner_tasks_event_receiver) = TaskMonitor::new("services");

        // Start built-in MQTT broker if configured
        {
            let broker_config = self.server_config.broker.clone();
            if broker_config.use_builtin == Some(true) {
                start_broker_in_thread(broker_config.clone())?;
                info!("Started built-in MQTT broker");
            }
        }

        // Start Runners service only if configured
        {
            let runners_config = self.server_config.runners.clone();
            match runners_config {
                None => {
                    info!("Runners service is disabled in configuration");
                }
                Some(_) => {
                    info!("Starting Runners service...");
                    let (runners, handle) = RunnersService::start(
                        self.server_config.clone(),
                        self.drivers_factory.clone(),
                    )
                    .await?;
                    task_monitor
                        .handle_sender()
                        .send(("runners".to_string(), handle))
                        .await?;
                }
            }
        }

        // // Start MCP server only if not disabled
        {
            McpService::start(self.server_config.clone()).await?;
            info!("Started MCP server");
        }

        // // Emit ready signal after all services are initialized
        // {
        //     let mut sender = self.ready_sender.lock().await;
        //     if let Some(tx) = sender.take() {
        //         let _ = tx.send(true);
        //         info!("Server state is ready - signal emitted");
        //     }
        // }

        // // Store the TaskMonitor instance
        // self.task_monitor.lock().await.replace(task_monitor.clone());

        // Monitor task events
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
    }

    // // ------------------------------------------------------------------------------

    // pub async fn instances_names(&self) -> Vec<String> {
    //     let instances = self.instances.lock().await;
    //     instances.clone()
    // }
}
