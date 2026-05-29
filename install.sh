#!/usr/bin/env bash
set -euo pipefail

REPO="halimbahae/phantom-click"
VERSION="${1:-latest}"
INSTALL_DIR="${2:-/usr/local/bin}"

echo "==> Phantom Click Installer"
echo "    Repo: $REPO"
echo "    Version: $VERSION"
echo "    Target: $INSTALL_DIR"

# Detect OS and arch
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux)   OS="linux" ;;
  darwin)  OS="macos" ;;
  mingw*|msys*|cygwin*) OS="windows" ;;
  *)       echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="arm64" ;;
  armv7l) ARCH="armv7" ;;
  *)       echo "Unsupported arch: $ARCH"; exit 1 ;;
esac

echo "    OS: $OS, Arch: $ARCH"

# Determine download URL
if [ "$VERSION" = "latest" ]; then
  API_URL="https://api.github.com/repos/$REPO/releases/latest"
  DOWNLOAD_URL=$(curl -fsSL "$API_URL" | grep "browser_download_url" | grep "$OS-$ARCH" | head -1 | cut -d '"' -f 4)
  if [ -z "$DOWNLOAD_URL" ]; then
    # Fallback: try older naming pattern
    DOWNLOAD_URL=$(curl -fsSL "$API_URL" | grep "browser_download_url" | grep "$OS" | head -1 | cut -d '"' -f 4)
  fi
else
  DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/phantom-click-$OS-$ARCH"
fi

if [ -z "$DOWNLOAD_URL" ]; then
  echo "No prebuilt binary found for $OS-$ARCH"
  echo "Falling back to cargo install..."
  if command -v cargo &>/dev/null; then
    cargo install phantom-click-cli
    cargo install phantom-click-gui
    cargo install phantom-click-mcp
    echo "==> Done! Run 'phantom-click --help' to get started."
    exit 0
  else
    echo "cargo not found. Install Rust from https://rustup.rs"
    exit 1
  fi
fi

echo "==> Downloading $DOWNLOAD_URL"

TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download all binaries
for BIN in "phantom-click" "phantom-click-gui" "phantom-click-mcp"; do
  URL="${DOWNLOAD_URL%/*}/$BIN-$OS-$ARCH"
  if [ "$OS" = "windows" ]; then URL="$URL.exe"; fi
  echo "    Downloading $BIN..."
  curl -fsSL "$URL" -o "$TMP_DIR/$BIN" || {
    # Try without binary name prefix
    ALT_URL="${DOWNLOAD_URL%/*}/$BIN"
    if [ "$OS" = "windows" ]; then ALT_URL="$ALT_URL.exe"; fi
    curl -fsSL "$ALT_URL" -o "$TMP_DIR/$BIN" || {
      echo "    Warning: Could not download $BIN"
      continue
    }
  }
  chmod +x "$TMP_DIR/$BIN"
done

echo "==> Installing to $INSTALL_DIR"
sudo mkdir -p "$INSTALL_DIR"
for f in "$TMP_DIR"/*; do
  if [ -f "$f" ]; then
    sudo mv "$f" "$INSTALL_DIR/$(basename "$f")"
    echo "    Installed $(basename "$f")"
  fi
done

echo ""
echo "==> Done!"
echo ""
echo "    phantom-click         Terminal UI auto-clicker"
echo "    phantom-click-gui     Desktop GUI"
echo "    phantom-click-mcp     MCP server for AI assistants"
echo ""
echo "    Run 'phantom-click --help' to get started."
