# Copilot Instructions for foundry-code

## Project Overview
This is a Rust binary project named "foundry-code" using Rust 2024 edition. The project follows a workspace structure with a single binary target named "foundry" and modular crate organization featuring separate CLI and TUI interfaces.

## Architecture & Structure
- **Binary Target**: `src/bin/foundry.rs` - Main executable entry point that routes to CLI or TUI
- **Root Library**: `src/lib.rs` - Empty placeholder for binary-specific shared code
- **Workspace**: Multi-crate workspace with modular interface design
- **Core Crate**: `crates/core` - `foundry-core` library crate for shared business logic
- **CLI Crate**: `crates/cli` - `foundry-cli` library crate for command-line interface (using clap)
- **TUI Crate**: `crates/tui` - `foundry-tui` library crate for terminal user interface (using ratatui)
- **Modular Design**: Feature crates organized in `crates/` directory following established patterns

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
- **Main binary logic**: `src/bin/foundry.rs` - Routes between CLI and TUI based on arguments
- **Root-level shared functionality**: `src/lib.rs` (currently unused)
- **Core functionality**: `crates/core/src/lib.rs` (`foundry-core` crate) - Ready for shared business logic
- **CLI interface**: `crates/cli/src/lib.rs` (`foundry-cli` crate) - Command-line interface with clap
- **TUI interface**: `crates/tui/src/lib.rs` (`foundry-tui` crate) - Terminal UI with ratatui
- **Workspace dependencies**: Managed at the root `Cargo.toml` level with workspace inheritance

## CI/CD Pipeline
- **GitHub Actions**: `.github/workflows/rust.yml`
  - Triggers on pushes and PRs to `main` branch
  - Runs `cargo build --verbose` and `cargo test --verbose`
  - Uses Ubuntu latest runner
- **Dependabot**: Configured for weekly Cargo updates and daily GitHub Actions updates

## Current State
- **Binary**: Fully functional routing between CLI and TUI in `src/bin/foundry.rs`
- **CLI Interface**: Working command-line interface with `--version` and `--help` flags
- **TUI Interface**: Functional terminal UI with welcome screen and quit instructions
- **Libraries**: `src/lib.rs` and `crates/core/src/lib.rs` are empty placeholders ready for implementation
- **Tests**: No test files exist yet - structure ready for implementation
- **Error Handling**: All crates use `anyhow::Result` for consistent error handling

## Key Files
- `Cargo.toml` - Root workspace configuration and binary definition
- `crates/core/Cargo.toml` - Core library crate configuration
- `crates/cli/Cargo.toml` - CLI crate configuration with clap dependency
- `crates/tui/Cargo.toml` - TUI crate configuration with ratatui and crossterm dependencies
- `Justfile` - Development command shortcuts
- `src/bin/foundry.rs` - Main application entry point
- `.github/workflows/rust.yml` - CI pipeline configuration

## Dependencies
- **Workspace-level**: `anyhow = "1.0"` for error handling across all crates
- **CLI Dependencies**: `clap = "4.5"` with derive features for argument parsing
- **TUI Dependencies**: `ratatui = "0.27"` and `crossterm = "0.27"` for terminal interface
- **Core Dependencies**: `anyhow` (workspace) ready for business logic implementation

## Usage Patterns
- **CLI Mode**: `foundry --version` or `foundry --help` for command-line operations
- **TUI Mode**: `foundry` (no arguments) launches the interactive terminal interface

## Technical Architecture
- Rust 2024 edition consistently across all crates in the workspace
- `foundry-core` as the first member crate for core functionality
- Feature-based modularization using `crates/` directory
- Root-level library available for binary-specific shared code
- Consistent error handling with `anyhow` across all crates
- Modular interface design with separate CLI and TUI implementations
