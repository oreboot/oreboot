#!/usr/bin/env bash
set -e

# Detect platform
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
IS_WINDOWS=false
if [[ "$OS" == *"mingw"* ]] || [[ "$OS" == *"cygwin"* ]] || [[ "$OS" == *"msys"* ]]; then
    IS_WINDOWS=true
fi

# Binary paths
if $IS_WINDOWS; then
    BINARY_ELF="target/release/xtask.exe"
    STRIP_CMD="llvm-strip"  # Use llvm-strip or mingw strip
else
    BINARY_ELF="target/release/xtask"
    STRIP_CMD="strip"
fi
BINARY_BIN="${BINARY_ELF%.*}.bin"

# Build
echo " Building in release mode..."
cargo build --release

# Check binary exists
if [ ! -f "$BINARY_ELF" ]; then
    echo " Could not find the release binary. Build may have failed."
    exit 1
fi

# Size before strip
echo " Binary size before strip:"
ls -lh "$BINARY_ELF"
if command -v size &> /dev/null; then
    echo "Section sizes:"
    size "$BINARY_ELF" || true
fi

# Strip symbols
if command -v $STRIP_CMD &> /dev/null; then
    echo "Stripping symbols with $STRIP_CMD..."
    $STRIP_CMD "$BINARY_ELF" || echo "⚠ Strip failed"
else
    echo "Strip command ($STRIP_CMD) not found. Skipping..."
fi

# Size after strip
echo " Binary size after strip:"
ls -lh "$BINARY_ELF"
if command -v size &> /dev/null; then
    echo "Section sizes:"
    size "$BINARY_ELF" || true
fi

# Generate raw .bin
if command -v objcopy &> /dev/null; then
    echo " Generating raw .bin file..."
    objcopy -O binary "$BINARY_ELF" "$BINARY_BIN" || echo " objcopy failed"
    ls -lh "$BINARY_BIN"
else
    echo " objcopy not found. Skipping .bin generation."
fi

# Cargo bloat
if command -v cargo-bloat &> /dev/null; then
    echo " Running cargo-bloat (top 20 functions):"
    cargo bloat --release -n 20
else
    echo " cargo-bloat not installed. Skipping."
fi
