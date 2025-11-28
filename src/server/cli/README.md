# Module: Command Line Interface

This module manage the CLI.
CLI allow the user to configure the application.

## Functional Requirements

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

- Scan devices pluged to the computer and return a json representation on stdout.

```bash
pza-power-supply -–scan
```

- Scan devices pluged to the computer and return a json representation on stdout.

```bash
pza-power-supply -–mcp-list
```

## Technical Requirements

- Use crate `clap`
