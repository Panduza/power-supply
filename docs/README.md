# Panduza Power Supply

> Control and monitor power supplies from different tools and environments

## Overview

Panduza Power Supply is a versatile application that provides multiple interfaces to control and monitor power supplies. Whether you need programmatic access, MQTT integration, or a graphical interface, this tool has you covered.

## Key Features

### 🔌 Multiple Interfaces

- **MQTT Interface**: Send and receive commands and status updates via MQTT topics
- **MCP (Model Context Protocol)**: Control programmatically for integrations and automation
- **Graphical User Interface**: Desktop GUI for interactive use and visual feedback

### ⚡ Power Supply Control

- Configure voltage and current settings
- Enable/disable power output
- Real-time monitoring and feedback
- Support for multiple power supply models

## Supported Devices

- **Emulator**: Virtual power supply for testing and development
- **KD3005P**: Korad/RND KD3005P power supply

## Quick Start

### Building and Running

```bash
# Clone the repository
git clone https://github.com/Panduza/power-supply.git
cd power-supply

# Build and run
cargo run --release
```

For detailed installation instructions, configuration options, and usage examples, please refer to the documentation sections in the sidebar.

## Architecture

The application is built in Rust with the following key components:

- **Drivers**: Device-specific implementations for controlling power supplies
- **MQTT Runner**: Manages MQTT communication and state synchronization
- **MCP Server**: Provides Model Context Protocol interface for integrations
- **GUI**: Native desktop interface built with Dioxus
- **Configuration**: JSON5-based configuration management

## Contributing

Contributions are welcome! This project is written in Rust and uses modern async patterns for reliable hardware communication.

## License

This project is licensed under the terms specified in the LICENSE file.
