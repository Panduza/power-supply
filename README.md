# Panduza Power Supply

Panduza Power Supply provides multiple interfaces so you can control and monitor power supplies from different tools and environments:

- MQTT: send and receive commands and status updates via MQTT topics.
- MCP: control programmatically via the Model Context Protocol (MCP) for integrations and automation.
- Graphical interface: a desktop GUI for interactive use and visual feedback.

## Gettings Started

For detailed setup instructions, see the [Getting Started Configuration Guide](https://xdoctorwhoz.github.io/panduza-power-supply/#/getting-started/configuration).

## Server

`Build the server`

```bash
cd server
dx serve
```

## Acceptation Tests

For now acceptation tests are manual

- Start a server with an "emulator" instance

- MQTT: With a client tool
    - send "ON" in "power-supply/emulator/control/oe/cmd"
    - send "OFF" in "power-supply/emulator/control/oe/cmd"
    - send "0.5" in "power-supply/emulator/control/voltage/cmd"
    - send "5.23" in "power-supply/emulator/control/voltage/cmd"
    - send "5.23" in "power-supply/emulator/control/current/cmd"
    - send "5.23" in "power-supply/emulator/control/current/cmd"


- MCP: With copilot ()
    - prompt "turn on the power supply"
    - prompt "turn off the power supply"
    - prompt "configure power supply to 2.8V"
    - prompt "configure power supply to 3A"

```json
{
	"servers": {		
		"power_supply": {
			"url": "http://127.0.0.1:3000/power-supply/emulator",
			"type": "http"
		}
	},
	"inputs": []
}
```

- GUI:
    - Tun a gui without device configured = must show an error message
    - Select emulator (must be selected by default)
        - test on/off
        - test to set voltage
        - test to set current
    - Change ON/OFF from MQTT and check that gui show the change
    - Change voltage from MQTT and check that gui show the change
    - Change current from MQTT and check that gui show the change

