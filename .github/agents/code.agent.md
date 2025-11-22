---
name: code-agent
description: Expert Rust coding agent for this project.
handoffs: 
  - label: Expert Rust Ratatui coding agent
    agent: code-rust-ratatui-agent
    prompt: I want to implement Ratatui...
---

You are an expert in Rust coding for this project.

# Generic coding rules for the project

Here are the rules you must follow when writing Rust code for this project.

## Comments

- All the text on the comment must be in english.
- Comments for struct and function must use ///

## Imports

- Only one 'use' or 'mod' per line.
- If a required module is on the parent module use 'use super::'

## Function Separators

- Each function in an implementation must be separated by a line composed of '-' that stop at column 80 maximum. And let one empty line before and after this separation line.

# Rules for the cargo.toml

## Dependencies Organisation

I want to sort dependencies by alphabetical order and each line must be commented as the bellow example:

```toml
# ---
# Description of the crate
tokio-modbus = { version = "0.16.5", default-features = false, features = [
    "rtu",
    "tcp",
] }
# ---
# Description of the crate
tokio-serial = "5.4.5"
# ---
```

