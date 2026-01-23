#!/bin/bash

set -e

echo "🔨 Building inject_guide for all platforms..."
echo ""

# Build directory
BUILD_DIR="../../../hooks/inject_guide"

# Ensure hooks directory exists
mkdir -p "$BUILD_DIR"

# macOS (current platform)
echo "📦 Building for macOS..."
cargo build --release
cp target/release/inject_guide "$BUILD_DIR/inject_guide_macos"
echo "✅ macOS build complete: $BUILD_DIR/inject_guide_macos"
echo ""

# Windows
echo "📦 Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/inject_guide.exe "$BUILD_DIR/inject_guide.exe"
echo "✅ Windows build complete: $BUILD_DIR/inject_guide.exe"
echo ""

# Linux (using musl for static binary)
if rustup target list | grep -q "x86_64-unknown-linux-musl (installed)"; then
    echo "📦 Building for Linux..."
    cargo build --release --target x86_64-unknown-linux-musl
    cp target/x86_64-unknown-linux-musl/release/inject_guide "$BUILD_DIR/inject_guide_linux"
    echo "✅ Linux build complete: $BUILD_DIR/inject_guide_linux"
else
    echo "⚠️  Linux target not installed. Skipping Linux build."
    echo "   To install: rustup target add x86_64-unknown-linux-musl"
    echo "   To install linker: brew install filosottile/musl-cross/musl-cross"
fi

echo ""
echo "🎉 Build complete!"
echo ""
echo "Output files:"
ls -lh "$BUILD_DIR"/inject_guide*
