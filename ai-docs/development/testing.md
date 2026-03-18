# Testing Guide

Comprehensive testing guide for OutOcut.

## Test Structure

### Unit Tests

Located in the same file as the code being tested:

```rust
// src/parser.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_line_comments() {
        let input = r#"{
            // This is a comment
            "key": "value"
        }"#;
        let output = strip_comments(input);
        assert!(!output.contains("This is a comment"));
    }
}
```

### Integration Tests

Located in `tests/` directory:

```rust
// tests/integration_test.rs
use outocut::*;

#[test]
fn test_full_render_pipeline() {
    // Test complete render
}
```

## Running Tests

### All Tests

```bash
cargo test
```

### Specific Module

```bash
cargo test parser
cargo test animation
```

### Specific Test

```bash
cargo test test_name
```

### With Output

```bash
cargo test -- --nocapture
```

### Release Tests

```bash
cargo test --release
```

## Test Categories

### Parser Tests

```rust
#[test]
fn test_strip_line_comments() { /* ... */ }

#[test]
fn test_strip_block_comments() { /* ... */ }

#[test]
fn test_preserve_string_content() { /* ... */ }

#[test]
fn test_valid_project() { /* ... */ }

#[test]
fn test_invalid_dimensions() { /* ... */ }
```

### Animation Tests

```rust
#[test]
fn test_linear_easing() { /* ... */ }

#[test]
fn test_ease_in_out() { /* ... */ }

#[test]
fn test_keyframe_interpolation() { /* ... */ }

#[test]
fn test_layer_visibility() { /* ... */ }
```

### Render Tests

```rust
#[test]
fn test_frame_render() { /* ... */ }

#[test]
fn test_layer_composite() { /* ... */ }

#[test]
fn test_hex_to_rgba() { /* ... */ }
```

## Writing Tests

### Basic Test Structure

```rust
#[test]
fn test_name() {
    // Arrange: Set up test data
    let input = value();
    
    // Act: Execute function
    let result = function(input);
    
    // Assert: Verify result
    assert_eq!(result, expected);
}
```

### Testing Errors

```rust
#[test]
fn test_error_case() {
    let result = function(invalid_input);
    
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    assert!(err.to_string().contains("expected error"));
}
```

### Testing with Fixtures

```rust
struct TestFixture {
    project: Project,
    // ...
}

impl TestFixture {
    fn new() -> Self {
        // Create test data
    }
}

#[test]
fn test_with_fixture() {
    let fixture = TestFixture::new();
    // use fixture
}
```

## Test Coverage

### Running with Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --output-formatted html
```

### Coverage Reports

Generated in `tarpaulin-report.html`

## Property-Based Testing

For functions with mathematical properties:

```rust
#[cfg(test)]
use quickcheck::QuickCheck;

#[test]
fn test_interpolation_properties(#[quickcheck] f: f64) {
    // Property: interpolation is bounded by endpoints
    let result = interpolate(0.0, 1.0, f);
    assert!(result >= 0.0 && result <= 1.0);
}
```

## Benchmarking

```rust
#[cfg(test)]
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_parse(c: &mut Criterion) {
    c.bench_function("parse_10mb_project", |b| {
        b.iter(|| parse_project(large_project_path()))
    });
}

criterion_group!(benches, benchmark_parse);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

## CI Testing

### GitHub Actions

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      - run: cargo test
      - run: cargo clippy
      - run: cargo fmt --check
```

## Best Practices

1. **Test one thing** per test
2. **Use descriptive names**: `test_invalid_fps_returns_error`
3. **Test edge cases**: empty, null, max values
4. **Test error paths**: not just happy path
5. **Keep tests independent**: no shared state
6. **Run tests frequently**: don't wait for CI

## Debugging Failed Tests

```bash
# Show output
cargo test -- --nocapture

# Run single thread
cargo test -- --test-threads=1

# Show backtrace
RUST_BACKTRACE=1 cargo test
```
