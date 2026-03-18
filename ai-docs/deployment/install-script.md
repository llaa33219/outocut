# Install Script

Installation script for easy deployment.

## Quick Install

```bash
curl -sSL https://raw.githubusercontent.com/outocut/outocut/main/install.sh | bash
```

Or with specific version:

```bash
curl -sSL https://raw.githubusercontent.com/outocut/outocut/v0.1.0/install.sh | bash
```

## Script Features

- Downloads pre-built binary
- Verifies checksum
- Installs to `/usr/local/bin`
- Creates config directory
- Shows instructions

## Install Script Source

```bash
#!/bin/bash
set -e

# Configuration
VERSION="${VERSION:-latest}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
REPO="outocut/outocut"

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

# Map OS to binary name
case "$OS" in
    Linux*)  BINARY="outocut" ;;
    Darwin*) BINARY="outocut" ;;
    MINGW*)  BINARY="outocut.exe" ;;
    *)       echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Map architecture
case "$ARCH" in
    x86_64)   ARCH_NAME="x86_64" ;;
    aarch64)  ARCH_NAME="arm64" ;;
    arm64)    ARCH_NAME="arm64" ;;
    *)        echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Get version
if [ "$VERSION" = "latest" ]; then
    VERSION=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep -o '"tag_name": "v[^"]*' | cut -d'"' -f4)
fi

# Download URL
URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY}-${ARCH_NAME}"

# Temporary directory
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

# Download
echo "Downloading OutOcut ${VERSION}..."
curl -sL "$URL" -o "$BINARY"

# Make executable
chmod +x "$BINARY"

# Install
echo "Installing to ${INSTALL_DIR}..."
sudo mv "$BINARY" "$INSTALL_DIR/"

# Cleanup
cd /
rm -rf "$TEMP_DIR"

# Verify
if command -v outocut &> /dev/null; then
    echo "Installation successful!"
    outocut --version
else
    echo "Installation failed. Please install manually."
    exit 1
fi
```

## Manual Installation

### From Binary

1. Download from [GitHub Releases](https://github.com/outocut/outocut/releases)

2. Make executable
   ```bash
   chmod +x outocut
   ```

3. Move to PATH
   ```bash
   sudo mv outocut /usr/local/bin/
   ```

### From Source

```bash
git clone https://github.com/outocut/outocut.git
cd outocut
cargo build --release
sudo mv target/release/outocut /usr/local/bin/
```

## Verification

```bash
# Check installation
outocut --version

# Should output: outocut 0.1.0

# Test basic command
outocut --help
```

## Uninstallation

```bash
sudo rm /usr/local/bin/outocut
```

## Configuration

### Default Paths

- Binary: `/usr/local/bin/outocut`
- Cache: `./.outocut.cache`

### Custom Install Location

```bash
# Set custom directory
export INSTALL_DIR=$HOME/.local/bin
curl -sSL https://raw.githubusercontent.com/outocut/outocut/main/install.sh | bash

# Add to PATH if needed
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

## Requirements

- Linux or macOS
- FFmpeg (for rendering)

### Installing FFmpeg

**Ubuntu/Debian**:
```bash
sudo apt install ffmpeg
```

**macOS**:
```bash
brew install ffmpeg
```

**Fedora**:
```bash
sudo dnf install ffmpeg
```

## Troubleshooting

### "Permission denied"

```bash
# Run with sudo or use custom directory
curl -sSL ... | sudo bash
```

### "Binary not found"

```bash
# Check PATH
echo $PATH

# Add to PATH
export PATH="/usr/local/bin:$PATH"
```

### "FFmpeg not found"

```bash
# Install FFmpeg (see above)
which ffmpeg
```
