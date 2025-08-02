# Copilot Instructions for foundry-code

## Project Overview
This is a Rust binary project named "foundry-code" using Rust 2024 edition. The project follows a workspace structure with a single binary target named "foundry" and modular crate organization.

## Architecture & Structure
- **Binary Target**: `src/bin/foundry.rs` - Main executable entry point (currently minimal "Hello, world!")
- **Root Library**: `src/lib.rs` - Empty placeholder for binary-specific shared code
- **Workspace**: Multi-crate workspace with `crates/core` as the first member crate
- **Core Crate**: `crates/core` - `foundry-core` library crate (currently empty, ready for core functionality)
- **Modular Design**: Additional feature crates should be added to `crates/` directory following the same pattern

## Development Workflow

### Build System
- **Primary Tool**: `just` (Justfile) for common commands
  - `just build` - Build the project
  - `just run` - Run the binary
  - `just test` - Run tests
  - `just check` - Run cargo check
  - `just format` - Format code
  - `just lint` - Run clippy linter
- **Direct Cargo**: Standard Cargo commands work as expected

### Binary Execution
- Binary name: `foundry` (defined in Cargo.toml)
- Run with: `cargo run` or `just run`
- Built binary location: `target/debug/foundry`

### Code Organization Patterns
- Main binary logic should go in `src/bin/foundry.rs`
- Root-level shared functionality in `src/lib.rs` (currently unused)
- Core functionality belongs in `crates/core/src/lib.rs` (`foundry-core` crate)
- New feature crates should follow the pattern: `crates/{feature}/` with their own `Cargo.toml`
- Workspace dependencies are managed at the root `Cargo.toml` level

## CI/CD Pipeline
- **GitHub Actions**: `.github/workflows/rust.yml`
  - Triggers on pushes and PRs to `main` branch
  - Runs `cargo build --verbose` and `cargo test --verbose`
  - Uses Ubuntu latest runner
- **Dependabot**: Configured for weekly Cargo updates and daily GitHub Actions updates

## Development Guidelines
- Follow Rust 2024 edition conventions
- Use `cargo fmt` for consistent formatting (available via `just format`)
- Use `cargo clippy` for linting (available via `just lint`)
- Ensure all code passes `cargo check` before committing
- Write tests for new functionality - run with `just test`

## Current State
- **Binary**: Contains minimal "Hello, world!" implementation in `src/bin/foundry.rs`
- **Libraries**: Both `src/lib.rs` and `crates/core/src/lib.rs` are empty placeholders
- **Tests**: No test files exist yet - structure ready for implementation
- **Dependencies**: No external dependencies defined in either workspace or core crate

## Key Files
- `Cargo.toml` - Root workspace configuration and binary definition
- `crates/core/Cargo.toml` - Core library crate configuration
- `Justfile` - Development command shortcuts
- `src/bin/foundry.rs` - Main application entry point
- `.github/workflows/rust.yml` - CI pipeline configuration

## Future Architecture Notes
- Project is set up as a workspace with `foundry-core` as the first member crate
- `crates/` directory follows the pattern for feature-based modularization
- Consider adding new crates to workspace members when implementing distinct features
- Root-level library (`src/lib.rs`) remains available for binary-specific code
