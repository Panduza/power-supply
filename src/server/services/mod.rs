mod mcp;
mod runners;
mod tui;
use crate::server::cli::Args as CliArgs;
use crate::server::config::ServerConfig;
use crate::server::services::runners::RunnersService;
use crate::server::services::tui::TuiService;
// use crate::server::factory::Factory;
// use crate::server::mcp::McpServer;
// use crate::server::mqtt::MqttRunner;
use super::drivers;
use anyhow::Ok;
use pza_toolkit::rumqtt::broker::start_broker_in_thread;
use pza_toolkit::task_monitor::TaskMonitor;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::signal;
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

    /// Runners service instance
    runners: Option<Arc<Mutex<RunnersService>>>,

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
            runners: None,
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
    pub async fn start(&mut self) -> anyhow::Result<()> {
        // Monitoring
        let (mut task_monitor, mut runner_tasks_event_receiver) = TaskMonitor::new("services");

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
                    self.runners = Some(Arc::new(Mutex::new(runners)));
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

        {
            // Start TUI service only if not disabled
            if self.server_config.tui.enable.unwrap_or(true) {
                info!("Starting TUI service...");
                let tui_handle = TuiService::start();
                task_monitor
                    .handle_sender()
                    .send(("tui".to_string(), tui_handle))
                    .await?;
            } else {
                info!("TUI service is disabled in configuration");
            }
        }

        // // Emit ready signal after all services are initialized
        // {
        //     let mut sender = self.ready_sender.lock().await;
        //     if let Some(tx) = sender.take() {
        //         let _ = tx.send(true);
        //         info!("Server state is ready - signal emitted");
        //     }
        // }

        // Setup Ctrl+C signal handler
        let mut ctrl_c = Box::pin(signal::ctrl_c());

        // Monitor task events and signals
        loop {
            tokio::select! {
                // Handle Ctrl+C signal
                _ = ctrl_c.as_mut() => {
                    info!("Received Ctrl+C signal, shutting down gracefully...");

                    // Cancel all running tasks
                    task_monitor.cancel_all_monitored_tasks().await;
                    info!("All tasks have been cancelled");

                    return Ok(());
                }

                // Handle task monitor events
                event_recv = runner_tasks_event_receiver.recv() => {
                    match event_recv {
                        Some(event) => {
                            match event {
                                pza_toolkit::task_monitor::Event::TaskMonitorError(event_body) => {
                                    // error!("Task monitor error: {}", event_body.error);
                                }
                                pza_toolkit::task_monitor::Event::TaskStopProperly(event_body) => {
                                    info!("Task '{}' stopped properly", event_body.task_name);
                                    if event_body.task_name == "tui" {
                                        // TUI stopped, shut down other services gracefully
                                        info!("TUI service stopped, shutting down other services...");
                                        task_monitor.cancel_all_monitored_tasks().await;
                                        return Ok(());
                                    }
                                }
                                pza_toolkit::task_monitor::Event::TaskStopWithPain(event_body) => {
                                    // error!("Task '{}' stopped with error: {:?}", event_body.task_name, event_body.error);
                                    // Continue monitoring other tasks
                                }
                                pza_toolkit::task_monitor::Event::TaskPanicOMG(event_body) => {
                                    // error!("Task '{}' panicked: {}", event_body.task_name, event_body.panic_info);
                                    // Decide whether to restart the task or continue
                                }
                                _ => {}
                            }
                        }
                        None => {
                            info!("TaskMonitor pipe closed, shutting down...");
                            return Ok(());
                        }
                    }
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
