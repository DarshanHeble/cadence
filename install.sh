#!/bin/sh
# One-line installer script for Cadence (cad / cadence)
set -e

REPO="DarshanHeble/cadence"
INSTALL_DIR="${HOME}/.local/bin"

echo "Downloading latest Cadence binary..."
mkdir -p "$INSTALL_DIR"

if command -v cargo >/dev/null 2>&1; then
    cargo install --git "https://github.com/${REPO}.git"
    echo "Cadence (cad) installed successfully via Cargo!"
else
    echo "Cargo not found. Please install Rust via https://rustup.rs/ or build from source."
    exit 1
fi
