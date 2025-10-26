# MQTT Interface

The MQTT interface provides a publish/subscribe mechanism for controlling and monitoring power supplies. This is ideal for automation, scripting, and integration with other systems.

## Overview

The Panduza Power Supply server includes an embedded MQTT broker and automatically publishes/subscribes to topics for each configured device. All topics follow the pattern:

```
power-supply/<device-name>/<category>/<attribute>[/cmd]
```

## MQTT Broker Configuration

The broker settings are configured in the server configuration file:

```json
{
  "broker": {
    "host": "127.0.0.1",
    "port": 1883
  }
}
```

- **host**: IP address to bind the MQTT broker (default: `127.0.0.1`)
- **port**: Port for the MQTT broker (default: `1883`)

## Topic Structure

### Control Topics

Control topics are used to send commands to the power supply.

#### Output Enable Control

**Topic**: `power-supply/<device-name>/control/oe/cmd`

Control the power output enable/disable state.

**Payload**:
- `"ON"` - Enable power output
- `"OFF"` - Disable power output

**Example**:
```bash
# Enable output
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "ON"

# Disable output
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "OFF"
```

#### Output Enable Status

**Topic**: `power-supply/<device-name>/control/oe`

Publishes the current output enable state.

**Payload**:
- `"true"` - Output is enabled
- `"false"` - Output is disabled

**Example**:
```bash
# Subscribe to output enable status
mosquitto_sub -h 127.0.0.1 -t "power-supply/emulator/control/oe"
```

#### Voltage Control

**Topic**: `power-supply/<device-name>/control/voltage/cmd`

Set the target voltage.

**Payload**: String representation of voltage in Volts (e.g., `"5.0"`, `"12.5"`)

**Example**:
```bash
# Set voltage to 5.0V
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "5.0"

# Set voltage to 12.5V
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "12.5"
```

#### Voltage Status

**Topic**: `power-supply/<device-name>/control/voltage`

Publishes the current voltage setting.

**Payload**: String representation of voltage in Volts

**Example**:
```bash
# Subscribe to voltage status
mosquitto_sub -h 127.0.0.1 -t "power-supply/emulator/control/voltage"
```

#### Current Control

**Topic**: `power-supply/<device-name>/control/current/cmd`

Set the current limit.

**Payload**: String representation of current in Amperes (e.g., `"1.0"`, `"2.5"`)

**Example**:
```bash
# Set current limit to 1.0A
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "1.0"

# Set current limit to 2.5A
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "2.5"
```

#### Current Status

**Topic**: `power-supply/<device-name>/control/current`

Publishes the current limit setting.

**Payload**: String representation of current in Amperes

**Example**:
```bash
# Subscribe to current status
mosquitto_sub -h 127.0.0.1 -t "power-supply/emulator/control/current"
```

### Measurement Topics

#### Voltage Measurement Refresh Rate

**Topic**: `power-supply/<device-name>/measure/voltage/refresh_freq`

Control the frequency of voltage measurements (if supported by the device).

**Payload**: Refresh frequency configuration

#### Current Measurement Refresh Rate

**Topic**: `power-supply/<device-name>/measure/current/refresh_freq`

Control the frequency of current measurements (if supported by the device).

**Payload**: Refresh frequency configuration

### Status Topics

#### General Status

**Topic**: `power-supply/<device-name>/status`

Publishes general status information about the power supply.

#### Error Messages

**Topic**: `power-supply/<device-name>/error`

Publishes error messages and alerts.

## Complete Example

Here's a complete example of controlling a power supply named "lab_psu":

```bash
# Subscribe to all topics from the device (in one terminal)
mosquitto_sub -h 127.0.0.1 -t "power-supply/lab_psu/#" -v

# In another terminal, send commands:

# Set voltage to 3.3V
mosquitto_pub -h 127.0.0.1 -t "power-supply/lab_psu/control/voltage/cmd" -m "3.3"

# Set current limit to 0.5A
mosquitto_pub -h 127.0.0.1 -t "power-supply/lab_psu/control/current/cmd" -m "0.5"

# Enable output
mosquitto_pub -h 127.0.0.1 -t "power-supply/lab_psu/control/oe/cmd" -m "ON"

# Wait a bit...

# Disable output
mosquitto_pub -h 127.0.0.1 -t "power-supply/lab_psu/control/oe/cmd" -m "OFF"
```

## Using with Programming Languages

### Python Example

Using the `paho-mqtt` library:

```python
import paho.mqtt.client as mqtt

# Callback when connected
def on_connect(client, userdata, flags, rc):
    print(f"Connected with result code {rc}")
    # Subscribe to all power supply topics
    client.subscribe("power-supply/emulator/#")

# Callback for received messages
def on_message(client, userdata, msg):
    print(f"{msg.topic}: {msg.payload.decode()}")

# Create client
client = mqtt.Client()
client.on_connect = on_connect
client.on_message = on_message

# Connect to broker
client.connect("127.0.0.1", 1883, 60)

# Set voltage
client.publish("power-supply/emulator/control/voltage/cmd", "5.0")

# Enable output
client.publish("power-supply/emulator/control/oe/cmd", "ON")

# Start loop
client.loop_forever()
```

### Node.js Example

Using the `mqtt` library:

```javascript
const mqtt = require('mqtt');

// Connect to broker
const client = mqtt.connect('mqtt://127.0.0.1:1883');

client.on('connect', () => {
    console.log('Connected to MQTT broker');
    
    // Subscribe to all topics
    client.subscribe('power-supply/emulator/#');
    
    // Set voltage
    client.publish('power-supply/emulator/control/voltage/cmd', '5.0');
    
    // Enable output
    client.publish('power-supply/emulator/control/oe/cmd', 'ON');
});

client.on('message', (topic, message) => {
    console.log(`${topic}: ${message.toString()}`);
});
```

## Security Limits

The server enforces security limits configured for each device. If you try to set a voltage or current beyond the configured limits, the command will be rejected and an error will be published to the error topic.

Example configuration with security limits:

```json
{
  "devices": {
    "lab_psu": {
      "model": "kd3005p",
      "security_min_voltage": 0.0,
      "security_max_voltage": 30.0,
      "security_min_current": 0.0,
      "security_max_current": 5.0
    }
  }
}
```

## Troubleshooting

### Cannot Connect to Broker

- Verify the broker is running (it starts automatically with the server)
- Check the host and port in your client configuration
- Ensure no firewall is blocking port 1883

### Commands Not Working

- Verify you're using the correct device name in the topic
- Check the payload format (must be exact: `"ON"`, `"OFF"`, or numeric strings)
- Subscribe to the error topic to see any error messages

### No Status Updates

- Ensure the server is running
- Check that the device is properly configured and initialized
- Subscribe to the status topic to verify messages are being published

## See Also

- [Configuration Guide](../getting-started/configuration.md) - Configure devices and broker settings
- [MCP Interface](mcp.md) - Alternative programmatic interface
- [GUI Interface](gui.md) - Visual control interface
