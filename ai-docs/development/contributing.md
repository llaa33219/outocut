# Contributing Guide

Guidelines for contributing to OutOcut.

## Getting Started

1. **Fork** the repository
2. **Clone** your fork
3. **Create** a feature branch
4. **Make** your changes
5. **Test** your changes
6. **Submit** a pull request

## Code Style

### General Principles

- Write **clear, readable code** over clever code
- Add comments for **complex algorithms** only
- Use **meaningful variable names**
- Keep functions **small and focused**

### Rust-Specific Guidelines

#### Naming Conventions

```rust
// Variables: snake_case
let project_name = "my_project";
let frame_count = 30;

// Functions: snake_case
fn parse_project(path: &Path) -> Result<Project> {}

// Structs/Enums: PascalCase
struct RenderEngine;
enum LayerType;

// Constants: SCREAMING_SNAKE_CASE
const MAX_FPS: f64 = 120.0;
```

#### Error Handling

```rust
// Use anyhow for application errors
use anyhow::Result;

fn process_file(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;
    // ... process content
    Ok(())
}

// Use thiserror for library errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Missing required field: {0}")]
    MissingField(String),
}
```

#### Avoid Type Suppression

```rust
// NEVER do this:
let value = some_function() as any;

// DO this:
let value = some_function();
```

#### Empty Error Handling

```rust
// NEVER do this:
catch (e) {}

// ALWAYS do this:
catch(e) {
    eprintln!("Error: {}", e);
}
```

## Testing

### Unit Tests

Add tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_behavior() {
        let result = my_function(42);
        assert_eq!(result, expected_value);
    }
}
```

### Integration Tests

Add in `tests/` directory:

```rust
// tests/integration_test.rs
use outocut::*;

#[test]
fn test_full_render() {
    // Test complete render pipeline
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Commit Messages

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

### Examples

```
feat(parser): add support for block comments

Add /* */ comment parsing to the JSON parser.
This allows users to add documentation directly
in their .outocut files.

Fixes #123
```

```
fix(animation): correct cubic bezier interpolation

The cubic bezier solver was not converging for
some edge cases. Added iterative refinement.
```

## Pull Request Process

### Before Submitting

1. **Run tests**: `cargo test`
2. **Check formatting**: `cargo fmt`
3. **Run clippy**: `cargo clippy`
4. **Build release**: `cargo build --release`

### PR Description

Include:
- **Summary** of changes
- **Motivation** (why this change)
- **Testing** performed
- **Screenshots** (if UI changes)

### Review Process

1. Automated checks run (CI)
2. Code review by maintainers
3. Address feedback
4. Merge when approved

## File Organization

### Adding New Modules

1. Create `src/new_module.rs`
2. Add `mod new_module;` to `src/lib.rs`
3. Add tests
4. Update documentation

### Adding New CLI Commands

1. Add variant to `Commands` enum in `cli.rs`
2. Implement handler function
3. Add command to match in `main.rs`
4. Add tests

## Documentation

### User Documentation (README.md)

- Quick start guide
- CLI command reference
- Installation instructions

### Developer Documentation (ai-docs/)

- Architecture overview
- Module descriptions
- API documentation
- Design decisions

### Code Comments

Comment when:
- Algorithm is non-obvious
- Business logic requires explanation
- Edge case handling
- Performance optimization

Don't comment:
- Obvious code
- What the code does (name it well instead)
- Commented-out code (delete it)

## Questions?

- Open an issue for bugs
- Start a discussion for features
- Ask in community channels
