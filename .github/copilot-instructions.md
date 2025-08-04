# Copilot Instructions for foundry-code

## Project Overview
This is a Rust binary project named "foundry-code" using Rust 2024 edition. The project follows a workspace structure with a single binary target named "foundry" that provides both CLI arguments and an interactive TUI interface.

## AI Instruction System
This project uses specialized instruction files in `.github/instructions/` for comprehensive AI guidance:
- `development-workflow.instructions.md` - Implementation planning and approval processes
- `dependency-management.instructions.md` - Dependency management and crate selection rules
- `code-quality.instructions.md` - Rust coding standards and best practices
- `testing-standards.instructions.md` - Testing requirements and organization

These files are automatically applied by VS Code when relevant file patterns are matched.

## Architecture & Structure
- **Binary Target**: `src/bin/foundry.rs` - Main executable entry point with embedded CLI (clap) that routes to TUI when no arguments provided
- **Root Library**: `src/lib.rs` - Empty placeholder for binary-specific shared code
- **Workspace**: Multi-crate workspace with modular crate organization
- **Core Crate**: `crates/core` - `foundry-core` library crate for shared business logic (currently empty placeholder)
- **TUI Crate**: `crates/terminal` - `foundry-terminal` library crate for interactive terminal user interface (using ratatui)
- **Modular Design**: Feature crates organized in `crates/` directory following established patterns

## Build System & Tools
- **Primary Tool**: `just` (Justfile) for common development commands
- **Binary name**: `foundry` (defined in Cargo.toml)
- **Built binary location**: `target/debug/foundry`
- **Direct Cargo**: Standard Cargo commands work as expected

## Code Organization
- **Main binary logic**: `src/bin/foundry.rs` - Contains embedded CLI using clap and routes to TUI when no arguments provided
- **Root-level shared functionality**: `src/lib.rs` (currently unused placeholder)
- **Core functionality**: `crates/core/src/lib.rs` (`foundry-core` crate) - Empty placeholder ready for shared business logic
- **TUI interface**: `crates/terminal/src/lib.rs` (`foundry-terminal` crate) - Full-featured terminal UI with ratatui
- **Workspace dependencies**: Managed at the root `Cargo.toml` level with workspace inheritance

## CI/CD Pipeline
- **GitHub Actions**: `.github/workflows/rust.yml`
  - Triggers on pushes and PRs to `main` branch
  - Uses custom GitHub Actions: `setup-cache` and `setup-just` for optimized builds
  - Parallel job execution: check, build, test, format, and lint jobs
  - Uses `just` commands: `just check`, `just build`, `just test`, `just format`, `just lint`
  - Ubuntu latest runner with comprehensive validation pipeline
- **Dependabot**: Configured in `.github/dependabot.yml` for daily GitHub Actions updates and weekly Cargo updates

## Current State
- **Binary**: Fully functional with embedded CLI routing in `src/bin/foundry.rs` - arguments trigger CLI mode, no arguments launch TUI
- **CLI Interface**: Working command-line interface with `--version` and `--help` flags implemented directly in the main binary
- **TUI Interface**: Full-featured interactive terminal UI with command input, output display, cursor navigation, and comprehensive test coverage
- **Libraries**: `src/lib.rs` and `crates/core/src/lib.rs` are empty placeholders ready for implementation
- **Tests**: Comprehensive test suite covering TUI functionality and binary integration
- **Error Handling**: All crates use `anyhow::Result` for consistent error handling

## Key Files
- `Cargo.toml` - Root workspace configuration and binary definition
- `crates/core/Cargo.toml` - Core library crate configuration
- `crates/terminal/Cargo.toml` - TUI crate configuration with ratatui and crossterm dependencies
- `Justfile` - Development command shortcuts
- `src/bin/foundry.rs` - Main application entry point with embedded CLI
- `tests/integration_tests.rs` - Integration test suite for binary functionality
- `.github/workflows/rust.yml` - CI pipeline configuration
- `.github/dependabot.yml` - Automated dependency updates configuration
- `.github/actions/` - Custom GitHub Actions (`setup-cache`, `setup-just`)
- `.github/instructions/` - Specialized AI instruction files for workflow, dependency management, and code quality

## Dependencies
- **Workspace-level**: `anyhow = "1.0"` for error handling across all crates
- **Root Binary Dependencies**: `clap = "4.5"` with derive features for embedded CLI, `foundry-terminal` for TUI mode
- **TUI Dependencies**: `ratatui = "0.29"` and `crossterm = "0.29"` for terminal interface, `foundry-core` for shared logic
- **Core Dependencies**: `anyhow` (workspace) ready for business logic implementation

## Usage Patterns
- **CLI Mode**: `foundry --version` or `foundry --help` for command-line operations
- **TUI Mode**: `foundry` (no arguments) launches the interactive terminal interface with:
  - Text input with cursor navigation (arrow keys)
  - Command execution and output display
  - Multi-panel layout (status, console, input, instructions)
  - Exit with 'Esc' key

## Technical Architecture
- Workspace-based multi-crate organization with feature separation
- Embedded CLI design with direct routing to TUI when no arguments provided
- Consistent error handling with `anyhow` across all crates
- Root-level library available for binary-specific shared code
