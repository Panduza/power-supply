use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpConfig {
    /// Enable or disable the MCP server
    pub enable: bool,
    /// Bind address of the MCP server
    pub host: String,
    /// Port of the MCP server
    pub port: u16,
}
