# Cargo Dependency Rules

## Dependencies Organisation
Rules:
- Alphabetically sort crate names within each section.
- Precede each dependency block with a fenced 3-line comment:
  - `# ---`
  - Short description (capitalized, ≤ 60 chars, no trailing period)
  - `# ---`
- Feature arrays: one feature per line, trailing comma on all but last optional; preserve existing style if already compliant.
- Keep version spec unchanged.
- No wildcard (`*`) or unsafely broad ranges introduced.

Example:
```toml
# ---
# Modbus protocol support
 tokio-modbus = { version = "0.16.5", default-features = false, features = [
    "rtu",
    "tcp",
 ] }
# ---
# Serial port abstraction
 tokio-serial = "5.4.5"
# ---
```
