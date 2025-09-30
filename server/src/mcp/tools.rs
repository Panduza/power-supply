use std::sync::{Arc, Mutex};

use rmcp::handler::server::router::prompt::PromptRouter;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::*;
use rmcp::prompt_handler;
use rmcp::prompt_router;
use rmcp::service::RequestContext;
use rmcp::tool;
use rmcp::tool_handler;
use rmcp::tool_router;
use rmcp::ErrorData as McpError;
use rmcp::RoleServer;
use rmcp::ServerHandler;
use tracing::debug;
use tracing::info;

use panduza_power_supply_client::ClientBuilder;
use panduza_power_supply_client::PowerSupplyClient;

use crate::config::GlobalConfig;

#[derive(Clone)]
struct PowerSupplyState {
    client: PowerSupplyClient,
}

/// Service structure that handles MCP protocol interactions and manages
/// connections to the Panduza platform.
#[derive(Clone)]
pub struct PowerSupplyService {
    /// Tool router for MCP tools
    tool_router: ToolRouter<PowerSupplyService>,
    /// Prompt router for MCP prompts
    prompt_router: PromptRouter<PowerSupplyService>,

    state: Arc<Mutex<PowerSupplyState>>,
}

impl PowerSupplyService {
    //--------------------------------------------------------------------------

    pub fn new(config: GlobalConfig, psu_name: String) -> Self {
        let client = ClientBuilder::from_broker_config(config.broker.clone()).build();
        let client = client.get_power_supply_client(psu_name);
        debug!("Client initialized");

        Self {
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
            state: Arc::new(Mutex::new(PowerSupplyState { client })),
        }
    }
}

#[tool_router]
impl PowerSupplyService {
    //--------------------------------------------------------------------------

    // /// Get the current output enable status of the power supply
    // #[tool(description = "Get the current output enable status of the power supply")]
    // async fn output_enable_get(&self) -> Result<CallToolResult, McpError> {
    //     debug!("MCP tool 'output_enable_get' called");

    //     info!("Retrieved output enable status: {}", self.output_enable);
    //     Ok(CallToolResult::success(vec![Content::text(format!(
    //         "Output enable status: {}",
    //         self.output_enable
    //     ))]))
    // }

    //--------------------------------------------------------------------------

    /// Enable the power supply output
    #[tool(description = "Enable the power supply output (turn on power)")]
    async fn output_enable(&self) -> Result<CallToolResult, McpError> {
        let client = {
            let psu_state = self.state.lock().unwrap();
            psu_state.client.clone()
        };

        client.enable_output().await.map_err(|_e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                "Failed to enable power supply output",
                None,
            )
        })?;

        info!("Successfully enabled power supply output");
        Ok(CallToolResult::success(vec![Content::text(
            "Power supply output enabled".to_string(),
        )]))
    }

    //--------------------------------------------------------------------------

    /// Disable the power supply output
    #[tool(description = "Disable the power supply output (turn off power)")]
    async fn output_disable(&self) -> Result<CallToolResult, McpError> {
        let client = {
            let psu_state = self.state.lock().unwrap();
            psu_state.client.clone()
        };

        client.disable_output().await.map_err(|_e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                "Failed to disable power supply output",
                None,
            )
        })?;

        info!("Successfully disabled power supply output");
        Ok(CallToolResult::success(vec![Content::text(
            "Power supply output disabled".to_string(),
        )]))
    }

    //--------------------------------------------------------------------------
}

#[prompt_router]
impl PowerSupplyService {
    // No prompts specified in requirements, but trait requires this implementation
    // Implementation block is needed for the macro to work properly
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for PowerSupplyService {
    //--------------------------------------------------------------------------

    /// Get server information and capabilities
    fn get_info(&self) -> ServerInfo {
        debug!("MCP get_info called");

        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                // Include Panduza topics spec in the server instructions for discoverability
                "This server provides access to a power supply.".to_string(),
            ),
        }
    }
}
