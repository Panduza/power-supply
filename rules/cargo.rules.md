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