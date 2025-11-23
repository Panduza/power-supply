# Module: Command Line Interface

This module manage the CLI.
CLI allow the user to configure the application.

## Functional Requirements

- Command to list power supply instance names

```bash
# long
panduza --list

# short
panduza -l
```

- Command to start a control box TUI

```bash
# long
panduza --tui [instance_name]

# short
panduza -t [instance_name]
```

## Technical Requirements

- Use crate `clap`
