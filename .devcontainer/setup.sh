#!/bin/sh
set -e

# Keep in sync with the version cocogitto/cocogitto-action@v3 installs in CI.
COG_VERSION=6.3.0

echo "Installing Rust toolchain from rust-toolchain.toml..."
rustup show active-toolchain || rustup toolchain install

echo "Installing cocogitto $COG_VERSION..."
case "$(uname -m)" in
    x86_64) COG_TARGET=x86_64-unknown-linux-musl ;;
    aarch64 | arm64) COG_TARGET=aarch64-unknown-linux-gnu ;;
    *) echo "Unsupported architecture: $(uname -m)" >&2; exit 1 ;;
esac
BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"
curl -fsSL "https://github.com/cocogitto/cocogitto/releases/download/$COG_VERSION/cocogitto-$COG_VERSION-$COG_TARGET.tar.gz" \
    | tar -xz -C "$BIN_DIR" --strip-components=1 --wildcards '*/cog'
"$BIN_DIR/cog" --version

echo "Warming the build cache..."
cargo fetch --locked
cargo build --all-targets --locked
