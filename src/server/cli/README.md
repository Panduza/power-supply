# Module: Command Line Interface

This module manage the CLI.
CLI allow the user to configure the application.

## Functional Requirements

- Command to list power supply instance names

```bash
# long
pza-power-supply --list

# short
pza-power-supply -l
```

- Command to start a control box TUI

TUI should be by default, if the user wants to interact it must be easy and short to write.
instance_name is optional, by default the application will choose the first instance available.

```bash
# long
pza-power-supply [instance_name]

# short
pza-power-supply [instance_name]
```

- Command to disable the TUI

When script call the application, it is important to be able to disable the TUI and start only server services.

```bash
pza-power-supply -–disable-tui
```

- Command to force disable MCP servers

```bash
pza-power-supply -–disable-mcp
```


## Technical Requirements

- Use crate `clap`


## NOTES for later improvement

DO NOT PROCESS THIS

–-identity must print the identity on stdout and update the identity file.
–-scan must print the scan results on stdout.

