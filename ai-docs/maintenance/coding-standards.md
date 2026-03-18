# Coding Standards

Coding conventions and standards for OutOcut.

## General Principles

1. **Readability over cleverness**
2. **Consistency across the codebase**
3. **Self-documenting code**
4. **Testable design**

## Rust-Specific Standards

### Formatting

Use automatic formatting:
```bash
cargo fmt
```

Check formatting:
```bash
cargo fmt --check
```

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Modules | snake_case | `animation.rs` |
| Functions | snake_case | `parse_project()` |
| Variables | snake_case | `project_path` |
| Structs | PascalCase | `RenderEngine` |
| Enums | PascalCase | `LayerType` |
| Enum variants | PascalCase | `SomeVariant` |
| Constants | SCREAMING_SNAKE | `MAX_FPS` |
| Traits | PascalCase | `Serialize` |

### Field Naming

Use `serde(rename = "...")` for JSON fields:

```rust
#[derive(Serialize, Deserialize)]
pub struct Layer {
    #[serde(rename = "type")]
    pub layer_type: LayerType,
    
    #[serde(rename = "startTime")]
    pub start_time: f64,
    
    #[serde(rename = "mainCompositionId")]
    pub main_composition_id: String,
}
```

This maintains JSON compatibility while using Rust naming conventions.

### Error Handling

Use `anyhow` for application code:

```rust
use anyhow::Result;

fn process() -> Result<()> {
    let data = read_file()?;
    parse_data(data)?;
    Ok(())
}
```

Use `thiserror` for library code:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    
    #[error("Missing field: {0}")]
    MissingField(String),
}
```

### No Type Suppression

NEVER use:
```rust
let value = x as any;
let _ = x;  // Silence unused
#[allow(unused)]
```

ALWAYS handle properly:
```rust
let value = proper_conversion(x);
let _unused = used_later;
```

### Avoid Panics

Use proper error handling:
```rust
// Bad
fn get_item(vec: &Vec<T>, idx: usize) -> &T {
    vec.get(idx).unwrap()  // Panics on bad index
}

// Good
fn get_item(vec: &Vec<T>, idx: usize) -> Option<&T> {
    vec.get(idx)
}
```

## Code Organization

### Module Structure

```rust
// src/lib.rs
pub mod parser;
pub mod models;
pub mod composition;
pub mod animation;
pub mod render;

// Re-export for convenient use
pub use parser::*;
pub use models::*;
```

### File Organization

1. Imports
2. Public types
3. Private types
4. Implementations
5. Tests

```rust
// 1. Imports
use std::path::Path;
use anyhow::Result;

// 2. Public types
pub struct PublicStruct { ... }

// 3. Private types
struct PrivateStruct { ... }

// 4. Implementations
impl PublicStruct {
    pub fn new() -> Self { ... }
}

// 5. Tests
#[cfg(test)]
mod tests { ... }
```

### Line Length

Maximum 100 characters per line.

### Import Organization

```rust
// Standard library
use std::fs;
use std::path::Path;

// External crates
use anyhow::Result;
use serde::{Deserialize, Serialize};

// Local modules
use crate::models::*;
```

## Testing Standards

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() { ... }

    #[test]
    fn test_edge_case() { ... }
}
```

### Test Naming

Use descriptive names:
- `test_parse_valid_project`
- `test_animation_ease_in`
- `test_blend_multiply`

### Test Assertions

```rust
// Specific assertions
assert_eq!(result, expected);
assert!(result.is_ok());
assert!(!result.is_empty());

// With messages
assert_eq!(result, expected, "Failed for input: {}", input);
```

## Documentation

### Public API

Document all public functions:

```rust
/// Parse a .outocut project file.
///
/// # Arguments
/// * `path` - Path to project file
///
/// # Returns
/// Parsed project or error
///
/// # Example
/// ```
/// let project = parse_project("test.outocut")?;
/// ```
pub fn parse_project(path: &Path) -> Result<Project> { ... }
```

### Internal Code

Comment complex logic:
```rust
// Calculate bezier curve parameter t
// Uses Newton-Raphson iteration for numerical solving
let t = solve_cubic_bezier(y1, y2, y, t_guess);
```

Don't comment obvious code:
```rust
// Bad
// Increment counter
counter += 1;

// Good
// Newton-Raphson iteration for solving bezier
t = next_iteration(t);
```

## Git Conventions

### Commit Messages

```
type(scope): description

[optional body]

[optional footer]
```

Types: feat, fix, docs, style, refactor, test, chore

### Branch Naming

- `feature/description`
- `fix/issue-description`
- `docs/update-section`

## Performance Guidelines

### Avoid Premature Optimization

1. Write clear code first
2. Profile to find bottlenecks
3. Optimize the hot paths
4. Document performance-critical code

### Memory Management

- Use stack allocation when possible
- Consider `&[T]` and `&str` over owned types
- Clear unnecessary references

## Review Checklist

Before submitting PR:

- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Formatting applied (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] No type suppression (`as any`, `@ts-ignore`)
- [ ] Error handling proper
- [ ] Comments added for complex logic
- [ ] Documentation updated (if API changed)
