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

instance_name is optional, by default the application will choose the first instance available.

```bash
# long
panduza --tui [instance_name]

# short
panduza -t [instance_name]
```



## Technical Requirements

- Use crate `clap`
