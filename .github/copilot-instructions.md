# Copilot Instructions for foundry-code

## Project Overview
This is a Rust binary project named "foundry-code" using Rust 2024 edition. The project follows a workspace structure with a single binary target named "foundry" and modular crate organization.

## Architecture & Structure
- **Binary Target**: `src/bin/foundry.rs` - Main executable entry point (currently minimal "Hello, world!")
- **Root Library**: `src/lib.rs` - Empty placeholder for binary-specific shared code
- **Workspace**: Multi-crate workspace with `crates/core` as the first member crate
- **Core Crate**: `crates/core` - `foundry-core` library crate (currently empty, ready for core functionality)
- **Modular Design**: Additional feature crates should be added to `crates/` directory following the same pattern

## Build System & Tools
- **Primary Tool**: `just` (Justfile) for common commands
  - `just build` - Build the project
  - `just run` - Run the binary
  - `just test` - Run tests
  - `just check` - Run cargo check
  - `just format` - Format code
  - `just lint` - Run clippy linter
- **Direct Cargo**: Standard Cargo commands work as expected
- **Binary name**: `foundry` (defined in Cargo.toml)
- **Built binary location**: `target/debug/foundry`

## Code Organization
- Main binary logic: `src/bin/foundry.rs`
- Root-level shared functionality: `src/lib.rs` (currently unused)
- Core functionality: `crates/core/src/lib.rs` (`foundry-core` crate)
- New feature crates: `crates/{feature}/` with their own `Cargo.toml`
- Workspace dependencies: Managed at the root `Cargo.toml` level

## CI/CD Pipeline
- **GitHub Actions**: `.github/workflows/rust.yml`
  - Triggers on pushes and PRs to `main` branch
  - Runs `cargo build --verbose` and `cargo test --verbose`
  - Uses Ubuntu latest runner
- **Dependabot**: Configured for weekly Cargo updates and daily GitHub Actions updates

## Current State
- **Binary**: Minimal "Hello, world!" implementation in `src/bin/foundry.rs`
- **Libraries**: Both `src/lib.rs` and `crates/core/src/lib.rs` are empty placeholders
- **Tests**: No test files exist yet - structure ready for implementation
- **Dependencies**: No external dependencies defined in either workspace or core crate

## Key Files
- `Cargo.toml` - Root workspace configuration and binary definition
- `crates/core/Cargo.toml` - Core library crate configuration
- `Justfile` - Development command shortcuts
- `src/bin/foundry.rs` - Main application entry point
- `.github/workflows/rust.yml` - CI pipeline configuration

## Technical Architecture
- Rust 2024 edition with workspace structure
- `foundry-core` as the first member crate for core functionality
- Feature-based modularization using `crates/` directory
- Root-level library available for binary-specific shared code
