# Rust Coding Rules

## Comments & Documentation
Rules:
- All comment text MUST be English.
- Use `///` doc comments for items (struct, enum, trait, fn, impl method) needing description.
- Use `//` only for brief inline clarifications.
- Do NOT translate identifiers.
- ALL public items MUST have doc comments (`///`).
- ALL private items SHOULD have doc comments unless trivially obvious.
- Required for: `struct`, `enum`, `trait`, `fn`, `impl` methods, public fields.
- Doc comments MUST describe purpose, behavior, or usage.
- Use complete sentences with proper capitalization and punctuation.

Allowed transformations:
- Convert contiguous `//` block immediately above an item into multiple `///` lines.
- Preserve markdown in doc comments.

Example:
```rust
// BEFORE
// Power supply driver
// Provides connection handling
pub struct Driver { /* ... */ }

// AFTER
/// Power supply driver that provides connection handling.
pub struct Driver { /* ... */ }

/// Represents a power supply device with voltage and current control.
pub struct PowerSupply {
    /// Current voltage output in volts.
    pub voltage: f32,
    /// Current output in amperes.
    pub current: f32,
}

/// Operating status of the power supply.
pub enum Status {
    /// Power supply is active and outputting power.
    On,
    /// Power supply is inactive.
    Off,
    /// Power supply encountered an error condition.
    Error(String),
}

let retries = 3; // fallback attempts for transient link errors
```

Exceptions:
- Standard trait implementations (Debug, Clone, etc.) may omit docs if trivial.
- Test functions may have brief or no documentation.
- Private helper functions with obvious names may omit docs.

Skip if conversion would alter doctest code fences or hidden attribute semantics.

## Imports
Rules:
- Exactly ONE `use` or `mod` per line.
- Use `use super::X` for direct parent module items when appropriate.
- Group with single blank lines between: std, external crates, internal (crate/super).
- Split multi-path brace imports into separate lines; do not merge already separate lines.
- No space between `use` and path.

Example:
```rust
// BEFORE
use std::{fmt, io}; use crate::drivers::emulator; use super::state;

// AFTER
use std::fmt;
use std::io;
use crate::drivers::emulator;
use super::state;
```

Skip reordering if it would change evaluation of `#[cfg]` sections.

## Function Separators
Rules:
- Within each `impl` block, place a separator line between function definitions (also before first or after last).
- Separator: line of `-` chars starting column 1, length 72–80 (choose 78 when adding new):
  `------------------------------------------------------------------------------`
- EXACTLY one blank line before and after separator.

Example:
```rust
impl Driver {
    // ------------------------------------------------------------------------------

    pub fn init(&mut self) {
        /* ... */
    }

    // ------------------------------------------------------------------------------

    pub fn read_status(&self) -> Status {
        /* ... */
    }
    
    // ------------------------------------------------------------------------------
}
```

Skip if insertion would bisect attribute blocks or conditional compilation sections.

## Top-Level Item Separators
Rules:
- Separate top-level item definitions (struct, enum, impl, trait, fn) with separator lines.
- Separator: `// ================` (exactly 18 equals signs)
- Place separator BEFORE each top-level item definition.
- EXACTLY one blank line before and after separator.
- Do NOT place separator before the first item in the file.

Example:
```rust
use std::fmt;
use crate::drivers;

pub struct PowerSupply {
    voltage: f32,
    current: f32,
}

// ================

pub enum Status {
    On,
    Off,
    Error(String),
}

// ================

impl PowerSupply {
    // ------------------------------------------------------------------------------

    pub fn new() -> Self {
        Self {
            voltage: 0.0,
            current: 0.0,
        }
    }

    // ------------------------------------------------------------------------------

    pub fn set_voltage(&mut self, voltage: f32) {
        self.voltage = voltage;
    }
    
    // ------------------------------------------------------------------------------
}

// ================

impl fmt::Display for PowerSupply {
    // ------------------------------------------------------------------------------

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}V {}A", self.voltage, self.current)
    }
    
    // ------------------------------------------------------------------------------
}
```

Skip if insertion would bisect attribute blocks or conditional compilation sections.

## Validation Checklist (Per File)
- [ ] English doc comments standardized with mandatory documentation
- [ ] Single-item import lines enforced & grouped
- [ ] Function separators inserted correctly in impl blocks
- [ ] Top-level item separators inserted correctly