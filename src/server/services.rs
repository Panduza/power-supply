use crate::server::mcp::McpServer;
use crate::ServerState;
use pza_toolkit::rumqtt::broker::start_broker_in_thread;
use std::sync::Arc;
use tracing::info;

/// Start background services for the server
///
pub async fn server_services(server_state: Arc<ServerState>) -> anyhow::Result<()> {
    // Start built-in MQTT broker if configured
    {
        let broker_config = server_state
            .server_config
            .as_ref()
            .lock()
            .await
            .broker
            .clone();
        if broker_config.use_builtin == Some(true) {
            start_broker_in_thread(broker_config.clone())?;
            info!("Started built-in MQTT broker");
        }
    }

    {
        server_state.start_runtime().await?;
    }

    {
        let instance_names = server_state.instances_names().await;
        let ccc = server_state.server_config.as_ref().lock().await.clone();
        McpServer::run(ccc, instance_names).await?;
    }

    loop {
        // Placeholder for service tasks
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
