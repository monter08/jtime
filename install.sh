#!/bin/bash

set -e

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Installing JTime - Jira Time Tracking CLI Tool${NC}"

# Detect OS
OS="unknown"
ARCH="unknown"

case "$(uname -s)" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    MINGW*|MSYS*|CYGWIN*) OS="windows";;
esac

# Detect architecture
case "$(uname -m)" in
    x86_64|amd64) ARCH="x86_64";;
    arm64|aarch64) ARCH="aarch64";;
    *)          ARCH="x86_64";;
esac

if [ "$OS" = "unknown" ]; then
    echo "Unsupported operating system. Please install manually."
    exit 1
fi

# Define the download URL
LATEST_VERSION=$(curl -s https://api.github.com/repos/monter08/jtime/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')


# Map OS and architecture to rust-build target format
TARGET=""
if [ "$OS" = "linux" ] && [ "$ARCH" = "x86_64" ]; then
    TARGET="linux-amd64"
    ARCHIVE_EXT="tar.gz"
elif [ "$OS" = "macos" ] && [ "$ARCH" = "x86_64" ]; then
    TARGET="darwin-amd64"
    ARCHIVE_EXT="tar.gz"
elif [ "$OS" = "macos" ] && [ "$ARCH" = "aarch64" ]; then
    TARGET="darwin-arm64"
    ARCHIVE_EXT="tar.gz"
elif [ "$OS" = "windows" ]; then
    TARGET="windows-amd64"
    ARCHIVE_EXT="tar.gz"
else
    echo "Unsupported OS/architecture combination: $OS/$ARCH"
    exit 1
fi

BINARY_NAME="jtime-${TARGET}"
if [ "$OS" = "windows" ]; then
    BINARY_NAME="jtime-${TARGET}.exe"
fi

DOWNLOAD_URL="https://github.com/monter08/jtime/releases/download/${LATEST_VERSION}/jtime-${TARGET}.${ARCHIVE_EXT}"

# Create temp directory
TMP_DIR=$(mktemp -d)
trap 'rm -rf -- "$TMP_DIR"' EXIT

# Download binary
echo "Downloading JTime from $DOWNLOAD_URL"
curl -L -s "$DOWNLOAD_URL" -o "$TMP_DIR/jtime-archive.${ARCHIVE_EXT}"

# Extract the archive
if [ "$ARCHIVE_EXT" = "zip" ]; then
    unzip -q "$TMP_DIR/jtime-archive.${ARCHIVE_EXT}" -d "$TMP_DIR"
else
    tar -xf "$TMP_DIR/jtime-archive.${ARCHIVE_EXT}" -C "$TMP_DIR"
fi

# Make it executable
chmod +x "$TMP_DIR/$BINARY_NAME"

# Install location
INSTALL_DIR="/usr/local/bin"
if [ "$OS" = "windows" ]; then
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
fi

# Move binary to install location
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/"
else
    echo "Installing to $INSTALL_DIR requires admin privileges"
    sudo mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/jtime"
fi

echo -e "${GREEN}JTime successfully installed to $INSTALL_DIR/jtime${NC}"
echo -e "Run ${BLUE}jtime --help${NC} to get started"