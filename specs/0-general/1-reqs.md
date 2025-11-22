# General Requirements

## Summary

Here are the general requirements of the project.

## Technical Requirements

- 1-TR1: This project must be implemented in Rust, at least version 1.91.

- 1-TR2: Do not use `mod.rs` files to define modules. Projects must follow the Rust 2018+ module layout convention: prefer a file-per-module layout (for example `src/foo.rs`) or the explicit `src/foo/mod.rs` only when using a directory module is necessary; however the project policy disallows legacy `mod.rs` usage — prefer `foo.rs` or `foo/mod.rs` with clear module root names. Tooling and CI should flag `mod.rs` files as a violation.

- 1-TR3: Functions must be split for clarity and maintainability. Prefer single-responsibility functions that are short and focused (recommendation: prefer functions under 80 lines). Complex or multi-step logic must be broken into named helper functions with clear responsibilities and covered by unit tests. Code reviews and CI should enforce this policy; add a CI linting check or static analysis script that flags overly large functions or unusually high cyclomatic complexity when practical.

