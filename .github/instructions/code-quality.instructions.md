---
description: Code quality standards and best practices for the foundry-code project
applyTo: "**/*.rs"
---

# Code Quality Standards

## Rust Best Practices
- Follow project conventions and established coding style
- Use standard development tools as defined in the Justfile:
  - `just format` for consistent formatting
  - `just lint` for clippy linting
  - `just check` for cargo check validation
- Implement proper error handling patterns appropriate for the Rust ecosystem
- Include appropriate inline documentation for public APIs

## Architecture Compliance
- Respect the established modular crate organization
- Keep core functionality in `crates/core` as established
- Place main application logic in `src/bin/foundry.rs`
- Propose new crates in `crates/` directory for distinct feature sets
- Follow the workspace dependency management patterns

## Testing Requirements
- Write tests for new functionality
- Run tests with `just test` to ensure they pass
- Include both unit tests and integration tests where appropriate
- Follow Rust testing conventions and patterns

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
