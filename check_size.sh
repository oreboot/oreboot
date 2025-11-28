#!/usr/bin/env bash
set -e

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
IS_WINDOWS=false
if [[ "$OS" == *"mingw"* ]] || [[ "$OS" == *"cygwin"* ]] || [[ "$OS" == *"msys"* ]]; then
    IS_WINDOWS=true
fi

# Binary paths
if $IS_WINDOWS; then
    BINARY="target/release/xtask.exe"
    STRIP_CMD="llvm-strip"  # or mingw-strip
else
    BINARY="target/release/xtask"
    STRIP_CMD="strip"
fi
BINARY_BIN="${BINARY%.*}.bin"

# Ensure Cargo.toml has [profile.release] section with size optimizations
PROFILE_SETTINGS=$(cat <<EOL

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
debug = 0
strip = true
EOL
)

# Append only if [profile.release] not already exists
if ! grep -q "^\[profile.release\]" Cargo.toml; then
    echo "$PROFILE_SETTINGS" >> Cargo.toml
    echo " Applied min-sized-rust optimizations to Cargo.toml"
else
    echo " [profile.release] exists in Cargo.toml. Make sure it has opt-level=z, lto=true, panic=abort, etc."
fi

echo " Building in release mode..."
cargo build --release

# Check binary exists
if [ ! -f "$BINARY" ]; then
    echo " Could not find the release binary. Build may have failed."
    exit 1
fi

# Show size before strip
echo " Binary size before strip:"
ls -lh "$BINARY"
if command -v size &> /dev/null; then
    size "$BINARY" || true
fi

# Strip binary
if command -v $STRIP_CMD &> /dev/null; then
    echo " Stripping symbols with $STRIP_CMD..."
    $STRIP_CMD "$BINARY" || echo "⚠ Strip failed"
else
    echo "⚠ Strip command ($STRIP_CMD) not found. Skipping..."
fi

# Show size after strip
echo "Binary size after strip:"
ls -lh "$BINARY"
if command -v size &> /dev/null; then
    size "$BINARY" || true
fi

# Generate raw .bin
if command -v objcopy &> /dev/null; then
    echo " Generating raw .bin file..."
    objcopy -O binary "$BINARY" "$BINARY_BIN" || echo "⚠ objcopy failed"
    ls -lh "$BINARY_BIN"
else
    echo "⚠ objcopy not found. Skipping .bin generation."
fi

# Run cargo-bloat
if command -v cargo-bloat &> /dev/null; then
    echo " Running cargo-bloat (top 20 functions):"
    cargo bloat --release -n 20
else
    echo " cargo-bloat not installed. Skipping."
fi
