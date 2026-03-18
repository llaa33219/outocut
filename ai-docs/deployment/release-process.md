# Release Process

How to create and publish releases.

## Versioning

### Semantic Versioning

Format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Current Version

Defined in `Cargo.toml`:
```toml
[package]
version = "0.1.0"
```

Also in `src/cli.rs`:
```rust
#[command(version = "0.1.0")]
```

## Release Checklist

### Pre-Release

1. **Update version**
   ```bash
   # Edit Cargo.toml
   # Edit src/cli.rs (if needed)
   ```

2. **Update changelog**
   ```bash
   # Create CHANGELOG.md
   ```

3. **Run full test suite**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Build release**
   ```bash
   cargo build --release
   ```

5. **Test release binary**
   ```bash
   ./target/release/outocut --version
   ./target/release/outocut validate test_project.outocut
   ```

### Release Steps

1. **Create git tag**
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   ```

2. **Push tag**
   ```bash
   git push origin v0.1.0
   ```

3. **Create GitHub release**
   - Use GitHub CLI:
   ```bash
   gh release create v0.1.0 \
     --title "Version 0.1.0" \
     --notes "Release notes here"
   ```

4. **Upload binary**
   ```bash
   gh release upload v0.1.0 \
     ./target/release/outocut \
     --clobber
   ```

### Post-Release

1. **Update version to next**
   ```bash
   # Bump version in Cargo.toml
   # git commit -m "Bump version"
   ```

## Build Targets

### Default Builds

| Platform | Binary Location |
|----------|----------------|
| Linux | `target/release/outocut` |
| macOS | `target/release/outocut` |
| Windows | `target/release/outocut.exe` |

### Cross-Compilation

**Linux → Windows**:
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

**macOS → Linux**:
```bash
# On macOS with cross toolchain
cargo build --release --target x86_64-unknown-linux-gnu
```

## Package Managers

### Homebrew (macOS)

```ruby
# Formula in homebrew-core
class Outocut < Formula
  desc "AI-friendly video editor"
  url "https://github.com/outocut/outocut.git"
  version "0.1.0"
  
  def install
    system "cargo", "build", "--release"
    bin.install "target/release/outocut"
  end
end
```

### AUR (Arch Linux)

```bash
# Package in AUR
git clone https://aur.archlinux.org/outocut.git
cd outocut
makepkg -si
```

## Distribution

### Direct Download

Provide pre-built binaries:
- GitHub Releases
- Create `install.sh` script

### Install Script

See [Install Script](deployment/install-script.md)

## CI/CD

### GitHub Actions

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build
        run: cargo build --release
        
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: target/release/outocut
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Security

### Signed Binaries

Sign releases (future):
```bash
# Sign binary
gpg --armor --detach-sign outocut

# Upload signature
gh release upload v0.1.0 outocut.asc
```

### Checksums

```bash
# Generate checksums
sha256sum target/release/outocut > checksums.txt

# Upload
gh release upload v0.1.0 checksums.txt
```

## Rollback

If release has critical bug:

1. **Revert version**
   ```bash
   git revert "Release v0.1.0"
   ```

2. **Create patch release**
   ```bash
   # Fix bug
   cargo test
   # Bump patch version
   git tag -a v0.1.1
   ```

3. **Announce**
   - Post in releases
   - Update documentation
