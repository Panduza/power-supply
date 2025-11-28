mod tools;

use axum::Router;
use pza_power_supply_client::constants;
use rmcp::transport::{
    streamable_http_server::session::local::LocalSessionManager, StreamableHttpService,
};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::oneshot;
use tower_http::cors::CorsLayer;

use tools::PowerSupplyService;

use crate::server::config::ServerMainConfig;

pub struct McpServer {}

impl McpServer {
    //
    // Must take a list of psu names to manage
    // for each name
    //  create an endpoint with the name and a service
    //

    /// Starts the server with the given service
    ///
    pub async fn run(config: ServerMainConfig, psu_names: Vec<String>) -> anyhow::Result<()> {
        // Bind and serve the application
        let bind_address = format!("{}:{}", config.mcp.host, config.mcp.port);
        let listener = TcpListener::bind(&bind_address).await?;

        //
        let mut app = Router::new().layer(CorsLayer::permissive());

        //
        for psu_name in psu_names {
            let service_tools = PowerSupplyService::new(config.clone(), psu_name.clone())?;

            // Create the streamable HTTP service for MCP protocol handling
            let mcp_service = StreamableHttpService::new(
                move || Ok(service_tools.clone()),
                LocalSessionManager::default().into(),
                Default::default(),
            );

            // MCP endpoint - using streamable_http_server for MCP protocol handling
            app = app.nest_service(
                format!("/{}/{}", constants::SERVER_TYPE_NAME, &psu_name).as_str(),
                mcp_service,
            );

            //
            tracing::info!(
                "MCP server listening on {}{}",
                bind_address,
                format!("/{}/{}", constants::SERVER_TYPE_NAME, &psu_name)
            );
        }

        // Set up shutdown signal handling
        let (shutdown_tx, _shutdown_rx) = oneshot::channel();

        // Spawn a task to listen for shutdown signals
        tokio::spawn(async move {
            let _ = signal::ctrl_c().await;
            tracing::info!("Received shutdown signal");
            let _ = shutdown_tx.send(());
        });

        // Start the server with graceful shutdown
        let server = axum::serve(listener, app);

        // Démarrer le serveur dans une tâche séparée
        let _server_handle = tokio::spawn(async move { server.await });

        // Attendre soit l'arrêt du serveur soit le signal d'arrêt
        // tokio::select! {
        //     result = server_handle => {
        //         match result {
        //             Ok(server_result) => server_result?,
        //             Err(e) => return Err(IoError::new(std::io::ErrorKind::Other, e)),
        //         }
        //     }
        //     _ = shutdown_signal.take().unwrap() => {
        //         tracing::info!("Graceful shutdown initiated");
        //     }
        // }

        // if let Some(shutdown_rx) = shutdown_signal.take() {
        //     server
        //         .with_graceful_shutdown(async move {
        //             let _ = shutdown_rx.await;
        //         })
        //         .await?;
        // } else {
        //     server.await?;
        // }

        Ok(())
    }
}
