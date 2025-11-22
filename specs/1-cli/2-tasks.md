# 02 - Tasks for CLI implementation

This file breaks the `1-cli` feature into small, independently implementable tasks that map to single commits or PRs. Refer to the stories in `0-stories.md` and requirements in `1-reqs.md` (notably: 5-TR1 requires using the `clap` crate).

Each task includes a short description, acceptance criteria and a suggested verification command.

## Tasks

1. Add `clap` dependency
   - Description: Add a compatible `clap` 4.x dependency to `Cargo.toml` and run `cargo update` to fetch it.
   - Acceptance criteria: `cargo build` succeeds and `clap` appears in `Cargo.lock`.
   - Verify: `cargo build`.

2. Create CLI types (`src/cli.rs`)
   - Description: Create `src/cli.rs` that defines a `Cli` struct and `Commands` enum using `clap::Parser` (derive). Define subcommands at minimum: `gui` and `server`. Use clap's built-in version support (see task 4).
   - Acceptance criteria: The crate compiles and `pza-power-supply --help` prints the clap-generated help text.
   - Verify: `cargo run -- --help` (or `cargo build` then `./target/debug/pza-power-supply --help`).

3. Refactor `main` to dispatch on CLI
   - Description: Refactor the existing `main` logic so the CLI parser dispatches to appropriate functions. Move the current application bootstrap/GUI launch into a `run_gui()` function and the background services into `run_server()` (either in `src/main.rs` or in `src/lib.rs`). Update `main` to parse `Cli` and call the selected subcommand.
   - Acceptance criteria: `pza-power-supply gui` launches the GUI (same behaviour as current `main`), and `pza-power-supply server` runs background services without launching the GUI.
   - Verify: `cargo run -- gui` and `cargo run -- server` behave as described.

4. Ensure `--version` shows package version
   - Description: Wire clap to show the crate name and version when `--version` is passed (use `env!` macros or clap-provided metadata so the output matches `Cargo.toml`).
   - Acceptance criteria: `pza-power-supply --version` prints `pza-power-supply 0.1.1` (or matches `package.name`/`package.version` in `Cargo.toml`).
   - Verify: `cargo run -- --version`.

5. Add unit/integration tests for CLI parsing
   - Description: Add a test file `tests/cli.rs` that uses `clap`'s `try_parse_from` (or the derive types' `try_parse_from`) to verify parsing of `--version`, `gui`, and `server` subcommands and common flags.
   - Acceptance criteria: `cargo test` passes and tests adequately cover command parsing behaviour.
   - Verify: `cargo test --lib` or `cargo test`.

6. Document usage and update specs
   - Description: Update `README.md` (or `docs/`) with short usage examples for the new CLI and ensure `specs/1-cli/*` reference each other. Add this `2-tasks.md` file to the spec folder (this file).
   - Acceptance criteria: `README.md` contains a brief CLI usage section and `specs/1-cli/` contains `0-stories.md`, `1-reqs.md`, and `2-tasks.md`.
   - Verify: open `README.md` and `specs/1-cli/2-tasks.md`.

7. Smoke test build and basic commands
   - Description: Perform a full build and smoke-test the common commands to catch integration issues.
   - Acceptance criteria: `cargo build` and `cargo test` pass; `pza-power-supply --version`, `pza-power-supply gui`, and `pza-power-supply server` run without panics for basic flows.
   - Verify: `cargo build`, `cargo test`, and manual invocations.

## Notes and implementation guidance
- Keep each change small and self-contained: adding the dependency, adding `src/cli.rs`, and the `main` refactor should be separate commits/PRs.
- Prefer using `clap` derive API (`#[derive(Parser)]`) for a compact and well-documented CLI definition.
- When refactoring `main`, avoid changing behaviour except to add CLI dispatch; existing initialization and logging should be preserved.
- If you prefer a different set of subcommands (for example `client`, `emulator`, or `driver`), create follow-up tasks and mark them `[NEEDS CLARIFICATION]` in the spec.

## Acceptance sign-off
Implementation is complete when all tasks above are marked done, `cargo build` and `cargo test` pass, and the CLI behaviour matches the stories in `0-stories.md` (notably `--version`).
