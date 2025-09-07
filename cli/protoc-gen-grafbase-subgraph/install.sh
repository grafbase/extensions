#!/usr/bin/env bash

set -e

LATEST_VERSION="0.3.0"
NAME="protoc-gen-grafbase-subgraph"

echo "Installing $NAME v$LATEST_VERSION..."

# Get the operating system and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map the OS and architecture to the target triple
case "$OS" in
    darwin)
        case "$ARCH" in
            x86_64)
                TARGET="x86_64-apple-darwin"
                ;;
            arm64)
                TARGET="aarch64-apple-darwin"
                ;;
            *)
                echo "Error: Unsupported architecture $ARCH on macOS"
                exit 1
                ;;
        esac
        ;;
    linux)
        case "$ARCH" in
            aarch64|arm64)
                TARGET="aarch64-unknown-linux-musl"
                ;;
            x86_64)
                TARGET="x86_64-unknown-linux-musl"
                ;;
            *)
                echo "Error: Unsupported architecture $ARCH on Linux"
                exit 1
                ;;
        esac
        ;;
    *)
        echo "Error: Unsupported operating system $OS"
        exit 1
        ;;
esac

# Create the installation directory
INSTALL_DIR="$HOME/.grafbase/bin"
mkdir -p "$INSTALL_DIR"

# Download the binary
URL="https://github.com/grafbase/extensions/releases/download/$NAME-v$LATEST_VERSION/$NAME-$TARGET"
echo "Downloading from $URL..."

if command -v curl >/dev/null 2>&1; then
    curl -sSL "$URL" -o "$INSTALL_DIR/$NAME"
elif command -v wget >/dev/null 2>&1; then
    wget -q "$URL" -O "$INSTALL_DIR/$NAME"
else
    echo "Error: Neither curl nor wget is installed"
    exit 1
fi

# Make the binary executable
chmod +x "$INSTALL_DIR/$NAME"

echo "$NAME has been installed to $INSTALL_DIR/$NAME"

# Check if the installation directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "Add $INSTALL_DIR to your PATH to use $NAME from anywhere."
    echo ""
    
    # Detect the shell and provide instructions
    if [ -n "$FISH_VERSION" ]; then
        echo "For fish shell, add to ~/.config/fish/config.fish:"
        echo "  fish_add_path $INSTALL_DIR"
    elif [ -n "$ZSH_VERSION" ]; then
        echo "For zsh, add to ~/.zshrc:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    elif [ -n "$BASH_VERSION" ]; then
        echo "For bash, add to ~/.bashrc or ~/.bash_profile:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    else
        echo "Add to your shell configuration file:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
fi

echo ""
echo "Installation complete! You can now use '$NAME'."