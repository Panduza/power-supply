# Server Configuration

The Panduza Power Supply server is configured through a JSON file located in your home directory.

## Configuration File Location

The configuration file is stored at:

- **Windows**: `C:\Users\<username>\.xdoctorwhoz\panduza-power-supply-server.json5`
- **Linux/Mac**: `~/.xdoctorwhoz/panduza-power-supply-server.json5`

## Automatic Configuration Generation

When you start the server for the first time, if no configuration file exists, a default configuration will be automatically generated at the location above.

## Configuration Structure

The configuration file is in JSON5 format (JSON with comments support) and contains the following sections:

### Complete Example

```json
{
  "gui": {
    "enable": true
  },
  "mcp": {
    "enable": false,
    "host": "127.0.0.1",
    "port": 50051
  },
  "broker": {
    "host": "127.0.0.1",
    "port": 1883
  },
  "devices": {
    "emulator": {
      "model": "emulator",
      "description": "Virtual power supply for testing",
      "security_min_voltage": 0.0,
      "security_max_voltage": 30.0,
      "security_min_current": 0.0,
      "security_max_current": 5.0
    },
    "lab_psu": {
      "model": "kd3005p",
      "description": "Laboratory bench power supply",
      "security_min_voltage": 0.0,
      "security_max_voltage": 30.0,
      "security_min_current": 0.0,
      "security_max_current": 5.0
    }
  }
}
```

## Configuration Sections

### GUI Configuration

Controls the graphical user interface.

```json
{
  "gui": {
    "enable": true  // Set to false to disable the GUI
  }
}
```

**Parameters:**
- `enable` (boolean): Enable or disable the GUI interface
  - Default: `true`

### MCP Configuration

Controls the Model Context Protocol server for programmatic access (e.g., GitHub Copilot integration).

```json
{
  "mcp": {
    "enable": false,
    "host": "127.0.0.1",
    "port": 50051
  }
}
```

**Parameters:**
- `enable` (boolean): Enable or disable the MCP server
  - Default: `false`
- `host` (string): IP address to bind the MCP server
  - Default: `"127.0.0.1"`
  - **Note**: Currently not used; server binds to `127.0.0.1`
- `port` (number): Port number for the MCP server
  - Default: `50051`
  - **Note**: Currently not used; server listens on port `3000`

?> **Current Implementation**: The MCP server currently runs on `http://127.0.0.1:3000` regardless of the configured values. The `host` and `port` settings are reserved for future use.

### MQTT Broker Configuration

Defines the MQTT broker connection settings.

```json
{
  "broker": {
    "host": "127.0.0.1",
    "port": 1883
  }
}
```

**Parameters:**
- `host` (string): MQTT broker hostname or IP address
  - Default: `"127.0.0.1"`
- `port` (number): MQTT broker port
  - Default: `1883`

### Devices Configuration

Configure your power supply devices. Each device is identified by a unique name (the key in the object).

```json
{
  "devices": {
    "device_name": {
      "model": "emulator",
      "description": "Optional description",
      "security_min_voltage": 0.0,
      "security_max_voltage": 30.0,
      "security_min_current": 0.0,
      "security_max_current": 5.0
    }
  }
}
```

**Parameters:**
- `model` (string, **required**): Type of power supply
  - Supported values: `"emulator"`, `"kd3005p"`
- `description` (string, optional): Human-readable description of the device
- `security_min_voltage` (number, optional): Minimum allowed voltage in Volts
- `security_max_voltage` (number, optional): Maximum allowed voltage in Volts
- `security_min_current` (number, optional): Minimum allowed current in Amperes
- `security_max_current` (number, optional): Maximum allowed current in Amperes

?> **Security Limits**: The security limits prevent accidental configuration of dangerous voltage or current levels. The server will reject any command that would exceed these limits.

## Supported Device Models

| Model | Description |
|-------|-------------|
| `emulator` | Virtual power supply for testing and development |
| `kd3005p` | Korad/RND KD3005P bench power supply |

## Configuration Examples

### Minimal Configuration

A minimal configuration with just MQTT and GUI enabled:

```json
{
  "gui": {
    "enable": true
  },
  "mcp": {
    "enable": false,
    "host": "127.0.0.1",
    "port": 50051
  },
  "broker": {
    "host": "127.0.0.1",
    "port": 1883
  }
}
```

### Development Setup

Configuration for development with emulator and all interfaces enabled:

```json
{
  "gui": {
    "enable": true
  },
  "mcp": {
    "enable": true,
    "host": "127.0.0.1",
    "port": 50051
  },
  "broker": {
    "host": "127.0.0.1",
    "port": 1883
  },
  "devices": {
    "emulator": {
      "model": "emulator",
      "description": "Development emulator",
      "security_min_voltage": 0.0,
      "security_max_voltage": 30.0,
      "security_min_current": 0.0,
      "security_max_current": 5.0
    }
  }
}
```

### Production Setup

Configuration for production with physical device and MQTT only:

```json
{
  "gui": {
    "enable": false
  },
  "mcp": {
    "enable": false,
    "host": "127.0.0.1",
    "port": 50051
  },
  "broker": {
    "host": "192.168.1.100",
    "port": 1883
  },
  "devices": {
    "lab_bench_1": {
      "model": "kd3005p",
      "description": "Main laboratory bench PSU",
      "security_min_voltage": 0.0,
      "security_max_voltage": 30.0,
      "security_min_current": 0.0,
      "security_max_current": 5.0
    }
  }
}
```

## Modifying the Configuration

1. Stop the server if it's running
2. Open the configuration file in your text editor:
   - Windows: `C:\Users\<username>\.xdoctorwhoz\panduza-power-supply-server.json5`
   - Linux/Mac: `~/.xdoctorwhoz/panduza-power-supply-server.json5`
3. Make your changes
4. Save the file
5. Restart the server

The server will validate the configuration on startup. If there are any errors, they will be displayed in the logs and the server may fall back to default values.

## Troubleshooting

### Configuration File Not Found

If the configuration file doesn't exist, it will be automatically created with default values when you start the server for the first time.

### Invalid JSON Format

If the configuration file contains syntax errors:
- Check for missing commas or brackets
- Ensure all strings are properly quoted
- Use a JSON validator to check your file
- The server will log the specific error and may generate a new default configuration

### Server Won't Start

Check that:
- The MQTT broker is running and accessible
- The specified ports are not already in use
- The device model names are correct
- All required fields are present

## Next Steps

- Learn how to use the [MQTT Interface](../interfaces/mqtt.md)
- Learn how to use the [MCP Interface](../interfaces/mcp.md)
- Learn how to use the [GUI Interface](../interfaces/gui.md)
