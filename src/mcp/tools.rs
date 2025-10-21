use std::sync::Arc;
use tokio::sync::Mutex;

use rmcp::handler::server::router::prompt::PromptRouter;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
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
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::debug;
use tracing::info;

use crate::client::PowerSupplyClient;
use crate::client::PowerSupplyClientBuilder;

use crate::config::GlobalConfig;

#[derive(Serialize, Deserialize, JsonSchema)]
struct VoltageParams {
    voltage: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct CurrentParams {
    current: String,
}

#[derive(Clone)]
struct PowerSupplyState {
    client: PowerSupplyClient,
}

/// Service structure that handles MCP protocol interactions and manages
/// connections to the Panduza platform.
#[derive(Clone)]
pub struct PowerSupplyService {
    /// Power Supply Name provided by the user
    psu_name: String,

    /// Tool router for MCP tools
    tool_router: ToolRouter<PowerSupplyService>,
    /// Prompt router for MCP prompts
    prompt_router: PromptRouter<PowerSupplyService>,

    state: Arc<Mutex<PowerSupplyState>>,
}

impl PowerSupplyService {
    //--------------------------------------------------------------------------

    pub fn new(config: GlobalConfig, psu_name: String) -> Self {
        let client = PowerSupplyClientBuilder::from_broker_config(config.broker.clone())
            .with_power_supply_name(psu_name.clone())
            .build();
        debug!("Client initialized");

        Self {
            psu_name,
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
            let psu_state = self.state.lock().await;
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
            let psu_state = self.state.lock().await;
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

    /// Set the output voltage of the power supply
    #[tool(
        description = "Set the output voltage of the power supply. Takes voltage as a string, e.g., '5.0'"
    )]
    async fn set_voltage(
        &self,
        params: Parameters<VoltageParams>,
    ) -> Result<CallToolResult, McpError> {
        let voltage = &params.0.voltage;
        let client = {
            let psu_state = self.state.lock().await;
            psu_state.client.clone()
        };

        client.set_voltage(voltage.clone()).await.map_err(|_e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                "Failed to set power supply voltage",
                None,
            )
        })?;

        info!("Successfully set power supply voltage to {}", voltage);
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Power supply voltage set to {}",
            voltage
        ))]))
    }

    /// Set the output current limit of the power supply
    #[tool(
        description = "Set the output current limit of the power supply. Takes current as a string, e.g., '1.0'"
    )]
    async fn set_current(
        &self,
        params: Parameters<CurrentParams>,
    ) -> Result<CallToolResult, McpError> {
        let current = &params.0.current;
        let client = {
            let psu_state = self.state.lock().await;
            psu_state.client.clone()
        };

        client.set_current(current.clone()).await.map_err(|_e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                "Failed to set power supply current limit",
                None,
            )
        })?;

        info!("Successfully set power supply current limit to {}", current);
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Power supply current limit set to {}",
            current
        ))]))
    }
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
            instructions: Some(format!(
                r#"""This server provides access to a power supply.
The name of this power supply is "{}" and can be used by the user to request actions.
            """#,
                self.psu_name
            )),
        }
    }
}
