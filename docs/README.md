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

### Building the Server

```bash
cd server
dx serve
```

For detailed installation instructions, configuration options, and usage examples, please refer to the documentation sections in the sidebar.

## Architecture

The project is organized into two main components:

- **Server**: The core application that manages power supply connections and provides interfaces (MQTT, MCP, GUI)
- **Client**: Libraries and examples for interacting with the server

## Contributing

Contributions are welcome! This project is written in Rust and uses modern async patterns for reliable hardware communication.

## License

This project is licensed under the terms specified in the LICENSE file.
