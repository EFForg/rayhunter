#!/bin/bash -e

# Simplified build script for rayhunter-enhanced
# This script builds all components for the host platform

echo "🏗️  Building rayhunter-enhanced..."
echo "=====================================\n"

# Source build environment
if [ -f ~/.cargo/env ]; then
    source ~/.cargo/env
    echo "✅ Rust environment loaded"
else
    echo "⚠️  Rust environment not found!"
    echo "   Please install Rust and Cargo first"
    exit 1
fi

# Verify compiler availability
if ! command -v gcc &> /dev/null; then
    echo "❌ Host compiler 'gcc' not found"
    exit 1
fi

echo "✅ Build environment configured"
echo "   Host compiler: $(which gcc)"
echo ""

# Build web frontend first
echo "📦 Building web frontend..."
cd daemon/web

# Clean install to avoid audit warnings during build
echo "  📦 Installing dependencies..."
npm ci --silent --audit=false 2>/dev/null || npm install --silent --audit=false

echo "  🔨 Building web assets..."
npm run build --silent

cd ../..
echo "✅ Web frontend built successfully\n"

# Build library and core binaries
echo "🔧 Building core library..."
cargo build --release -p rayhunter
echo "✅ Core library built successfully\n"

echo "🔧 Building telcom-parser..."
cargo build --release -p telcom-parser
echo "✅ Telcom-parser built successfully\n"

# Build binaries
echo "🔧 Building rootshell..."
cargo build --release -p rootshell
echo "✅ Rootshell built successfully\n"

echo "🔧 Building rayhunter-daemon..."
cargo build --release --bin rayhunter-daemon
echo "✅ Rayhunter-daemon built successfully\n"

echo "🔧 Building rayhunter-check..."
cargo build --release --bin rayhunter-check
echo "✅ Rayhunter-check built successfully\n"

# Build installer
echo "🔧 Building installer..."
cargo build --release -p installer
echo "✅ Installer built successfully\n"

echo "🎉 All components built successfully!"
echo "=====================================\n"
echo "📁 Binaries location: target/release/"
echo "📁 Web files location: daemon/web/build/"
echo ""
echo "🚀 Ready to run!" 