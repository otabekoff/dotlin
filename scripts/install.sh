#!/bin/sh
# Dotlin Installation Script for Unix-like systems (Linux, macOS)
# Usage: curl --proto '=https' --tlsv1.2 -sSf https://dotlin.dev/install.sh | sh

set -e

# Configuration
DOTLIN_VERSION="${DOTLIN_VERSION:-latest}"
INSTALL_DIR="${DOTLIN_HOME:-$HOME/.dotlin}"
BIN_DIR="$INSTALL_DIR/bin"
GITHUB_REPO="dotlin-lang/dotlin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
info() {
    printf "${GREEN}[INFO]${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1"
}

error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1"
    exit 1
}

# Detect OS and architecture
detect_platform() {
    local os
    local arch
    
    case "$(uname -s)" in
        Linux*)     os="linux" ;;
        Darwin*)    os="macos" ;;
        *)          error "Unsupported operating system: $(uname -s)" ;;
    esac
    
    case "$(uname -m)" in
        x86_64)     arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)          error "Unsupported architecture: $(uname -m)" ;;
    esac
    
    echo "${os}-${arch}"
}

# Download and extract Dotlin
install_dotlin() {
    local platform
    platform=$(detect_platform)
    
    info "Detected platform: $platform"
    info "Installing Dotlin $DOTLIN_VERSION to $INSTALL_DIR"
    
    # Create installation directory
    mkdir -p "$BIN_DIR"
    
    # Determine download URL
    local download_url
    if [ "$DOTLIN_VERSION" = "latest" ]; then
        download_url="https://github.com/$GITHUB_REPO/releases/latest/download/dotlin-$platform.tar.gz"
    else
        download_url="https://github.com/$GITHUB_REPO/releases/download/v$DOTLIN_VERSION/dotlin-$platform.tar.gz"
    fi
    
    info "Downloading from: $download_url"
    
    # Download and extract
    local temp_file
    temp_file=$(mktemp)
    
    if command -v curl > /dev/null 2>&1; then
        curl -fsSL "$download_url" -o "$temp_file" || error "Failed to download Dotlin"
    elif command -v wget > /dev/null 2>&1; then
        wget -q "$download_url" -O "$temp_file" || error "Failed to download Dotlin"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
    
    info "Extracting archive..."
    tar -xzf "$temp_file" -C "$INSTALL_DIR" || error "Failed to extract archive"
    rm "$temp_file"
    
    # Make binaries executable
    chmod +x "$BIN_DIR"/*
    
    info "Dotlin installed successfully!"
}

# Update PATH
update_path() {
    local shell_rc
    
    # Detect shell configuration file
    if [ -n "$BASH_VERSION" ]; then
        shell_rc="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        shell_rc="$HOME/.zshrc"
    else
        shell_rc="$HOME/.profile"
    fi
    
    # Check if PATH already contains Dotlin
    if echo "$PATH" | grep -q "$BIN_DIR"; then
        info "PATH already contains $BIN_DIR"
        return
    fi
    
    # Add to shell configuration
    info "Adding $BIN_DIR to PATH in $shell_rc"
    echo "" >> "$shell_rc"
    echo "# Dotlin" >> "$shell_rc"
    echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$shell_rc"
    
    warn "Please restart your shell or run: source $shell_rc"
}

# Verify installation
verify_installation() {
    export PATH="$PATH:$BIN_DIR"
    
    if command -v dotc > /dev/null 2>&1; then
        local version
        version=$(dotc --version 2>&1 | head -n 1)
        info "Verification successful: $version"
        info "Installed components:"
        ls -1 "$BIN_DIR"
    else
        error "Installation verification failed. dotc not found in PATH."
    fi
}

# Main installation flow
main() {
    info "Dotlin Installation Script"
    info "=========================="
    
    install_dotlin
    update_path
    verify_installation
    
    echo ""
    info "Dotlin has been installed successfully!"
    info "To get started, run: dotc --help"
    info "For the REPL, run: dotrepl"
    info ""
    info "Documentation: https://dotlin.dev/docs"
    info "Examples: https://github.com/$GITHUB_REPO/tree/main/examples"
}

main "$@"
