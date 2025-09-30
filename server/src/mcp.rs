mod tools;

use axum::Router;
use rmcp::transport::{
    streamable_http_server::session::local::LocalSessionManager, StreamableHttpService,
};
use rmcp::{RoleServer, Service};
use std::io::Error as IoError;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::oneshot;
use tower_http::cors::CorsLayer;

pub struct McpServer {}

impl McpServer {
    //
    // Must take a list of psu names to manage
    // for each name
    //  create an endpoint with the name and a service
    //

    /// Starts the server with the given service
    ///
    pub async fn run<S>(service: S) -> Result<(), IoError>
    where
        S: Service<RoleServer> + Clone + Send + 'static,
    {
        // Define the MCP endpoint
        let endpoint = "/power_supply";

        // Create the streamable HTTP service for MCP protocol handling
        let mcp_service = StreamableHttpService::new(
            move || Ok(service.clone()),
            LocalSessionManager::default().into(),
            Default::default(),
        );

        // Build the router with routes and middleware
        let app: Router = Router::new()
            // MCP endpoint - using streamable_http_server for MCP protocol handling
            .nest_service(endpoint, mcp_service)
            // Add CORS middleware
            .layer(CorsLayer::permissive());

        // Bind and serve the application
        let bind_address = "127.0.0.1:3000";
        let listener = TcpListener::bind(&bind_address).await?;

        tracing::info!("MCP server listening on {}{}", bind_address, endpoint);

        // Set up shutdown signal handling
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let mut shutdown_signal = Some(shutdown_rx);

        // Spawn a task to listen for shutdown signals
        tokio::spawn(async move {
            let _ = signal::ctrl_c().await;
            tracing::info!("Received shutdown signal");
            let _ = shutdown_tx.send(());
        });

        // Start the server with graceful shutdown
        let server = axum::serve(listener, app);

        if let Some(shutdown_rx) = shutdown_signal.take() {
            server
                .with_graceful_shutdown(async move {
                    let _ = shutdown_rx.await;
                })
                .await?;
        } else {
            server.await?;
        }

        tracing::info!("McpServer shutdown complete");

        Ok(())
    }
}
