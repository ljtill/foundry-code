---
description: Code quality standards and best practices for the foundry-code project
applyTo: "**/*.rs"
---

# Code Quality Standards

## Rust Best Practices
- Follow project conventions and established coding style
- Use Rust 2024 edition consistently across all crates
- Implement proper error handling patterns using `anyhow::Result<()>` and `.context()` for descriptive errors
- Include appropriate inline documentation for public APIs
- Use idiomatic Rust patterns and follow community best practices

## Architecture Compliance
- Respect the established modular crate organization
- Follow the workspace dependency management patterns
- Maintain clear separation of concerns between crates
- Propose new crates for distinct feature sets when appropriate

## Code Validation
- Use standard Rust development tools for code quality
- Ensure code passes format checks, linting, and compilation
- Follow consistent formatting standards across the codebase
- Address clippy warnings and suggestions appropriately

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
