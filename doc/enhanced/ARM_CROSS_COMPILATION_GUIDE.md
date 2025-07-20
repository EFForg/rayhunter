# ARM Cross-Compilation Troubleshooting Guide

## Problem
You're getting the error:
```
❌ Cross-compilation test failed
   This usually means the ARM toolchain is not properly configured
```

## Solutions

### Option 1: Quick Fix - Build for Host Architecture (Recommended for Development)

If you just want to build and test the application, use the host architecture build:

```bash
# Build for your current system (x86_64)
./build_host.sh
```

This will:
- Build all components for your current architecture
- Skip ARM cross-compilation entirely
- Create working binaries for testing and development
- Be much faster and simpler

**Use this if you're:**
- Developing and testing features
- Running in a development environment
- Don't need to deploy to ARM devices immediately

### Option 2: Full ARM Toolchain Setup (For Production Deployment)

If you need to build for ARM devices (Android phones), set up the complete toolchain:

```bash
# Run as root or with sudo
sudo ./setup_arm_toolchain.sh
```

This will:
- Install ARM GCC toolchain
- Configure Rust for ARM targets
- Set up all build dependencies
- Create proper environment variables

**Use this if you're:**
- Deploying to ARM devices
- Building production releases
- Need ARM-specific optimizations

### Option 3: Manual ARM Toolchain Installation

If the automated script doesn't work, install manually:

```bash
# Update package list
sudo apt-get update

# Install ARM toolchain
sudo apt-get install -y \
    gcc-arm-linux-gnueabihf \
    g++-arm-linux-gnueabihf \
    binutils-arm-linux-gnueabihf \
    libc6-dev-armhf-cross \
    libstdc++6-armhf-cross \
    musl-tools \
    musl-dev

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Add ARM targets
rustup target add armv7-unknown-linux-musleabihf
rustup target add armv7-unknown-linux-gnueabihf

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo bash -
sudo apt-get install -y nodejs

# Install ADB
sudo apt-get install -y android-tools-adb
```

## Common Issues and Fixes

### Issue 1: "arm-linux-gnueabihf-gcc not found"

**Solution:**
```bash
sudo apt-get install -y gcc-arm-linux-gnueabihf
```

### Issue 2: "musl target not found"

**Solution:**
```bash
sudo apt-get install -y musl-tools musl-dev
rustup target add armv7-unknown-linux-musleabihf
```

### Issue 3: "Node.js not found"

**Solution:**
```bash
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo bash -
sudo apt-get install -y nodejs
```

### Issue 4: "adb not found"

**Solution:**
```bash
sudo apt-get install -y android-tools-adb
```

### Issue 5: "Permission denied" errors

**Solution:**
```bash
# Run setup script with sudo
sudo ./setup_arm_toolchain.sh

# Or run individual commands with sudo
sudo apt-get install -y [package-name]
```

### Issue 6: "cc resolves to ARM compiler"

**Solution:**
The build scripts automatically fix this, but if you have issues:

```bash
# Ensure host compiler comes first in PATH
export PATH="/usr/bin:/bin:$PATH"

# Unset global compiler variables
unset CC CXX AR LD CFLAGS CXXFLAGS LDFLAGS LINK
```

## Verification Steps

After setup, verify everything works:

```bash
# Test ARM compiler
arm-linux-gnueabihf-gcc --version

# Test Rust ARM target
cargo check --target armv7-unknown-linux-musleabihf

# Test Node.js
node --version
npm --version

# Test ADB
adb version
```

## Environment Variables

The setup creates `~/.rayhunter_build_env` with these variables:

```bash
# ARM Cross-compilation variables
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_CC=arm-linux-gnueabihf-gcc
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_CXX=arm-linux-gnueabihf-g++
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_AR=arm-linux-gnueabihf-ar

# Host compiler variables
export CC_x86_64_unknown_linux_gnu=gcc
export CXX_x86_64_unknown_linux_gnu=g++
export AR_x86_64_unknown_linux_gnu=ar
```

## Docker-Specific Notes

If you're running in Docker:

1. **Use a base image with build tools:**
   ```dockerfile
   FROM ubuntu:22.04
   RUN apt-get update && apt-get install -y build-essential curl
   ```

2. **Run setup as root:**
   ```bash
   sudo ./setup_arm_toolchain.sh
   ```

3. **Or use the host build for development:**
   ```bash
   ./build_host.sh
   ```

## Quick Commands

### For Development (Host Build)
```bash
./build_host.sh
```

### For Production (ARM Build)
```bash
sudo ./setup_arm_toolchain.sh
./build_and_deploy.sh
```

### Test Current Setup
```bash
./test_gps_correlation.sh
```

## Still Having Issues?

1. **Check your Ubuntu version:**
   ```bash
   lsb_release -a
   ```

2. **Check available packages:**
   ```bash
   apt-cache search arm-linux-gnueabihf
   ```

3. **Check Rust targets:**
   ```bash
   rustup target list --installed
   ```

4. **Check PATH:**
   ```bash
   echo $PATH
   which gcc
   which arm-linux-gnueabihf-gcc
   ```

5. **Try the host build first:**
   ```bash
   ./build_host.sh
   ```

## Summary

- **For development/testing**: Use `./build_host.sh`
- **For ARM deployment**: Use `sudo ./setup_arm_toolchain.sh` then `./build_and_deploy.sh`
- **If in doubt**: Start with the host build to verify everything else works 