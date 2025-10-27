use crate::ServerState;
use pza_toolkit::rumqtt::broker::start_broker_in_thread;
use std::sync::Arc;
use tracing::info;

///
///
pub async fn server_services(server_state: Arc<ServerState>) -> anyhow::Result<()> {
    {
        let broker_config = &server_state.server_config.lock().await.broker;
        if broker_config.use_builtin == Some(true) {
            start_broker_in_thread(broker_config.clone())?;
            info!("Started built-in MQTT broker");
        }
    }

    loop {
        // Placeholder for service tasks
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
