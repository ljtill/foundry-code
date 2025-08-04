# Justfile for foundry-code project
# Usage: just [recipe] [args...]

# Show available recipes
[group('general')]
default:
    just --list

#
# Build
#

# Build the entire workspace
[group('build')]
build *args="":
    @cargo build {{args}}

# Build a specific package
[group('build')]
build-pkg package *args="":
    @cargo build --package {{package}} {{args}}

#
# Release
#

# Build workspace in release mode
[group('release')]
release *args="":
    @cargo build --release {{args}}

#
# Test
#

# Run all workspace tests
[group('test')]
test *args="":
    @cargo test {{args}}

# Test a specific package
[group('test')]
test-pkg package *args="":
    @cargo test --package {{package}} {{args}}

#
# Run
#

# Run the foundry binary
[group('run')]
run *args="":
    @cargo run -q {{args}}

#
# Clean
#

# Clean the entire workspace
[group('clean')]
clean *args="":
    @cargo clean {{args}}

# Clean a specific package
[group('clean')]
clean-pkg package *args="":
    @cargo clean --package {{package}} {{args}}

#
# Dependencies
#

# Show dependency tree
[group('dependency')]
deps-tree:
    @cargo tree

#
# Syntax
#

# Check the current package
[group('syntax')]
check *args="":
    @cargo check {{args}}

# Format the codebase
[group('syntax')]
format *args="":
    @cargo fmt -- --check {{args}}

# Format & fix the codebase
[group('syntax')]
format-fix *args="":
    @cargo fmt {{args}}

# Lint the codebase
[group('syntax')]
lint *args="":
    @cargo clippy {{args}}

# Lint & fix the codebase
[group('syntax')]
lint-fix *args="":
    @cargo clippy --fix --allow-dirty --allow-staged {{args}}
