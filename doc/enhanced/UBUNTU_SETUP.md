# Ubuntu Setup Guide for Rayhunter Enhanced

This guide covers setting up the build environment for rayhunter-enhanced on Ubuntu systems with comprehensive cross-compilation support, Docker environment, and GPS integration.

## Prerequisites

**IMPORTANT: Always start with updating your package lists:**

```bash
sudo apt update
```

This ensures you have the latest package information and prevents dependency issues.

## Setup Options

### Option 1: Docker Environment (Recommended for New Users)
The Docker environment provides a complete, isolated build environment with all dependencies pre-configured:

```bash
# Start Docker environment
./docker-build.sh up
./docker-build.sh shell

# Inside container - simple 3-step process
./setup_ubuntu_ci.sh     # Install toolchains & dependencies
./fetch_source.sh        # Download latest source code (if needed)
./build_and_deploy.sh    # Build and deploy to device
```

**Docker Benefits:**
- ✅ **Isolated environment** - No system modifications required
- ✅ **All dependencies included** - Ubuntu 22.04 with full toolchain
- ✅ **Persistent storage** - Work survives container restarts
- ✅ **Cross-compilation ready** - ARM toolchain pre-configured
- ✅ **adb support** - Direct device deployment via USB
- ✅ **USB device access** - Full access to connected devices

### Option 2: Local Dependencies (Recommended for Native)
No root access needed after initial tool installation. Downloads and installs all dependencies locally.

```bash
# Install basic tools if missing (one-time setup)
sudo apt update
sudo apt install -y curl tar unzip xz-utils ca-certificates

# Install dependencies locally
./setup_local_deps.sh

# Build the project
./build_all.sh
```

### Option 3: System-Wide CI Setup
Installs everything system-wide. Good for CI/CD environments.

```bash
# This script runs apt update automatically at the beginning
./setup_ubuntu_ci.sh
./fetch_source.sh
./build_and_deploy.sh
```

### Option 4: Interactive Setup
Full interactive setup with options for Docker, additional tools, etc.

```bash
# This script runs apt update automatically at the beginning
./setup_ubuntu_build_env.sh
./build_all.sh
```

## Docker Setup

For containerized builds with enhanced USB access:

```bash
# Build Docker environment
./docker-build.sh build

# Start container with USB access
./docker-build.sh up

# Access the container
./docker-build.sh shell

# If USB devices not accessible, restart with enhanced access
./restart_container_with_usb.sh
```

## Environment Verification

Before building, verify your environment:

```bash
# Test cross-compilation setup
./test_cross_compilation.sh

# This verifies:
# - Build scripts compile for host architecture
# - Target binaries compile for ARM
# - No compiler conflicts
# - Environment variables are set correctly
```

## Enhanced Features

### GPS Integration
The enhanced version includes comprehensive GPS integration:

- ✅ **Real-time GPS coordinate submission** via REST API
- ✅ **Mobile app compatibility** (GPS2REST-Android)
- ✅ **Multiple export formats** (CSV, JSON, GPX)
- ✅ **Per-scan GPS files** with automatic timestamp correlation
- ✅ **External GPS device support** via API endpoints

### GPS API Usage
```bash
# Submit GPS coordinates (GET method - GPS2REST-Android compatible)
curl "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"

# Submit GPS coordinates (POST method)
curl -X POST "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"

# Download GPS data for a recording session
curl "http://192.168.1.1:8080/api/gps/1720080123/csv" -o gps_data.csv
```

## Common Issues

### Package Cache Issues
If you see warnings about old package cache, run:
```bash
sudo apt update
```

### Missing Tools
If basic tools are missing, install them:
```bash
sudo apt update
sudo apt install -y build-essential curl git
```

### ARM Cross-Compiler Issues
If ARM cross-compilation fails:
```bash
sudo apt update
sudo apt install -y gcc-arm-linux-gnueabihf libc6-dev-armhf-cross
```

### Docker USB Access Issues
If Docker container can't access USB devices:
```bash
# Restart container with enhanced USB access
./restart_container_with_usb.sh

# Check USB device permissions
lsusb
# Should show your device

# Verify adb connection from container
adb devices
# Should show device as "device"
```

## Build Commands

Once environment is set up:

```bash
# Clean build
./clean.sh

# Full build with environment detection
./build_all.sh

# Test build environment
./test_cross_compilation.sh

# Deploy to device with GPS API endpoints
./deploy.sh
```

## Environment Variables

The setup scripts configure these automatically:
- `CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc`
- `CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc`
- Proper PATH for all tools
- Cross-compilation environment isolation

## Enhanced Build System

### Cross-Compilation Improvements
The build system has been enhanced with comprehensive cross-compilation fixes:

- ✅ **Proper host/target separation** - Build scripts compile for host (x86_64), target binaries for ARM
- ✅ **PATH management** - Ensures correct compiler resolution
- ✅ **Environment isolation** - Prevents ARM compiler from interfering with build scripts
- ✅ **Automatic verification** - Tests cross-compilation setup before building

### Available Build Scripts
- **`./build_all.sh`** - Comprehensive build with environment detection and verification
- **`./make.sh`** - Quick build for iterative development
- **`./clean.sh`** - Clean all build artifacts and prepare for fresh build
- **`./deploy.sh`** - Deploy to device via adb with GPS API endpoints
- **`./test_cross_compilation.sh`** - Verify cross-compilation environment

## Troubleshooting

### Build Errors
1. Run `./test_cross_compilation.sh` to diagnose issues
2. Check that `apt update` was run recently
3. Verify all tools are installed: `rustc --version`, `node --version`, `arm-linux-gnueabihf-gcc --version`

### Permission Errors
- Don't run setup scripts as root (except for system package installation)
- Ensure user has sudo privileges

### Cross-Compilation Errors
- Verify ARM toolchain: `arm-linux-gnueabihf-gcc --version`
- Check Rust targets: `rustup target list --installed | grep armv7`
- Run `./test_cross_compilation.sh` to verify environment

### GPS API Issues
- Check API endpoint `http://192.168.1.1:8080/api/v1/gps/` and device connectivity
- Verify GPS data is being recorded in the web interface
- Check device logs for GPS correlation errors

## Support

For issues:

1. **Check the troubleshooting section** in this guide
2. **Run environment verification**: `./test_cross_compilation.sh`
3. **Use Docker environment** for isolated testing: `./docker-build.sh shell`
4. **Check build logs** for specific error messages
5. **Refer to documentation**: See `DOCUMENTATION_INDEX.md` for complete guides

## Related Documentation

- **[BUILD_GUIDE.md](BUILD_GUIDE.md)** - Comprehensive build instructions with cross-compilation fixes
- **[docker-build/DOCKER_BUILD_GUIDE.md](docker-build/DOCKER_BUILD_GUIDE.md)** - Complete Docker environment guide
- **[GPS_API_DOCUMENTATION.md](GPS_API_DOCUMENTATION.md)** - Complete GPS API reference
- **[README_ENHANCED.md](README_ENHANCED.md)** - Project overview and features
- **[DOCUMENTATION_INDEX.md](DOCUMENTATION_INDEX.md)** - Complete documentation index
