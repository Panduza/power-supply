# Module: Command Line Interface

This module manage the CLI.
CLI allow the user to configure the application.

## Functional Requirements

```bash
pza-power-supply list --mcps
pza-power-supply list --drivers
pza-power-supply list --devices


pza-power-supply run --no-tui  --no-broker --no-mcp --no-runners --no-traces
```


## Technical Requirements

- Use crate `clap`
