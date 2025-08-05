---
description: Dependency management and crate selection rules for Rust projects
applyTo: "**/*.toml,**/*.rs"
---

# Dependency Management Instructions

## Crate Addition Approval
- Always seek developer approval before adding any external crates to `Cargo.toml`
- Explain why the crate is needed and how it fits the project goals
- Discuss whether the functionality could be implemented without external dependencies
- Propose specific version constraints with reasoning

## Crate Selection Criteria
Only suggest crates that meet ALL of the following criteria:
- Widely used in the Rust community
- Actively maintained with recent updates
- Have good documentation and community support
- Follow Rust best practices and conventions
- Have good security track records and regular maintenance

## Restrictions
- Do not suggest pre-1.0 crates without explicit developer approval
- Avoid experimental crates or those with limited adoption
- Prefer standard library solutions when appropriate
- Consider the maintenance burden of additional dependencies

## Version Management
- Suggest specific version constraints (not just `*`)
- Prefer semantic versioning ranges that allow patch updates
- Document the reasoning for version choices
- Consider compatibility with existing dependencies

## Workspace Organization
- New crates should be placed in the `crates/` directory
- Follow the established naming pattern: `foundry-{feature}`
- Add new crate to workspace members in root `Cargo.toml`
- Use workspace dependency inheritance where possible (e.g., `anyhow = { workspace = true }`)
