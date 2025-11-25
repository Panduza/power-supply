use crate::server::config::ServerMainConfig;
use crate::server::factory::Factory;
use crate::server::mcp::McpServer;
use crate::server::mqtt::MqttRunner;
use crate::server::mqtt::MqttRunnerHandler;
use pza_toolkit::rumqtt::broker::start_broker_in_thread;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::Mutex;
use tracing::info;

// Global state for sharing data between background services and GUI
#[derive(Clone, Debug)]
pub struct ServerState {
    /// Factory instance
    pub factory: Arc<Mutex<Factory>>,

    /// Server configuration
    pub server_config: Arc<Mutex<ServerMainConfig>>,

    /// Names of available instances
    pub instances: Arc<Mutex<HashMap<String, MqttRunnerHandler>>>,

    /// Watch channel sender for ready signal
    ready_sender: Arc<Mutex<Option<watch::Sender<bool>>>>,

    /// Watch channel receiver for ready signal
    ready_receiver: watch::Receiver<bool>,
}

impl PartialEq for ServerState {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.factory, &other.factory)
            && Arc::ptr_eq(&self.server_config, &other.server_config)
            && Arc::ptr_eq(&self.instances, &other.instances)
            && Arc::ptr_eq(&self.ready_sender, &other.ready_sender)
    }
}

impl ServerState {
    /// Create a new ServerState instance
    pub fn new(factory: Arc<Mutex<Factory>>, server_config: Arc<Mutex<ServerMainConfig>>) -> Self {
        let (ready_sender, ready_receiver) = watch::channel(false);
        Self {
            factory,
            server_config,
            instances: Arc::new(Mutex::new(HashMap::new())),
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
    pub async fn start_services(&self) -> anyhow::Result<()> {
        // Start built-in MQTT broker if configured
        {
            let broker_config = self.server_config.as_ref().lock().await.broker.clone();
            if broker_config.use_builtin == Some(true) {
                start_broker_in_thread(broker_config.clone())?;
                info!("Started built-in MQTT broker");
            }
        }

        //
        {
            let mut instances = HashMap::new();
            let factory = self.factory.lock().await;
            info!("Starting server runtime services...");
            if let Some(devices) = &self.server_config.lock().await.devices {
                for (name, device_config) in devices {
                    let instance = factory.instanciate_driver(device_config.clone())?;

                    instances.insert(name.clone(), MqttRunner::start(name.clone(), instance)?);
                }
            }
            *self.instances.lock().await = instances;
        }

        {
            let instance_names = self.instances_names().await;
            let ccc = self.server_config.as_ref().lock().await.clone();
            McpServer::run(ccc, instance_names).await?;
        }

        // Emit ready signal after all services are initialized
        {
            let mut sender = self.ready_sender.lock().await;
            if let Some(tx) = sender.take() {
                let _ = tx.send(true);
                info!("Server state is ready - signal emitted");
            }
        }

        loop {
            // Placeholder for service tasks
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }

    pub async fn instances_names(&self) -> Vec<String> {
        let instances = self.instances.lock().await;
        instances.keys().cloned().collect()
    }
}
