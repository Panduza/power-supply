# Rust Coding Rules

## Comments
Rules:
- All comment text MUST be English.
- Use `///` doc comments for items (struct, enum, trait, fn, impl method) needing description.
- Use `//` only for brief inline clarifications.
- Do NOT translate identifiers.

Allowed transformations:
- Convert contiguous `//` block immediately above an item into multiple `///` lines.
- Preserve markdown in doc comments.

Example:
```rust
// BEFORE
// Power supply driver
// Provides connection handling
pub struct Driver { /* ... */ }

/// Power supply driver
/// Provides connection handling
pub struct Driver { /* ... */ }

let retries = 3; // fallback attempts for transient link errors
```

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
- Within each `impl` block, place a separator line between function definitions (not before first or after last).
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

## Validation Checklist (Per File)
- [ ] English doc comments standardized
- [ ] Single-item import lines enforced & grouped
- [ ] Separators inserted correctly