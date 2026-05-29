#!/usr/bin/env bash
set -uo pipefail
IFS=$'\n\t'

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

# Determine download URLs from the GitHub release
ASSET_NAMES=("phantom-click" "phantom-click-gui" "phantom-click-mcp")
if [ "$OS" = "windows" ]; then
  ASSET_NAMES=("phantom-click.exe" "phantom-click-gui.exe" "phantom-click-mcp.exe")
fi

BINARIES_DOWNLOADED=0

if [ "$VERSION" = "latest" ]; then
  API_URL="https://api.github.com/repos/$REPO/releases/latest"
else
  API_URL="https://api.github.com/repos/$REPO/releases/tags/$VERSION"
fi

# Try API first
echo "==> Fetching release info..."
API_DATA=$(curl -fsSL "$API_URL" 2>/dev/null) || API_DATA=""

if [ -n "$API_DATA" ]; then
  TAG=$(echo "$API_DATA" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tag_name',''))" 2>/dev/null)
  echo "    Found release: $TAG"

  TMP_DIR=$(mktemp -d)
  trap "rm -rf $TMP_DIR" EXIT

  for NAME in "${ASSET_NAMES[@]}"; do
    # Get the download URL for this asset from the API
    DL_URL=$(echo "$API_DATA" | python3 -c "
import sys, json
data = json.load(sys.stdin)
for a in data.get('assets', []):
    if a['name'] == '$NAME':
        print(a['browser_download_url'])
" 2>/dev/null)

    if [ -z "$DL_URL" ]; then
      # Try alternative naming pattern: name-OS-ARCH or name-OS_ARCH
      for pattern in "$NAME-$OS-$ARCH" "$NAME-$OS_$ARCH" "$NAME-$OS-$ARCH.exe"; do
        DL_URL=$(echo "$API_DATA" | python3 -c "
import sys, json
data = json.load(sys.stdin)
for a in data.get('assets', []):
    if a['name'] == '$pattern':
        print(a['browser_download_url'])
" 2>/dev/null)
        [ -n "$DL_URL" ] && break
      done
    fi

    if [ -n "$DL_URL" ]; then
      echo "    Downloading $NAME..."
      curl -fsSL "$DL_URL" -o "$TMP_DIR/$NAME" && {
        chmod +x "$TMP_DIR/$NAME"
        BINARIES_DOWNLOADED=1
        echo "      ✓ $NAME"
      } || echo "      ✗ $NAME (download failed)"
    else
      echo "    Skipping $NAME (not found in release assets)"
    fi
  done
else
  echo "    (API unavailable — GitHub may be rate-limited)"
  echo "    Will try cargo install instead."
fi

if [ "$BINARIES_DOWNLOADED" = "1" ]; then
  echo ""
  echo "==> Installing to $INSTALL_DIR"
  mkdir -p "$INSTALL_DIR" 2>/dev/null || sudo mkdir -p "$INSTALL_DIR"
  for f in "$TMP_DIR"/*; do
    if [ -f "$f" ]; then
      if mv "$f" "$INSTALL_DIR/$(basename "$f")" 2>/dev/null; then
        echo "    Installed $(basename "$f")"
      elif sudo mv "$f" "$INSTALL_DIR/$(basename "$f")"; then
        echo "    Installed $(basename "$f") (with sudo)"
      else
        echo "    Failed to install $(basename "$f")"
      fi
    fi
  done

  echo ""
  echo "==> Done! Binaries installed to $INSTALL_DIR"
  echo "    phantom-click         Terminal UI auto-clicker"
  echo "    phantom-click-gui     Desktop GUI"
  echo "    phantom-click-mcp     MCP server for AI assistants"
  echo ""
  echo "    Run 'phantom-click --help' to get started."
  exit 0
fi

# Fallback: cargo install
echo "==> No prebuilt binaries found. Falling back to cargo install..."
if command -v cargo &>/dev/null; then
  echo "    Installing phantom-click-cli..."
  cargo install phantom-click-cli || { echo "    Failed to install phantom-click-cli"; exit 1; }
  echo "    Installing phantom-click-mcp..."
  cargo install phantom-click-mcp || { echo "    Failed to install phantom-click-mcp"; exit 1; }
  echo "    Installing phantom-click-gui..."
  cargo install phantom-click-gui || echo "    (gui install optional, skipped)"

  # Detect cargo bin directory and ensure it's in PATH
  CARGO_BIN="$(dirname "$(cargo install --list 2>/dev/null | head -1)" 2>/dev/null)"
  if [ -z "$CARGO_BIN" ]; then
    CARGO_BIN="$HOME/.cargo/bin"
  fi

  if [ -d "$CARGO_BIN" ]; then
    # Symlink to INSTALL_DIR if different from cargo bin
    if [ "$CARGO_BIN" != "$INSTALL_DIR" ]; then
      echo ""
      echo "    Also creating symlinks in $INSTALL_DIR..."
      mkdir -p "$INSTALL_DIR" 2>/dev/null || sudo mkdir -p "$INSTALL_DIR"
      for BIN in phantom-click phantom-click-gui phantom-click-mcp; do
        if [ -f "$CARGO_BIN/$BIN" ]; then
          ln -sf "$CARGO_BIN/$BIN" "$INSTALL_DIR/$BIN" 2>/dev/null || \
          sudo ln -sf "$CARGO_BIN/$BIN" "$INSTALL_DIR/$BIN" 2>/dev/null || true
        fi
      done
      echo "    Done."
    fi

    # Check PATH
    case ":$PATH:" in
      *":$CARGO_BIN:"*) ;;
      *)
        echo ""
        echo "    ⚠  $CARGO_BIN is not in your PATH!"
        echo "    Add this to your ~/.zshrc or ~/.bashrc:"
        echo "      export PATH=\"\$PATH:$CARGO_BIN\""
        echo "    Or run: phantom-click from full path: $CARGO_BIN/phantom-click"
        ;;
    esac
  fi

  echo ""
  echo "==> Done! Run 'phantom-click --help' to get started."
  exit 0
else
  echo ""
  echo "ERROR: cargo not found. Install Rust from https://rustup.rs"
  echo "Or download binaries manually from:"
  echo "  https://github.com/$REPO/releases"
  exit 1
fi
