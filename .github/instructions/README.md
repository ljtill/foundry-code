# GitHub Copilot Instructions Structure

This directory contains specialized instruction files that customize GitHub Copilot's behavior for the foundry-code project. These files follow VS Code's custom instructions format and are automatically applied based on their scope.

## Instruction Files

### `development-workflow.instructions.md`
- **Scope**: All files (`applyTo: "**"`)
- **Purpose**: Development process and approval workflows
- **Content**: Implementation planning, code review processes, communication guidelines

### `dependency-management.instructions.md`
- **Scope**: Rust and TOML files (`applyTo: "**/*.toml,**/*.rs"`)
- **Purpose**: Crate addition and dependency management rules
- **Content**: Approval processes, selection criteria, version management

### `code-quality.instructions.md`
- **Scope**: Rust files (`applyTo: "**/*.rs"`)
- **Purpose**: Code quality standards and best practices
- **Content**: Rust conventions, architecture compliance, testing requirements

## How It Works

These instruction files are automatically included in GitHub Copilot chat requests based on their `applyTo` patterns. The front matter in each file specifies:
- `description`: Brief explanation of the file's purpose
- `applyTo`: Glob pattern defining when the instructions apply

## Relationship to Other Files

- **`.github/copilot-instructions.md`**: Contains technical project information and architecture details
- **These instruction files**: Contain behavioral rules and workflow processes
- **No overlap**: Each file has a distinct purpose to avoid conflicting instructions

## Usage

These files are automatically detected and used by VS Code when the `github.copilot.chat.codeGeneration.useInstructionFiles` setting is enabled. No manual intervention is required - they work seamlessly with GitHub Copilot in VS Code.
