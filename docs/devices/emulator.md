# Emulator Device

The emulator is a virtual power supply device that simulates the behavior of a real power supply without requiring physical hardware. It's perfect for development, testing, and demonstrations.

## Overview

The emulator device provides:
- Full power supply functionality without hardware
- Instant startup and configuration
- Predictable, deterministic behavior
- No risk of hardware damage during development
- Same interface as physical devices

## Use Cases

### Development and Testing

- Develop and test control software without hardware
- Verify MQTT message flows
- Test MCP integrations
- Debug GUI features
- Validate automation scripts

### Demonstrations

- Show power supply control capabilities
- Train users on the interface
- Demo integrations without physical setup
- Presentations and tutorials

### Continuous Integration

- Automated testing in CI/CD pipelines
- No hardware dependencies
- Consistent, reproducible behavior
- Fast test execution

## Configuration

The emulator is configured like any other device in the configuration file:

```json
{
  "devices": {
    "emulator": {
      "model": "emulator",
      "description": "Virtual power supply for testing",
      "security_min_voltage": 0.0,
      "security_max_voltage": 30.0,
      "security_min_current": 0.0,
      "security_max_current": 5.0
    }
  }
}
```

### Configuration Parameters

- **model** (required): Must be `"emulator"`
- **description** (optional): Human-readable description
- **security_min_voltage** (optional): Minimum allowed voltage (default: 0.0V)
- **security_max_voltage** (optional): Maximum allowed voltage (default: 30.0V)
- **security_min_current** (optional): Minimum allowed current (default: 0.0A)
- **security_max_current** (optional): Maximum allowed current (default: 5.0A)

## Capabilities

### Voltage Control

- **Range**: 0V to 30V (or as configured)
- **Resolution**: Arbitrary precision (string-based)
- **Response**: Immediate

The emulator accepts any voltage command within the security limits and immediately reflects the new setting.

### Current Control

- **Range**: 0A to 5A (or as configured)
- **Resolution**: Arbitrary precision (string-based)
- **Response**: Immediate

The emulator accepts any current limit command within the security limits and immediately reflects the new setting.

### Output Enable/Disable

- **States**: ON (enabled) / OFF (disabled)
- **Response**: Immediate
- **Default**: OFF (disabled on startup)

The emulator tracks output state and responds to enable/disable commands instantly.

## Behavior

### Initialization

When the emulator starts:
1. Sets output to disabled (OFF)
2. Sets voltage to 0V
3. Sets current limit to 0A
4. Subscribes to MQTT command topics
5. Publishes initial state

### Command Processing

The emulator responds to commands immediately:
- No communication delays
- No hardware limitations
- Deterministic responses
- Instant state updates

### State Persistence

The emulator's state is volatile:
- Settings reset when the server restarts
- State is not saved between runs
- Each startup begins with default values

This is intentional for testing - you get a clean slate each time.

## Measurements

The emulator simulates voltage and current measurements:

### Voltage Measurement

The measured voltage equals the set voltage when output is enabled:
- **Output OFF**: Measured voltage is 0V
- **Output ON**: Measured voltage equals target voltage

### Current Measurement

The measured current simulates a resistive load:
- **Output OFF**: Measured current is 0A
- **Output ON**: Current is simulated based on voltage and a virtual load
- Maximum current respects the set current limit

## Differences from Physical Devices

Unlike real power supplies:

### No Communication Delays

- Physical devices have serial communication latency
- Emulator responds instantly
- Useful for testing, but may differ from real-world timing

### No Hardware Limitations

- No warm-up time
- No calibration needed
- No wear or drift
- No power supply limitations

### Perfect Behavior

- Always responds correctly
- Never loses connection
- No noise or interference
- No transient effects

### No Physical Constraints

- Can't damage hardware (there is none!)
- No overcurrent protection needed
- No thermal considerations
- No power consumption

## Testing Patterns

### Basic Functionality Test

```bash
# Set up the power supply
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "5.0"
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "1.0"

# Enable output
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "ON"

# Verify state (subscribe to status topics)
mosquitto_sub -h 127.0.0.1 -t "power-supply/emulator/#" -v
```

### Security Limit Test

```bash
# Try to exceed maximum voltage
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "50.0"

# Should be rejected and error published
mosquitto_sub -h 127.0.0.1 -t "power-supply/emulator/error" -v
```

### State Transition Test

```bash
# Test multiple state changes
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "ON"
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "OFF"
mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "ON"

# All should succeed immediately
```

## Example Configurations

### Minimal Configuration

```json
{
  "devices": {
    "emulator": {
      "model": "emulator"
    }
  }
}
```

Uses default security limits.

### Development Configuration

```json
{
  "devices": {
    "emulator": {
      "model": "emulator",
      "description": "Development test device",
      "security_min_voltage": 0.0,
      "security_max_voltage": 30.0,
      "security_min_current": 0.0,
      "security_max_current": 5.0
    }
  }
}
```

### Multiple Emulators

You can configure multiple emulator instances for testing multi-device scenarios:

```json
{
  "devices": {
    "emulator_1": {
      "model": "emulator",
      "description": "First test device"
    },
    "emulator_2": {
      "model": "emulator",
      "description": "Second test device"
    },
    "emulator_3": {
      "model": "emulator",
      "description": "Third test device"
    }
  }
}
```

Each emulator operates independently with its own MQTT topics and MCP endpoint.

### Restricted Configuration

For testing with lower limits:

```json
{
  "devices": {
    "emulator_3v3": {
      "model": "emulator",
      "description": "3.3V limited emulator",
      "security_min_voltage": 0.0,
      "security_max_voltage": 3.5,
      "security_min_current": 0.0,
      "security_max_current": 1.0
    }
  }
}
```

## Implementation Details

The emulator is implemented in Rust and provides:

- Async operations for consistency with real drivers
- Full implementation of the `PowerSupplyDriver` trait
- Security limit validation
- State management
- MQTT integration through the standard runner

Source code: `src/drivers/emulator.rs`

## Troubleshooting

### Emulator Not Starting

**Check configuration**:
- Ensure `model` is exactly `"emulator"` (case-sensitive)
- Verify JSON syntax is correct

**Check logs**:
```bash
cargo run --release 2>&1 | grep -i emulator
```

### Commands Not Working

**Verify device name**:
- MQTT topics must use the exact device name from configuration
- Default is "emulator" but can be any name you choose

**Check security limits**:
- Commands outside limits are rejected
- Review security configuration

### Unexpected Behavior

Remember the emulator is simplified:
- Instant response (no delays)
- Perfect operation (no errors)
- Stateless between restarts

If you need more realistic behavior, consider testing with a physical device.

## Migration to Physical Device

When moving from emulator to a physical device:

1. **Update the model**:
   ```json
   {
     "devices": {
       "lab_psu": {
         "model": "kd3005p",  // Changed from "emulator"
         "description": "Physical power supply",
         "security_min_voltage": 0.0,
         "security_max_voltage": 30.0,
         "security_min_current": 0.0,
         "security_max_current": 5.0
       }
     }
   }
   ```

2. **Update MQTT topics** if you changed the device name

3. **Update MCP endpoints** if using MCP

4. **Test with conservative limits** first

5. **Expect communication delays** that weren't present with the emulator

## See Also

- [KD3005P Device](kd3005p.md) - Physical power supply device
- [Configuration Guide](../getting-started/configuration.md) - Device configuration details
- [MQTT Interface](../interfaces/mqtt.md) - Control via MQTT
- [Testing](../testing.md) - Testing procedures
