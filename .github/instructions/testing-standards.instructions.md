---
description: Testing standards and practices for the foundry-code project
applyTo: "**/*.rs,**/tests/**"
---

# Testing Standards

## Test Coverage Requirements
- Maintain comprehensive test coverage for all public APIs and core functionality
- Each new crate must include unit tests in its `src/lib.rs` file
- Integration tests should be added to `tests/` directory for cross-crate functionality
- Achieve meaningful test coverage that validates both success and failure scenarios
- Maintain comprehensive test coverage across the workspace, including both integration and TUI unit tests

## Test Organization
- **Unit Tests**: Place in `#[cfg(test)] mod tests` within each crate's `src/lib.rs`
- **Integration Tests**: Place in `tests/integration_tests.rs` or separate files in `tests/` directory
- **Test Naming**: Use descriptive names following `test_<functionality>` pattern
- **Test Groups**: Group related tests in the same module or file

## Testing Tools and Patterns
- Use standard Rust testing framework with `#[test]` attribute
- For CLI testing: Use `clap::Command::try_get_matches_from()` for argument validation
- For TUI testing: Use `ratatui::backend::TestBackend` for UI component testing
- For integration testing: Use `std::process::Command` for binary execution tests
- Test state management, input handling, and component behavior appropriately
- Keep test code clean and focused - let test names be descriptive

## Test Quality Standards
- Tests should be deterministic and not depend on external state
- Include both success and failure scenarios where appropriate
- Use clear assertion messages when failures need context
- Avoid testing implementation details - focus on behavior and contracts
- Tests should run quickly and not require external dependencies

## Running Tests
- Use the project's build system to run tests consistently
- Test individual crates separately when focusing on specific functionality
- Ensure all tests pass before committing changes
- Tests are automatically validated in the CI pipeline

## Test Maintenance
- Update tests when modifying functionality
- Remove or update obsolete tests when refactoring
- Add tests for bug fixes to prevent regressions
- Review test coverage when adding new features
