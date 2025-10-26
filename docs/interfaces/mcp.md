# MCP Interface

The Model Context Protocol (MCP) interface enables programmatic control of power supplies through a standardized protocol, perfect for integrations with AI assistants like GitHub Copilot, custom automation tools, and other MCP-compatible clients.

## Overview

MCP provides a structured way to interact with power supplies through well-defined tools and operations. Each power supply device gets its own MCP endpoint, allowing fine-grained control in multi-device setups.

## Configuration

To enable the MCP interface, update your configuration file:

```json
{
  "mcp": {
    "enable": true,
    "host": "127.0.0.1",
    "port": 3000
  }
}
```

**Configuration Parameters:**
- `enable` (boolean): Enable or disable the MCP server (default: `false`)
- `host` (string): IP address to bind the server (default: `"127.0.0.1"`)
- `port` (number): Port number for the MCP server (default: `3000`)

After enabling, restart the server for changes to take effect.

## Endpoint Structure

Each configured device gets its own MCP endpoint:

```
http://<host>:<port>/power-supply/<device-name>
```

For example, with a device named "emulator":
```
http://127.0.0.1:3000/power-supply/emulator
```

## Available Tools

The MCP interface provides the following tools for controlling power supplies:

### output_enable

Enable the power supply output (turn on power).

**Description**: "Enable the power supply output (turn on power)"

**Parameters**: None

**Returns**: Success message confirming output was enabled

**Example Use**:
- "Turn on the power supply"
- "Enable output"
- "Power on"

### output_disable

Disable the power supply output (turn off power).

**Description**: "Disable the power supply output (turn off power)"

**Parameters**: None

**Returns**: Success message confirming output was disabled

**Example Use**:
- "Turn off the power supply"
- "Disable output"
- "Power off"

### set_voltage

Set the output voltage of the power supply.

**Description**: "Set the output voltage of the power supply. Takes voltage as a string, e.g., '5.0'"

**Parameters**:
- `voltage` (string, required): The target voltage in Volts (e.g., "5.0", "12.5")

**Returns**: Success message with the voltage value

**Example Use**:
- "Set voltage to 5V"
- "Configure power supply to 3.3V"
- "Change voltage to 12 volts"

**Security**: The server will reject voltage values outside the configured security limits.

### set_current

Set the output current limit of the power supply.

**Description**: "Set the output current limit of the power supply. Takes current as a string, e.g., '1.0'"

**Parameters**:
- `current` (string, required): The target current limit in Amperes (e.g., "1.0", "2.5")

**Returns**: Success message with the current limit value

**Example Use**:
- "Set current limit to 1A"
- "Configure power supply to 2.5A"
- "Limit current to 0.5 amperes"

**Security**: The server will reject current values outside the configured security limits.

## Using with GitHub Copilot

To use the MCP interface with GitHub Copilot:

1. **Enable MCP in the configuration**:

```json
{
  "mcp": {
    "enable": true,
    "host": "127.0.0.1",
    "port": 3000
  }
}
```

2. **Configure Copilot** to use the MCP server by adding to your MCP settings (typically in `.copilot/mcp.json` or similar):

```json
{
  "servers": {
    "power_supply": {
      "url": "http://127.0.0.1:3000/power-supply/emulator",
      "type": "http"
    }
  }
}
```

3. **Use natural language** to control the power supply through Copilot:

```
You: "Turn on the power supply"
Copilot: [Uses output_enable tool] "Power supply output enabled"

You: "Set voltage to 3.3V"
Copilot: [Uses set_voltage tool with voltage="3.3"] "Power supply voltage set to 3.3"

You: "Set current limit to 0.5A and turn on"
Copilot: [Uses set_current and output_enable tools] "Current limit set to 0.5A. Output enabled."
```

## Multiple Devices

When you have multiple devices configured, each gets its own endpoint:

```json
{
  "devices": {
    "emulator": {
      "model": "emulator"
    },
    "lab_bench_1": {
      "model": "kd3005p"
    },
    "lab_bench_2": {
      "model": "kd3005p"
    }
  }
}
```

Configure each device as a separate server in your MCP client:

```json
{
  "servers": {
    "emulator": {
      "url": "http://127.0.0.1:3000/power-supply/emulator",
      "type": "http"
    },
    "lab_bench_1": {
      "url": "http://127.0.0.1:3000/power-supply/lab_bench_1",
      "type": "http"
    },
    "lab_bench_2": {
      "url": "http://127.0.0.1:3000/power-supply/lab_bench_2",
      "type": "http"
    }
  }
}
```

## Server Information

Each MCP endpoint provides server information including:
- Protocol version (currently v2024-11-05)
- Available capabilities (tools and prompts)
- Server implementation details
- Instructions for using the specific power supply

You can query this information using standard MCP protocol methods.

## Error Handling

The MCP interface returns structured errors when operations fail:

**Security Limit Exceeded**:
```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "message": "Failed to set power supply voltage"
  }
}
```

This typically occurs when:
- Voltage exceeds configured max/min limits
- Current exceeds configured max/min limits
- Device communication fails

**Device Not Available**:
If a device is not properly initialized or becomes disconnected, operations will fail with appropriate error messages.

## Communication Flow

1. Client connects to MCP endpoint
2. Client discovers available tools via MCP protocol
3. Client invokes tools with parameters
4. Server validates parameters (including security limits)
5. Server sends MQTT commands to the power supply
6. Power supply confirms the operation
7. Server returns success/error to client

## Performance Considerations

- MCP commands are asynchronous and typically complete in milliseconds
- The server handles multiple concurrent MCP connections
- Each tool invocation is independent and stateless
- Security limit validation happens before hardware commands are sent

## Example Automation Script

Here's an example of how you might use the MCP interface programmatically (pseudo-code):

```python
# Connect to MCP server
mcp_client = MCPClient("http://127.0.0.1:3000/power-supply/emulator")

# Power on sequence for testing a 3.3V device
async def power_on_test_device():
    # Set safe current limit first
    await mcp_client.call_tool("set_current", {"current": "0.5"})
    
    # Set target voltage
    await mcp_client.call_tool("set_voltage", {"voltage": "3.3"})
    
    # Enable output
    await mcp_client.call_tool("output_enable")
    
    print("Device powered on at 3.3V with 0.5A current limit")

# Run test
await power_on_test_device()

# Wait for test to complete...

# Power off sequence
async def power_off():
    await mcp_client.call_tool("output_disable")
    print("Device powered off")

await power_off()
```

## Security

### Transport Security

By default, the MCP server runs on localhost (`127.0.0.1`) and is not accessible from the network. If you need to expose it:

1. Change the host to `0.0.0.0` in the configuration
2. Consider using a reverse proxy with HTTPS
3. Implement authentication at the proxy level
4. Use firewall rules to restrict access

### Parameter Validation

All voltage and current values are validated against configured security limits before being sent to the device. This prevents:
- Overvoltage conditions
- Excessive current draw
- Out-of-range configurations

### CORS

The MCP server includes CORS (Cross-Origin Resource Sharing) headers for browser-based clients. This is configured to be permissive by default for localhost development.

## Troubleshooting

### MCP Server Not Starting

**Check the configuration**:
- Ensure `mcp.enable` is set to `true`
- Verify the port is not already in use

**Check the logs**:
```bash
# Look for MCP-related log messages
cargo run --release 2>&1 | grep -i mcp
```

You should see:
```
MCP server listening on 127.0.0.1:3000/power-supply/emulator
```

### Tools Not Working

**Verify the endpoint**:
- Ensure you're connecting to the correct URL with the device name
- Check that the device is properly configured and initialized

**Check security limits**:
- Voltage/current commands may fail if they exceed configured limits
- Review the error messages returned by the tool calls

### Connection Refused

**Firewall blocking the port**:
- Check firewall settings for port 3000
- Verify the server is actually listening: `netstat -an | grep 3000`

**Wrong host/port**:
- Verify the configuration matches your client settings
- Default is `127.0.0.1:3000`

## See Also

- [Configuration Guide](../getting-started/configuration.md) - Enable and configure MCP
- [MQTT Interface](mqtt.md) - Alternative control interface
- [GUI Interface](gui.md) - Visual control interface
- [MCP Protocol Specification](https://spec.modelcontextprotocol.io/) - Official MCP documentation
