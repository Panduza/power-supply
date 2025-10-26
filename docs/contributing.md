# Contributing

Thank you for considering contributing to Panduza Power Supply! This document provides guidelines for contributing to the project.

## Code of Conduct

Be respectful, professional, and constructive in all interactions. We aim to maintain a welcoming and inclusive community.

## How to Contribute

### Reporting Bugs

Before creating a bug report:

1. **Check existing issues** to avoid duplicates
2. **Test with the latest version** to ensure the bug still exists
3. **Use the emulator** to isolate the issue when possible

When reporting a bug, include:

- **Clear title**: Brief description of the issue
- **Environment**: OS, Rust version, device type
- **Configuration**: Relevant parts of your config file (sanitize sensitive data)
- **Steps to reproduce**: Detailed, step-by-step instructions
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Logs**: Relevant log output
- **Screenshots**: If GUI-related

Example:
```
Title: GUI doesn't update when voltage changed via MQTT

Environment:
- OS: Ubuntu 22.04
- Rust: 1.75.0
- Device: Emulator

Configuration:
{
  "gui": {"enable": true},
  "devices": {
    "emulator": {"model": "emulator"}
  }
}

Steps to Reproduce:
1. Start the application
2. Send MQTT command: mosquitto_pub -h 127.0.0.1 -t "power-supply/emulator/control/voltage/cmd" -m "5.0"
3. Observe GUI voltage slider

Expected: Slider moves to 5.0V
Actual: Slider doesn't move
```

### Suggesting Enhancements

Enhancement suggestions are welcome! Include:

- **Use case**: Why is this enhancement needed?
- **Proposed solution**: How should it work?
- **Alternatives considered**: Other approaches you've thought about
- **Impact**: Who benefits and how?

### Pull Requests

1. **Fork the repository**

2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**:
   - Follow the coding style (see below)
   - Add tests if applicable
   - Update documentation as needed

4. **Test your changes**:
   ```bash
   cargo test
   cargo build --release
   # Manual testing as appropriate
   ```

5. **Commit with clear messages**:
   ```bash
   git commit -m "Add feature: brief description"
   ```

6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**:
   - Provide a clear description of changes
   - Reference any related issues
   - Explain why the change is needed

## Development Setup

### Prerequisites

- Rust (latest stable)
- Git
- Optional: Physical power supply for hardware testing

### Getting the Source

```bash
git clone https://github.com/Panduza/power-supply.git
cd power-supply
```

### Building

```bash
# Debug build (faster compilation, slower execution)
cargo build

# Release build (slower compilation, faster execution)
cargo build --release
```

### Running

```bash
# Debug mode
cargo run

# Release mode
cargo run --release
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Coding Guidelines

### Rust Style

Follow standard Rust conventions:

- Use `rustfmt` for formatting:
  ```bash
  cargo fmt
  ```

- Use `clippy` for linting:
  ```bash
  cargo clippy
  ```

- Fix all clippy warnings before submitting

### Code Organization

```
src/
  ├── main.rs           # Application entry point
  ├── config.rs         # Configuration management
  ├── drivers/          # Device drivers
  │   ├── emulator.rs   # Emulator driver
  │   ├── kd3005p.rs    # KD3005P driver
  │   └── ...
  ├── mqtt_runner/      # MQTT interface
  ├── mcp/              # MCP interface
  └── server/           # GUI implementation
      └── gui/          # GUI components
```

### Naming Conventions

- **Types**: PascalCase (e.g., `PowerSupplyDriver`)
- **Functions**: snake_case (e.g., `enable_output`)
- **Constants**: UPPER_SNAKE_CASE (e.g., `MAX_VOLTAGE`)
- **Modules**: snake_case (e.g., `mqtt_runner`)

### Documentation

- Add doc comments to public APIs:
  ```rust
  /// Enable the power supply output
  ///
  /// # Errors
  ///
  /// Returns `DriverError` if the device cannot be enabled
  async fn enable_output(&mut self) -> Result<(), DriverError>;
  ```

- Update markdown documentation for user-facing changes
- Keep README.md current

### Error Handling

- Use `Result<T, E>` for fallible operations
- Define custom error types using `thiserror`
- Provide meaningful error messages
- Log errors appropriately

Example:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DriverError {
    #[error("Failed to communicate with device: {0}")]
    CommunicationError(String),
    
    #[error("Voltage {0}V exceeds maximum limit {1}V")]
    VoltageLimitExceeded(f32, f32),
}
```

### Async Code

- Use `async/await` for I/O operations
- Use `tokio` runtime features
- Avoid blocking operations in async context
- Use appropriate timeout values

### Testing

- Add unit tests for new functionality:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      
      #[test]
      fn test_voltage_validation() {
          // Test code here
      }
  }
  ```

- Test edge cases and error conditions
- Use the emulator for integration tests
- Document test requirements

## Adding a New Device Driver

To add support for a new power supply model:

1. **Create driver file**: `src/drivers/your_device.rs`

2. **Implement the trait**:
   ```rust
   use async_trait::async_trait;
   use super::{PowerSupplyDriver, DriverError};
   
   pub struct YourDevice {
       // Device-specific fields
   }
   
   #[async_trait]
   impl PowerSupplyDriver for YourDevice {
       async fn initialize(&mut self) -> Result<(), DriverError> {
           // Implementation
       }
       
       // Implement other required methods...
   }
   ```

3. **Register in factory**: Update `src/factory.rs`:
   ```rust
   pub fn new() -> Self {
       let mut map = HashMap::new();
       map.insert("emulator", Box::new(...) as Box<dyn DriverFactory>);
       map.insert("kd3005p", Box::new(...) as Box<dyn DriverFactory>);
       map.insert("your_device", Box::new(...) as Box<dyn DriverFactory>); // Add this
       
       Self { map }
   }
   ```

4. **Add module**: Update `src/drivers.rs`:
   ```rust
   pub mod emulator;
   pub mod kd3005p;
   pub mod your_device; // Add this
   ```

5. **Document the driver**: Create `docs/devices/your_device.md`

6. **Update sidebar**: Add entry to `docs/_sidebar.md`:
   ```markdown
   * Supported Devices
     * [Emulator](devices/emulator.md)
     * [KD3005P](devices/kd3005p.md)
     * [Your Device](devices/your_device.md)
   ```

7. **Test thoroughly**:
   - With emulator for basic functionality
   - With physical device for hardware validation
   - Document any quirks or limitations

## Documentation Changes

### Documentation Structure

```
docs/
  ├── README.md                    # Landing page
  ├── _sidebar.md                  # Navigation
  ├── getting-started/
  │   ├── quickstart.md
  │   ├── installation.md
  │   └── configuration.md
  ├── interfaces/
  │   ├── mqtt.md
  │   ├── mcp.md
  │   └── gui.md
  ├── devices/
  │   ├── emulator.md
  │   └── kd3005p.md
  ├── testing.md
  └── contributing.md
```

### Writing Documentation

- Use clear, concise language
- Include code examples
- Provide context and motivation
- Use proper markdown formatting
- Test all code examples
- Keep it current with code changes

### Building Documentation Site

The documentation is served using Docsify:

```bash
# Install docsify-cli (if not already installed)
npm i docsify-cli -g

# Serve documentation locally
docsify serve docs
```

Then open http://localhost:3000 to view.

## Commit Message Guidelines

Use clear, descriptive commit messages:

### Format

```
<type>: <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat: Add support for RND 320-KA3305P power supply

Implement driver for the RND 320-KA3305P model, which uses
the same protocol as KD3005P but with different voltage range.

Closes #42
```

```
fix: GUI not updating when MQTT commands received

The GUI event loop was not processing MQTT state updates.
Added subscription to relevant topics and update handlers.

Fixes #38
```

```
docs: Update MQTT interface documentation

Add examples for Python and Node.js clients.
Clarify topic structure and payload formats.
```

## Review Process

Pull requests are reviewed for:

1. **Functionality**: Does it work as intended?
2. **Code quality**: Is it well-written and maintainable?
3. **Testing**: Are there adequate tests?
4. **Documentation**: Are docs updated?
5. **Style**: Does it follow project conventions?
6. **Breaking changes**: Are they necessary and documented?

Expect feedback and be prepared to make revisions.

## Getting Help

- **Issues**: Ask questions in GitHub issues
- **Discussions**: Use GitHub discussions for broader topics
- **Code**: Add comments or create draft PRs for feedback

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).

## Recognition

Contributors are appreciated! Significant contributions may be recognized in:
- CONTRIBUTORS file
- Release notes
- Project documentation

Thank you for helping improve Panduza Power Supply! 🎉
