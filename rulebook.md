# rulebook

General rules and standards for vllm_inferencer workspace.

## Vocabulary

- **Standard Rust Codestyle**: The official Rust formatting conventions as implemented by `rustfmt` and documented in the Rust Style Guide.
- **cargo fmt**: The Rust formatting tool that automatically applies standard Rust codestyle to source code.
- **Custom Codestyle Rules**: Any project-specific formatting rules that deviate from standard Rust conventions (e.g., custom indentation, brace placement).
- **Workspace Member**: A crate within the workspace (`vllm_core`, `vllm_inferencer`, `vllm_server`).

Alternative names: project_rules, vllm_rules

## Governing Principles

This rulebook defines the standards and conventions for the vllm_inferencer workspace. All contributors and code must adhere to these rules.

**Core Principles**:
- **Standard over Custom**: Follow Rust ecosystem standards rather than project-specific conventions
- **Simplicity**: Favor simple, straightforward solutions
- **Consistency**: Maintain consistency across all workspace members

## Quick Reference Summary

| Group                  | Rule                                           | Description                                                       |
|------------------------|------------------------------------------------|-------------------------------------------------------------------|
| **Codestyle** | [Use rustfmt](#codestyle--use-rustfmt) | All Rust code must be formatted with cargo fmt using default settings. |
| **Codestyle** | [Ignore Custom Rules](#codestyle--ignore-custom-rules) | Ignore any custom codestyle rules that conflict with rustfmt. |
| **Codestyle** | [Format Before Commit](#codestyle--format-before-commit) | Run cargo fmt before creating commits. |

## Rules

### Codestyle : Use rustfmt

All Rust source code in the vllm_inferencer workspace must be formatted using `cargo fmt` with default `rustfmt` settings.

**Rationale**: Standard Rust formatting ensures consistency across the Rust ecosystem, improves code readability for all Rust developers, and eliminates formatting debates.

**Application**:
```bash
# Format entire workspace
cargo fmt --all

# Format specific package
cargo fmt -p vllm_core
```

**Scope**: Applies to all `.rs` files in:
- `module/vllm_core/src/`
- `module/vllm_inferencer/src/`
- `module/vllm_server/src/`
- All test files

**Verification**: Run `cargo fmt --check` to verify compliance without modifying files.

### Codestyle : Ignore Custom Rules

Any custom codestyle rules that conflict with standard `rustfmt` formatting must be ignored and not applied.

**Examples of rules to ignore**:
- Custom indentation (e.g., 2-space indents)
- Custom brace placement
- Custom line length limits different from rustfmt defaults
- Custom spacing around operators
- Any formatting rules from other rulebooks that contradict rustfmt

**Rationale**: Maintaining consistency with standard Rust conventions is more valuable than adhering to project-specific formatting preferences.

**Priority**: This rule takes precedence over any conflicting formatting guidance from:
- Workspace lints in `Cargo.toml`
- Other rulebooks in the project
- Previous formatting conventions

> ✅ **Good** - Standard rustfmt formatting

```rust
fn main() {
    let x = 5;
    println!("Hello, world!");
}
```

> ❌ **Bad** - Custom 2-space indentation

```rust
fn main() {
  let x = 5;
  println!("Hello, world!");
}
```

### Codestyle : Format Before Commit

Run `cargo fmt --all` before creating any git commits to ensure all code is properly formatted.

**Workflow**:
1. Make code changes
2. Run `cargo fmt --all`
3. Review formatting changes
4. Stage files: `git add .`
5. Commit: `git commit -m "message"`

**Automation** (optional):
```bash
# Add to .git/hooks/pre-commit
#!/bin/bash
cargo fmt --all --check || {
  echo "Code not formatted. Running cargo fmt..."
  cargo fmt --all
  exit 1
}
```

**Rationale**: Ensures all committed code maintains consistent formatting and prevents formatting-only commits from cluttering history.

**Exceptions**: If using a pre-commit hook, the hook should format code automatically rather than blocking the commit.
