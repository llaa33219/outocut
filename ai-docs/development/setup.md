# Development Setup

Guide for setting up OutOcut development environment.

## Prerequisites

### Required Tools

1. **Rust** (version 1.75 or higher)
   ```bash
   # Install via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Verify installation
   rustc --version
   # Should output: rustc 1.75.0 or higher
   ```

2. **FFmpeg** (for video encoding)
   ```bash
   # Ubuntu/Debian
   sudo apt install ffmpeg
   
   # macOS
   brew install ffmpeg
   
   # Verify installation
   ffmpeg -version
   ```

3. **Git** (for version control)
   ```bash
   # Most systems have this pre-installed
   git --version
   ```

## Clone and Build

### 1. Clone Repository

```bash
git clone https://github.com/outocut/outocut.git
cd outocut
```

### 2. Build Debug Version

```bash
cargo build
```

This creates a debug build at `target/debug/outocut`.

### 3. Build Release Version

```bash
cargo build --release
```

This creates an optimized build at `target/release/outocut`.

### 4. Run Tests

```bash
cargo test
```

Expected output:
```
running 3 tests
test parser::tests::test_strip_line_comments ... ok
test parser::tests::test_strip_block_comments ... ok
test parser::tests::test_preserve_string_content ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 5. Run CLI

```bash
# Debug build
./target/debug/outocut --help

# Release build
./target/release/outocut --help
```

## Project Structure

```
outocut/
├── Cargo.toml           # Project manifest
├── Cargo.lock           # Dependency lock file
├── README.md            # User documentation
├── ai-docs/             # Developer documentation
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library root
│   ├── cli.rs           # CLI command definitions
│   ├── parser.rs        # JSON parsing
│   ├── models.rs        # Data structures
│   ├── composition.rs    # Layer composition
│   ├── animation.rs     # Keyframe system
│   └── render.rs        # Rendering engine
└── tests/               # Integration tests (if any)
```

## Development Workflow

### Running Individual Commands

```bash
# Validate a project file
./target/release/outocut validate test_project.outocut

# Preview at a specific time
./target/release/outocut preview test_project.outocut --time 5.0

# Export JSON
./target/release/outocut export-json test_project.outocut --pretty
```

### Testing Changes

```bash
# Quick rebuild (faster after initial build)
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- validate test_project.outocut
```

### Adding Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
new_crate = "1.0"
```

Then run:
```bash
cargo fetch
cargo build
```

## IDE Setup

### VS Code

Install extensions:
- `rust-analyzer` - Rust language support
- `rustfmt` - Code formatting

Settings (`.vscode/settings.json`):
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "rustfmt.previewStyle": "Default"
}
```

### IntelliJ IDEA / CLion

Install the Rust plugin:
- File → Settings → Plugins → Marketplace
- Search "Rust"

## Common Issues

### "cc" not found

Install C compiler:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# macOS
xcode-select --install
```

### FFmpeg not found

Ensure FFmpeg is in your PATH:
```bash
which ffmpeg
# Should output: /usr/bin/ffmpeg or similar
```

### Compilation errors

Make sure Rust is up to date:
```bash
rustup update
cargo update
```

## Next Steps

- Read [Contributing Guide](development/contributing.md)
- Read [Coding Standards](maintenance/coding-standards.md)
- Explore [Architecture](architecture/overview.md)
