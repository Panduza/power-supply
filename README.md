<div align="center">
   
# Panduza Power Supply

Control and monitor power supplies from **MCP** or **MQTT**

</div>

![ddddddd](https://github.com/user-attachments/assets/a09d2ed7-7e3b-410e-b71e-b9d379750661)


## Features

- Multiple interface support (MQTT, MCP, GUI)
- Control voltage and current settings
- Enable/disable power output
- Real-time monitoring and feedback
- Support for multiple power supply models
- Security limits to prevent unsafe configurations

## Supported Devices

- **Emulator**: Virtual power supply for testing and development
- **KD3005P**: Korad/RND KD3005P laboratory bench power supply (0-30V, 0-5A)

## Quick Start

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Panduza/power-supply.git
   cd power-supply
   ```

2. **Build and run**:
   ```bash
   cargo run --release
   ```

3. **Use the GUI** or control via MQTT/MCP (see documentation for details)

## Documentation

Full documentation is available at: https://panduza.github.io/power-supply/

Key sections:
- [Quick Start Guide](https://panduza.github.io/power-supply/#/getting-started/quickstart)
- [Installation](https://panduza.github.io/power-supply/#/getting-started/installation)
- [Configuration](https://panduza.github.io/power-supply/#/getting-started/configuration)
- [MQTT Interface](https://panduza.github.io/power-supply/#/interfaces/mqtt)
- [MCP Interface](https://panduza.github.io/power-supply/#/interfaces/mcp)
- [GUI Interface](https://panduza.github.io/power-supply/#/interfaces/gui)
- [Testing](https://panduza.github.io/power-supply/#/testing)
- [Contributing](https://panduza.github.io/power-supply/#/contributing)

## Example Usage

### MQTT Control

```bash
# Enable output
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "ON"

# Set voltage to 5V
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "5.0"

# Set current limit to 1A
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "1.0"
```

### MCP Control (with GitHub Copilot)

Configure Copilot to use the MCP endpoint:

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

Then use natural language commands:
- "Turn on the power supply"
- "Set voltage to 3.3V"
- "Configure power supply to 2.8V and 500mA"

## Configuration

The server is configured via a JSON file at `~/.panduza/pza-power-supply-server.json5`.

A default configuration is automatically generated on first run. See the [Configuration Guide](https://panduza.github.io/power-supply/#/getting-started/configuration) for details.

## Contributing

Contributions are welcome! Please see the [Contributing Guide](https://panduza.github.io/power-supply/#/contributing) for details.


```bash
./flatc.exe --version
# flatc version 25.2.10
```

```bash
# To rebuild flatbuffers
./flatc.exe --rust -o src/payload/ payloads.fbs
```

## License

This project is licensed under the terms specified in the LICENSE file.

