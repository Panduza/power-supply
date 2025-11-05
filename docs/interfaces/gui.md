# GUI Interface

The Graphical User Interface (GUI) provides an intuitive visual way to control and monitor power supplies. Built with Dioxus, the GUI runs as a native desktop application with real-time updates.

## Overview

The GUI interface offers:
- Visual control of power supply settings
- Real-time display of voltage and current
- Device selection when multiple devices are configured
- Immediate feedback on commands and status
- Native desktop performance

## Enabling the GUI

The GUI is enabled by default. To control it, edit the configuration file:

```json
{
  "gui": {
    "enable": true
  }
}
```

- Set `enable` to `true` to show the GUI window
- Set `enable` to `false` to run in headless mode (MQTT/MCP only)

## GUI Components

### Device Selector

When multiple devices are configured, a dropdown selector appears at the top of the window.

**Features**:
- Lists all configured power supply devices
- Shows device names as configured in the settings
- Automatically selects the first device on startup
- Switching devices updates all controls and displays

**Usage**:
- Click the dropdown to see available devices
- Select a device to control it
- The entire GUI updates to reflect the selected device's state

### Power Button

The power button controls the output enable state.

**Visual States**:
- **Gray/Off**: Output is disabled
- **Active/On**: Output is enabled

**Usage**:
- Click to toggle output on/off
- Or press the configured keyboard shortcut (default: `p`)
- The button provides immediate visual feedback
- Changes are reflected in MQTT topics simultaneously

**Safety**: The button respects security limits - if a voltage or current setting is unsafe, the output may not enable.

### Voltage Setter

A slider control for setting the target output voltage.

**Features**:
- Adjustable slider for precise control
- Numeric display of current setting
- Real-time updates when changed via MQTT
- Respects configured min/max security limits

**Usage**:
1. Move the slider to the desired voltage
2. The value updates in real-time
3. The new setting is immediately sent to the device
4. The display shows the current voltage setting

**Range**: 
- Default: 0V to 30V
- Actual range depends on device capabilities and security limits
- Values outside security limits are prevented

### Current Setter

A slider control for setting the current limit.

**Features**:
- Adjustable slider for precise control
- Numeric display of current setting
- Real-time updates when changed via MQTT
- Respects configured min/max security limits

**Usage**:
1. Move the slider to the desired current limit
2. The value updates in real-time
3. The new setting is immediately sent to the device
4. The display shows the current limit setting

**Range**:
- Default: 0A to 5A
- Actual range depends on device capabilities and security limits
- Values outside security limits are prevented

### Configuration Button

Opens external configuration tools or files.

**Usage**:
- Click to access advanced configuration options
- May open the configuration file in your default editor
- Used for settings that don't change frequently

## Real-Time Synchronization

The GUI stays synchronized with the power supply state through MQTT:

### Incoming Updates

When changes are made via MQTT or MCP:
- The GUI automatically updates to reflect new values
- No manual refresh required
- Updates typically appear within milliseconds

**Example**: If you send an MQTT command to set voltage to 5V, the voltage slider in the GUI immediately moves to 5V.

### Outgoing Commands

When you interact with GUI controls:
- Commands are sent via MQTT to the power supply
- The change is reflected in the GUI immediately
- Other interfaces (MQTT subscribers, MCP clients) see the change

## Multi-Device Support

When multiple devices are configured, the GUI allows you to control each one:

```json
{
  "devices": {
    "emulator": {
      "model": "emulator",
      "description": "Development emulator"
    },
    "lab_bench": {
      "model": "kd3005p",
      "description": "Main lab power supply"
    }
  }
}
```

The device selector will show both devices. Switching between them:
- Saves the current device's settings
- Loads and displays the selected device's state
- Reconnects GUI controls to the new device's MQTT topics

## Typical Workflow

### Basic Power Supply Control

1. **Start the application**
   - The GUI window opens automatically
   - Default device is selected

2. **Set desired voltage**
   - Use the voltage slider
   - Example: Move to 5.0V

3. **Set current limit**
   - Use the current slider
   - Example: Set to 1.0A for safety

4. **Enable output**
   - Click the power button
   - Button changes to active state

5. **Monitor operation**
   - GUI shows real-time values
   - Make adjustments as needed

6. **Disable output**
   - Click power button again
   - Output turns off safely

### Testing a Circuit

1. **Set current limit to safe value** (e.g., 0.5A)
2. **Set voltage to desired value** (e.g., 3.3V)
3. **Enable output**
4. **Monitor current draw**
5. **Adjust voltage if needed**
6. **Disable output when done**

## Error Handling

### Device Not Found

If no devices are configured:
- The GUI shows an error message
- Prompts you to configure at least one device
- May show instructions for editing the configuration file

**Solution**: Edit the configuration file to add at least one device.

### Connection Lost

If the MQTT connection is lost:
- The GUI may show a warning indicator
- Controls may become disabled
- Automatic reconnection is attempted

**Solution**: Check that the MQTT broker is running and restart if needed.

### Security Limit Violation

If you try to set values beyond security limits:
- The slider snaps back to the maximum allowed value
- An error may be displayed
- The command is not sent to the device

**Solution**: Adjust security limits in the configuration if appropriate.

## Platform-Specific Notes

### Linux

- The GUI uses native GTK components
- Requires GTK development libraries (see [Installation](../getting-started/installation.md))
- Window manager integration works out of the box

### macOS

- Native macOS look and feel
- Integrates with system menus
- May request permissions for USB device access

### Windows

- Native Windows controls
- Uses system theme
- USB drivers may be required for physical devices

## Performance

The GUI is designed for efficiency:
- Low CPU usage when idle
- Minimal memory footprint (~50MB)
- Responsive even with frequent updates
- Native performance (not browser-based)

## Keyboard Shortcuts

The GUI supports keyboard shortcuts for quick control:

### Power Toggle Shortcut

**Feature**: Press a configurable key to toggle the power output on/off without using the mouse.

**Configuration**:
```json
{
  "gui": {
    "enable": true,
    "power_toggle_key": "p"
  }
}
```

**Usage**:
1. Ensure the GUI window is focused (click anywhere in the window)
2. Press the configured key (default: `p`)
3. The power output will toggle between enabled and disabled
4. The same visual feedback as clicking the power button is shown

**Configuration Options**:
- Single character keys: `"p"`, `"t"`, `"o"`, etc.
- Special keys: `"space"`, `"enter"`, etc.
- Case-insensitive: `"P"` and `"p"` are equivalent
- Default: `"p"` (for "power")

**Tips**:
- Choose a key that doesn't conflict with system shortcuts
- Use an intuitive key like `"p"` for power or `"t"` for toggle
- The shortcut only works when the GUI window has focus
- To disable keyboard shortcuts, set `power_toggle_key` to `null` or remove it from the configuration

### Future Shortcuts

Future versions may include additional keyboard shortcuts for:
- Voltage adjustment
- Current adjustment
- Device switching
- Configuration access

## Accessibility

The GUI aims to be accessible:
- Clear visual indicators
- Logical tab order
- High contrast elements
- Future versions may add screen reader support

## Troubleshooting

### GUI Window Doesn't Open

**Check configuration**:
```json
{
  "gui": {
    "enable": true
  }
}
```

**Check dependencies** (Linux):
- Ensure GTK libraries are installed
- See [Installation Guide](../getting-started/installation.md)

**Check logs**:
```bash
cargo run --release 2>&1 | grep -i gui
```

### Controls Not Responding

**Check MQTT connection**:
- The GUI communicates via MQTT
- Ensure the broker is running
- Check network connectivity

**Check device configuration**:
- Ensure device is properly configured
- Verify device name matches configuration

### Values Not Updating

**Check for security limit violations**:
- Values may be capped at security limits
- Check configuration for min/max values

**Restart the application**:
- Sometimes a fresh start resolves state issues

### Display Issues

**Try resizing the window**:
- Some display issues resolve with window resize

**Check system theme**:
- Some system themes may cause visual glitches
- Try switching to default theme

## Customization

While the GUI doesn't currently support extensive customization, future versions may include:
- Theme selection
- Layout options
- Custom widgets for specific use cases
- Keyboard shortcut configuration

## See Also

- [Configuration Guide](../getting-started/configuration.md) - Configure GUI settings
- [MQTT Interface](mqtt.md) - Understanding the underlying MQTT communication
- [Quick Start](../getting-started/quickstart.md) - Get started with the GUI
