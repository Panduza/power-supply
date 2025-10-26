# Quick Start

This guide will help you get up and running with Panduza Power Supply in just a few minutes.

## Prerequisites

- Rust and Cargo installed (latest stable version recommended)
- MQTT broker (optional, one will be started automatically)
- For physical devices: appropriate USB drivers and permissions

## Installation

1. Clone the repository:

```bash
git clone https://github.com/Panduza/power-supply.git
cd power-supply
```

2. Build the project:

```bash
cargo build --release
```

## First Run

1. Start the application:

```bash
cargo run --release
```

On first run, the application will:
- Create a default configuration file at `~/.xdoctorwhoz/panduza-power-supply-server.json5`
- Start an embedded MQTT broker
- Launch the graphical interface (if enabled)
- Initialize an emulator device by default

2. You should see the GUI window open with the emulator device available.

## Quick Test

### Using the GUI

1. The emulator device should be selected by default
2. Click the power button to enable output
3. Adjust the voltage slider to set desired voltage (0-30V)
4. Adjust the current slider to set desired current limit (0-5A)
5. The GUI will show real-time values

### Using MQTT

You can also control the power supply via MQTT. Using any MQTT client (like mosquitto_pub):

```bash
# Enable output
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "ON"

# Set voltage to 5V
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "5.0"

# Set current limit to 1A
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "1.0"

# Disable output
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "OFF"
```

### Using MCP (Model Context Protocol)

To enable MCP support for integrations like GitHub Copilot:

1. Edit the configuration file to enable MCP:

```json
{
  "mcp": {
    "enable": true,
    "host": "127.0.0.1",
    "port": 3000
  }
}
```

2. Restart the application

3. Configure your MCP client to connect to `http://127.0.0.1:3000/power-supply/emulator`

## Next Steps

- [Configure your setup](configuration.md) - Customize settings and add devices
- [MQTT Interface](../interfaces/mqtt.md) - Learn about MQTT topics and commands
- [MCP Interface](../interfaces/mcp.md) - Use MCP for programmatic control
- [Supported Devices](../devices/emulator.md) - Learn about supported hardware

## Troubleshooting

### GUI doesn't start

Check the configuration file and ensure `gui.enable` is set to `true`.

### MQTT connection failed

The embedded broker starts on port 1883 by default. Ensure this port is available or change it in the configuration.

### Device not found

For physical devices, ensure:
- Device is connected via USB
- You have the necessary permissions (may need to add udev rules on Linux)
- The correct device model is specified in the configuration
