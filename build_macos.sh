#!/bin/bash -e

# Rayhunter macOS Build Script
# Cross-compiles daemon and rootshell for ARM hard float
# Builds web interface and installer natively for macOS

set -euo pipefail

echo "🦎 Rayhunter macOS Build Script"
echo "==============================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    print_error "This script is designed for macOS. Current OS: $OSTYPE"
    exit 1
fi

# Check required tools
check_requirements() {
    print_status "Checking build requirements..."
    
    # Check for Docker
    if ! command -v docker &> /dev/null; then
        print_error "Docker is required for cross-compilation but not installed"
        print_status "Please install Docker Desktop for Mac from: https://www.docker.com/products/docker-desktop"
        exit 1
    fi
    
    # Check for Node.js and npm
    if ! command -v node &> /dev/null || ! command -v npm &> /dev/null; then
        print_error "Node.js and npm are required but not installed"
        print_status "Please install Node.js from: https://nodejs.org/"
        exit 1
    fi
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust is required but not installed"
        print_status "Please install Rust from: https://rustup.rs/"
        exit 1
    fi
    
    print_success "All requirements satisfied"
}

# Build web interface
build_web() {
    print_status "Building web interface..."
    pushd bin/web > /dev/null
    
    # Install dependencies if node_modules doesn't exist
    if [ ! -d "node_modules" ]; then
        print_status "Installing npm dependencies..."
        npm install
    fi
    
    # Build the web interface
    npm run build
    print_success "Web interface built successfully"
    popd > /dev/null
}

# Build Docker image for cross-compilation
build_docker_image() {
    print_status "Building Docker development environment..."
    docker build -t rayhunter-devenv -f tools/devenv.dockerfile .
    print_success "Docker environment ready"
}

# Cross-compile daemon and rootshell for ARM hard float
cross_compile_arm() {
    print_status "Cross-compiling daemon and rootshell for ARM hard float..."
    
    # Get current user ID for Docker volume mounting on macOS
    USER_ID=$(id -u)
    GROUP_ID=$(id -g)
    
    # Cross-compile the daemon with firmware profile
    print_status "Building rayhunter-daemon for armv7-unknown-linux-musleabihf..."
    docker run --rm \
        -v "$PWD:/workdir" \
        -w /workdir \
        rayhunter-devenv \
        sh -c 'cargo build --profile firmware --bin rayhunter-daemon --target=armv7-unknown-linux-musleabihf'
    
    # Cross-compile rootshell
    print_status "Building rootshell for armv7-unknown-linux-musleabihf..."
    docker run --rm \
        -v "$PWD:/workdir" \
        -w /workdir \
        rayhunter-devenv \
        sh -c 'cargo build --profile firmware --bin rootshell --target=armv7-unknown-linux-musleabihf'
    
    print_success "ARM cross-compilation completed"
}

# Build installer natively for macOS
build_installer_macos() {
    print_status "Building installer natively for macOS..."
    
    # Determine the correct target triple for current macOS architecture
    if [[ $(uname -m) == "arm64" ]]; then
        NATIVE_TARGET="aarch64-apple-darwin"
        print_status "Detected Apple Silicon (ARM64)"
    else
        NATIVE_TARGET="x86_64-apple-darwin"
        print_status "Detected Intel x86_64"
    fi
    
    # Build installer for native macOS
    cargo build --release --bin installer --target=$NATIVE_TARGET
    
    print_success "macOS installer built successfully for $NATIVE_TARGET"
}

# Create output directory and copy binaries
organize_output() {
    print_status "Organizing build outputs..."
    
    # Create output directory
    OUTPUT_DIR="build_output"
    rm -rf "$OUTPUT_DIR"
    mkdir -p "$OUTPUT_DIR/armhf" "$OUTPUT_DIR/macos"
    
    # Copy ARM binaries
    if [ -f "target/armv7-unknown-linux-musleabihf/firmware/rayhunter-daemon" ]; then
        cp target/armv7-unknown-linux-musleabihf/firmware/rayhunter-daemon "$OUTPUT_DIR/armhf/"
        print_success "Copied rayhunter-daemon (ARM)"
    else
        print_warning "ARM rayhunter-daemon not found"
    fi
    
    if [ -f "target/armv7-unknown-linux-musleabihf/firmware/rootshell" ]; then
        cp target/armv7-unknown-linux-musleabihf/firmware/rootshell "$OUTPUT_DIR/armhf/"
        print_success "Copied rootshell (ARM)"
    else
        print_warning "ARM rootshell not found"
    fi
    
    # Copy macOS installer
    if [[ $(uname -m) == "arm64" ]]; then
        NATIVE_TARGET="aarch64-apple-darwin"
    else
        NATIVE_TARGET="x86_64-apple-darwin"
    fi
    
    if [ -f "target/$NATIVE_TARGET/release/installer" ]; then
        cp "target/$NATIVE_TARGET/release/installer" "$OUTPUT_DIR/macos/"
        print_success "Copied installer (macOS)"
    else
        print_warning "macOS installer not found"
    fi
    
    # Show output summary
    print_status "Build outputs organized in $OUTPUT_DIR:"
    ls -la "$OUTPUT_DIR/"
    if [ -d "$OUTPUT_DIR/armhf" ]; then
        echo "ARM binaries:"
        ls -la "$OUTPUT_DIR/armhf/"
    fi
    if [ -d "$OUTPUT_DIR/macos" ]; then
        echo "macOS binaries:"
        ls -la "$OUTPUT_DIR/macos/"
    fi
}

# Show usage information
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --help, -h          Show this help message"
    echo "  --web-only          Build only the web interface"
    echo "  --arm-only          Build only ARM cross-compiled binaries"
    echo "  --installer-only    Build only the macOS installer"
    echo "  --no-docker         Skip Docker build (use existing image)"
    echo ""
    echo "Default: Build everything (web + ARM cross-compiled + macOS installer)"
}

# Main build process
main() {
    BUILD_WEB=true
    BUILD_ARM=true
    BUILD_INSTALLER=true
    BUILD_DOCKER_IMAGE=true
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_usage
                exit 0
                ;;
            --web-only)
                BUILD_ARM=false
                BUILD_INSTALLER=false
                BUILD_DOCKER_IMAGE=false
                ;;
            --arm-only)
                BUILD_WEB=false
                BUILD_INSTALLER=false
                ;;
            --installer-only)
                BUILD_WEB=false
                BUILD_ARM=false
                BUILD_DOCKER_IMAGE=false
                ;;
            --no-docker)
                BUILD_DOCKER_IMAGE=false
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
        shift
    done
    
    # Start build process
    print_status "Starting Rayhunter macOS build process..."
    print_status "Web Interface: $([ "$BUILD_WEB" = true ] && echo "✓" || echo "✗")"
    print_status "ARM Cross-compile: $([ "$BUILD_ARM" = true ] && echo "✓" || echo "✗")"  
    print_status "macOS Installer: $([ "$BUILD_INSTALLER" = true ] && echo "✓" || echo "✗")"
    echo ""
    
    check_requirements
    
    if [ "$BUILD_WEB" = true ]; then
        build_web
    fi
    
    if [ "$BUILD_ARM" = true ]; then
        if [ "$BUILD_DOCKER_IMAGE" = true ]; then
            build_docker_image
        fi
        cross_compile_arm
    fi
    
    if [ "$BUILD_INSTALLER" = true ]; then
        build_installer_macos
    fi
    
    organize_output
    
    print_success "🎉 Rayhunter build completed successfully!"
    
    echo ""
    echo "Next steps:"
    if [ "$BUILD_ARM" = true ]; then
        echo "• Deploy ARM binaries to your device using ADB or your preferred method"
        echo "• ARM binaries are optimized with the 'firmware' profile for embedded use"
    fi
    if [ "$BUILD_INSTALLER" = true ]; then
        echo "• Use the macOS installer from build_output/macos/ for device setup"
    fi
}

# Run main function with all arguments
main "$@" 