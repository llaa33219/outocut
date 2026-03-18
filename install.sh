#!/usr/bin/env bash
set -e

REPO="llaa33219/outocut"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="outocut"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        CYGWIN*)    echo "windows";;
        MINGW*)     echo "windows";;
        *)          echo "unknown"
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64)     echo "x86_64";;
        aarch64)    echo "aarch64";;
        armv7l)     echo "armv7l";;
        *)          echo "unknown"
    esac
}

# Check dependencies
check_deps() {
    info "Checking dependencies..."

    if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
        error "curl or wget is required but not installed."
        exit 1
    fi

    if ! command -v cargo &> /dev/null; then
        warn "Rust/Cargo not found. Will try to install pre-built binary."
        RUST_INSTALLED=false
    else
        RUST_INSTALLED=true
    fi

    if ! command -v ffmpeg &> /dev/null; then
        warn "FFmpeg not found. Install it for video rendering."
        info "  Ubuntu/Debian: sudo apt install ffmpeg"
        info "  macOS: brew install ffmpeg"
    fi
}

# Download pre-built binary
download_binary() {
    local os=$1
    local arch=$2
    local version="${3:-latest}"

    info "Downloading OutoCut ${version} for ${os}-${arch}..."

    # Determine file extension
    local ext="tar.gz"
    local archive_name="outocut-${os}-${arch}.tar.gz"

    # Get download URL
    local base_url="https://github.com/${REPO}/releases/download"
    local download_url="${base_url}/${version}/${archive_name}"

    # Create temp directory
    local tmp_dir=$(mktemp -d)
    local archive_path="${tmp_dir}/${archive_name}"

    # Download
    if command -v curl &> /dev/null; then
        curl -fsSL -o "${archive_path}" "${download_url}"
    else
        wget -q -O "${archive_path}" "${download_url}"
    fi

    # Extract
    info "Installing to ${INSTALL_DIR}..."
    mkdir -p "${INSTALL_DIR}"
    tar -xzf "${archive_path}" -C "${INSTALL_DIR}"

    # Cleanup
    rm -rf "${tmp_dir}"

    info "${BINARY_NAME} installed to ${INSTALL_DIR}"
}

# Build from source
build_from_source() {
    info "Building from source (requires Rust)..."

    if [ ! -d "${HOME}/outocut" ]; then
        info "Cloning repository..."
        git clone "https://github.com/${REPO}.git" "${HOME}/outocut"
    else
        info "Using existing repository..."
        cd "${HOME}/outocut"
        git pull
    fi

    cd "${HOME}/outocut"
    cargo build --release

    mkdir -p "${INSTALL_DIR}"
    cp "target/release/${BINARY_NAME}" "${INSTALL_DIR}/"

    info "${BINARY_NAME} installed to ${INSTALL_DIR}"
}

# Add to PATH if needed
setup_path() {
    local shell_rc=""

    if [ -n "$BASH_VERSION" ]; then
        shell_rc="${HOME}/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        shell_rc="${HOME}/.zshrc"
    fi

    if [ -n "$shell_rc" ]; then
        local path_line="export PATH=\"${INSTALL_DIR}:\$PATH\""
        if ! grep -q "${INSTALL_DIR}" "$shell_rc" 2>/dev/null; then
            echo "" >> "$shell_rc"
            echo "# OutoCut" >> "$shell_rc"
            echo "${path_line}" >> "$shell_rc"
            info "Added ${INSTALL_DIR} to PATH in ${shell_rc}"
            info "Run 'source ${shell_rc}' or restart your terminal"
        fi
    fi

    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        warn "${INSTALL_DIR} is not in your PATH."
        warn "Add this line to your shell config:"
        echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi
}

# Main
main() {
    local install_type="source"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --binary)
                install_type="binary"
                shift
                ;;
            --version)
                VERSION="$2"
                shift 2
                ;;
            --dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            *)
                error "Unknown option: $1"
                echo "Usage: $0 [--binary] [--version VERSION] [--dir DIR]"
                exit 1
                ;;
        esac
    done

    check_deps

    local os=$(detect_os)
    local arch=$(detect_arch)

    if [ "$os" == "unknown" ] || [ "$arch" == "unknown" ]; then
        warn "Could not detect OS/arch. Building from source."
        install_type="source"
    fi

    if [ "$install_type" == "binary" ] && [ "$RUST_INSTALLED" == "false" ]; then
        error "Cannot download binary: curl/wget not available."
        exit 1
    fi

    if [ "$install_type" == "binary" ] && [ "$RUST_INSTALLED" == "true" ]; then
        download_binary "$os" "$arch" "${VERSION:-latest}"
    else
        build_from_source
    fi

    setup_path

    echo ""
    info "Installation complete!"
    info "Run 'outocut --help' to get started."
}

main "$@"
