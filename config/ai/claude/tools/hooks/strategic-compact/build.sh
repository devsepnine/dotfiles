#!/bin/bash

set -e

echo "🔨 Building strategic-compact hook for all platforms..."
echo ""

# Build directory
HOOK_DIR="../../../hooks/suggest-compact"

# Ensure hooks directory exists
mkdir -p "$HOOK_DIR"

# macOS (current platform)
echo "📦 Building for macOS..."
cargo build --release

cp target/release/suggest-compact "$HOOK_DIR/suggest-compact_macos"

echo "✅ macOS build complete"
echo ""

# Windows
echo "📦 Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

cp target/x86_64-pc-windows-gnu/release/suggest-compact.exe "$HOOK_DIR/suggest-compact.exe"

echo "✅ Windows build complete"
echo ""

# Linux (using musl for static binary)
if rustup target list | grep -q "x86_64-unknown-linux-musl (installed)"; then
    echo "📦 Building for Linux..."
    if cargo build --release --target x86_64-unknown-linux-musl 2>/dev/null; then
        cp target/x86_64-unknown-linux-musl/release/suggest-compact "$HOOK_DIR/suggest-compact_linux"

        echo "✅ Linux build complete"
    else
        echo "⚠️  Linux build failed (linker issue). Skipping Linux build."
        echo "   To install musl linker: brew install filosottile/musl-cross/musl-cross"
    fi
else
    echo "⚠️  Linux target not installed. Skipping Linux build."
    echo "   To install: rustup target add x86_64-unknown-linux-musl"
    echo "   To install linker: brew install filosottile/musl-cross/musl-cross"
fi

echo ""
echo "🎉 Build complete!"
echo ""
echo "Output files:"
ls -lh "$HOOK_DIR"/suggest-compact*
