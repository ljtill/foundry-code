---
description: Code quality standards and best practices for the foundry-code project
applyTo: "**/*.rs"
---

# Code Quality Standards

## Rust Best Practices
- Follow project conventions and established coding style
- Use Rust 2024 edition consistently across all crates (set `edition = "2024"` in all Cargo.toml files)
- Use standard development tools as defined in the Justfile:
  - `just format` for consistent formatting
  - `just lint` for clippy linting
  - `just check` for cargo check validation
- Implement proper error handling patterns using `anyhow::Result<()>` and `.context()` for descriptive errors
- Include appropriate inline documentation for public APIs

## Architecture Compliance
- Respect the established modular crate organization (see copilot-instructions.md for details)
- Follow the workspace dependency management patterns
- Propose new crates in `crates/` directory for distinct feature sets

## Testing Requirements
- Write tests for new functionality (see testing-standards.instructions.md for details)
- Run tests with `just test` to ensure they pass

## Documentation Standards
- Document public APIs with clear examples
- Update README.md when adding significant features
- Include inline comments for complex logic
- Maintain consistency with existing documentation style

## Performance Considerations
- Consider performance implications of implementations
- Use appropriate data structures and algorithms
- Prefer zero-cost abstractions when possible
- Profile performance-critical code when necessary
