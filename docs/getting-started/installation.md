# Installation

This guide covers the installation process for Panduza Power Supply on different platforms.

## System Requirements

- **Operating System**: Linux, macOS, or Windows
- **Rust**: Latest stable version (1.70.0 or newer)
- **Memory**: 100 MB RAM minimum
- **Disk Space**: 50 MB for binaries and configuration

## Prerequisites

### Install Rust

If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/):

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run [rustup-init.exe](https://win.rustup.rs/)

After installation, verify:
```bash
rustc --version
cargo --version
```

### Platform-Specific Requirements

#### Linux

For GUI support, you may need additional dependencies:

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Fedora:**
```bash
sudo dnf install webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

For USB device access (physical power supplies):
```bash
# Add your user to the dialout group
sudo usermod -a -G dialout $USER
# Log out and log back in for the change to take effect
```

#### macOS

No additional dependencies are typically required. For USB device access, you may need to grant terminal permissions.

#### Windows

For USB device access with physical devices, you may need to install the appropriate USB drivers for your power supply model.

## Installation Methods

### From Source (Recommended)

1. **Clone the repository:**

```bash
git clone https://github.com/Panduza/power-supply.git
cd power-supply
```

2. **Build the project:**

```bash
cargo build --release
```

The compiled binary will be located at `target/release/pza-power-supply` (or `pza-power-supply.exe` on Windows).

3. **Run the application:**

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/pza-power-supply
```

### Development Build

For development, you can use the debug build which compiles faster but runs slower:

```bash
cargo build
cargo run
```

## Post-Installation

### Configuration

On first run, a default configuration file will be created at:

- **Linux/macOS**: `~/.xdoctorwhoz/panduza-power-supply-server.json5`
- **Windows**: `C:\Users\<username>\.xdoctorwhoz\panduza-power-supply-server.json5`

See the [Configuration Guide](configuration.md) for details on customizing your setup.

### Verifying Installation

1. **Check that the application starts:**

```bash
cargo run --release
```

You should see:
- Log messages indicating the server is starting
- The GUI window opening (if GUI is enabled)
- No error messages about missing dependencies

2. **Test MQTT connectivity:**

```bash
# In another terminal, subscribe to power supply topics
mosquitto_sub -h 127.0.0.1 -t "power-supply/#" -v
```

You should see status messages from the power supply.

## Optional Components

### MQTT Client Tools

For testing and debugging MQTT, install mosquitto clients:

**Ubuntu/Debian:**
```bash
sudo apt-get install mosquitto-clients
```

**macOS (with Homebrew):**
```bash
brew install mosquitto
```

**Windows:**
Download from [Eclipse Mosquitto](https://mosquitto.org/download/)

## Updating

To update to the latest version:

```bash
cd power-supply
git pull
cargo build --release
```

## Uninstallation

To remove the application:

1. Delete the cloned repository
2. Remove the configuration directory:
   - Linux/macOS: `rm -rf ~/.xdoctorwhoz`
   - Windows: Delete `C:\Users\<username>\.xdoctorwhoz`

## Troubleshooting

### Build Fails

**Error: linking with `cc` failed**
- Ensure you have a C compiler installed (gcc on Linux, clang on macOS, MSVC on Windows)

**Error: failed to load source for dependency**
- Check your internet connection
- Try: `cargo clean && cargo build --release`

### Permission Denied (Linux)

If you get "Permission denied" errors accessing USB devices:
```bash
sudo usermod -a -G dialout $USER
# Log out and log back in
```

### GUI Doesn't Start

Check that you have the required GUI dependencies installed (see platform-specific requirements above).

## Next Steps

- [Quick Start Guide](quickstart.md) - Get started quickly
- [Configuration](configuration.md) - Customize your setup
- [Supported Devices](../devices/emulator.md) - Learn about device support
