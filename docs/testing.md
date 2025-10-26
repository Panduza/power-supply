# Testing

This document describes testing procedures for the Panduza Power Supply application.

## Overview

Testing ensures that:
- All interfaces work correctly
- Devices respond as expected
- Security limits are enforced
- State synchronization works properly
- Error handling is robust

## Test Environment Setup

### Prerequisites

1. **Running Server**: Ensure the server is running
   ```bash
   cargo run --release
   ```

2. **MQTT Client**: Install mosquitto clients
   ```bash
   # Ubuntu/Debian
   sudo apt-get install mosquitto-clients
   
   # macOS
   brew install mosquitto
   ```

3. **Configured Devices**: At least one device configured (emulator recommended for testing)

## Manual Testing Procedures

### Testing with Emulator

The emulator provides a safe, hardware-free testing environment.

#### Basic Functionality Test

1. **Configure emulator** in the configuration file:
   ```json
   {
     "devices": {
       "emulator": {
         "model": "emulator",
         "security_min_voltage": 0.0,
         "security_max_voltage": 30.0,
         "security_min_current": 0.0,
         "security_max_current": 5.0
       }
     }
   }
   ```

2. **Start the server**:
   ```bash
   cargo run --release
   ```

3. **Test MQTT interface**:

   **Enable output**:
   ```bash
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "ON"
   ```

   **Disable output**:
   ```bash
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "OFF"
   ```

   **Set voltage**:
   ```bash
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "5.0"
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "12.5"
   ```

   **Set current**:
   ```bash
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "1.0"
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "2.5"
   ```

4. **Verify state** by subscribing to status topics:
   ```bash
   mosquitto_sub -h 127.0.0.1 -t "power-supply/emulator/#" -v
   ```

   You should see:
   - Output enable state changes
   - Voltage updates
   - Current updates

#### Security Limit Testing

1. **Test voltage limits**:
   ```bash
   # Try to exceed maximum voltage
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "50.0"
   
   # Try negative voltage
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "-5.0"
   ```

   Expected: Commands should be rejected, errors published to error topic

2. **Test current limits**:
   ```bash
   # Try to exceed maximum current
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "10.0"
   
   # Try negative current
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "-1.0"
   ```

   Expected: Commands should be rejected, errors published to error topic

3. **Monitor errors**:
   ```bash
   mosquitto_sub -h 127.0.0.1 -t "power-supply/emulator/error" -v
   ```

### Testing MCP Interface

Prerequisites: MCP enabled in configuration

```json
{
  "mcp": {
    "enable": true,
    "host": "127.0.0.1",
    "port": 3000
  }
}
```

#### With GitHub Copilot

1. **Configure Copilot** with the MCP endpoint:
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

2. **Test commands** using natural language:
   - "Turn on the power supply"
   - "Turn off the power supply"
   - "Configure power supply to 2.8V"
   - "Set current limit to 3A"
   - "Set voltage to 5V and current to 1A then turn on"

3. **Verify responses**:
   - Commands should execute successfully
   - Copilot should confirm actions
   - Check MQTT topics to verify changes

#### With MCP Client

If you have a direct MCP client, test the tools:

- `output_enable` - Should enable output
- `output_disable` - Should disable output
- `set_voltage` - Should set voltage (e.g., `{"voltage": "5.0"}`)
- `set_current` - Should set current limit (e.g., `{"current": "1.0"}`)

### Testing GUI Interface

1. **Start with GUI enabled** (default):
   ```json
   {
     "gui": {
       "enable": true
     }
   }
   ```

2. **Test with no devices** configured:
   - Expected: Error message indicating no devices configured
   - GUI should not crash

3. **Test with emulator**:
   
   **Device Selection**:
   - If multiple devices configured, dropdown should appear
   - Selecting a device should update all controls
   - Default device should be pre-selected

   **Power Button**:
   - Click to toggle ON/OFF
   - Visual state should change
   - MQTT topics should reflect change

   **Voltage Slider**:
   - Move slider to set voltage
   - Value should update in real-time
   - MQTT should publish new value

   **Current Slider**:
   - Move slider to set current
   - Value should update in real-time
   - MQTT should publish new value

4. **Test external changes**:
   
   **Change voltage via MQTT**:
   ```bash
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "7.5"
   ```
   
   Expected: GUI voltage slider moves to 7.5V

   **Change current via MQTT**:
   ```bash
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/current/cmd" -m "1.5"
   ```
   
   Expected: GUI current slider moves to 1.5A

   **Change output state via MQTT**:
   ```bash
   mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/oe/cmd" -m "ON"
   ```
   
   Expected: GUI power button shows ON state

### Testing with Physical Device (KD3005P)

**CAUTION**: Use conservative settings when testing with physical devices.

1. **Configure KD3005P**:
   ```json
   {
     "devices": {
       "test_psu": {
         "model": "kd3005p",
         "security_min_voltage": 0.0,
         "security_max_voltage": 5.0,
         "security_min_current": 0.0,
         "security_max_current": 1.0
       }
     }
   }
   ```

2. **Connect device** via USB

3. **Start server** and check logs:
   ```bash
   cargo run --release 2>&1 | grep -i kd3005p
   ```
   
   Should see: Device detected and initialized

4. **Test basic operations** (same as emulator tests above)

5. **Test with actual load**:
   - Connect a known load (e.g., resistor)
   - Set voltage
   - Enable output
   - Verify current reading matches expected value
   - Disable output

6. **Test current limiting**:
   - Set low current limit (e.g., 0.1A)
   - Connect a load that would draw more
   - Enable output
   - Verify current is limited
   - Verify device enters CC mode

## Automated Testing

Currently, automated tests are limited. Future versions may include:
- Unit tests for driver implementations
- Integration tests for MQTT flows
- End-to-end tests with emulator
- CI/CD pipeline tests

### Running Existing Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Test Checklist

Use this checklist for comprehensive testing:

### MQTT Interface

- [ ] Enable output via MQTT
- [ ] Disable output via MQTT
- [ ] Set voltage via MQTT
- [ ] Set current via MQTT
- [ ] Subscribe to status topics
- [ ] Verify state updates
- [ ] Test security limit violations
- [ ] Check error messages

### MCP Interface

- [ ] MCP server starts correctly
- [ ] Tools are discoverable
- [ ] output_enable works
- [ ] output_disable works
- [ ] set_voltage works
- [ ] set_current works
- [ ] Security limits enforced
- [ ] Multiple devices accessible

### GUI Interface

- [ ] GUI starts without devices (error shown)
- [ ] GUI starts with emulator
- [ ] Device selector works (if multiple devices)
- [ ] Power button toggles output
- [ ] Voltage slider sets voltage
- [ ] Current slider sets current
- [ ] External MQTT changes update GUI
- [ ] GUI changes publish to MQTT

### Emulator Device

- [ ] Emulator initializes
- [ ] Accepts voltage commands
- [ ] Accepts current commands
- [ ] Output enable/disable works
- [ ] Security limits enforced
- [ ] State persists during session
- [ ] State resets on restart

### KD3005P Device

- [ ] Device detected on USB
- [ ] Device initializes
- [ ] Voltage control works
- [ ] Current control works
- [ ] Output enable/disable works
- [ ] Measurements are accurate
- [ ] Security limits enforced
- [ ] Error handling works
- [ ] Reconnection works after disconnect

## Regression Testing

When making changes, verify:

1. **Backward Compatibility**:
   - Existing configurations still work
   - MQTT topics unchanged
   - MCP tools unchanged

2. **No Breaking Changes**:
   - All interfaces still functional
   - Security limits still enforced
   - Error handling still works

3. **Performance**:
   - No significant slowdowns
   - Memory usage reasonable
   - Responsive UI

## Reporting Issues

When reporting test failures, include:

1. **Configuration file** (sanitized if needed)
2. **Steps to reproduce**
3. **Expected behavior**
4. **Actual behavior**
5. **Logs** (relevant portions)
6. **Environment**: OS, Rust version, device type
7. **Version**: Git commit hash or release version

## See Also

- [Contributing](contributing.md) - How to contribute test improvements
- [Quick Start](getting-started/quickstart.md) - Basic setup
- [MQTT Interface](interfaces/mqtt.md) - MQTT testing details
- [MCP Interface](interfaces/mcp.md) - MCP testing details
