# Rayhunter Enhanced 🔬📡

## IMSI Catcher Detection and Cellular Monitoring System

Rayhunter Enhanced is an advanced cellular monitoring and IMSI catcher detection system designed for security researchers, network analysts, and privacy advocates. This enhanced version provides comprehensive cellular data extraction capabilities with expanded coverage and advanced analysis features.

### 🎯 What is Rayhunter?

Rayhunter helps detect and analyze potential IMSI catchers (cell-site simulators) by monitoring cellular network behavior and identifying suspicious patterns. It captures detailed cellular network information to help users understand their mobile connectivity and detect potential surveillance devices.

### 🚀 Key Features

#### 🛡️ **IMSI Catcher Detection**
- **Rogue cell detection** through neighbor cell analysis
- **Signal anomaly identification** with multi-metric monitoring
- **Location tracking prevention** via TAC/LAC monitoring
- **Fake base station identification** using cellular fingerprinting

#### 📊 **Comprehensive Cellular Data Extraction**
- **Network Identifiers**: MCC/MNC (Mobile Country/Network Code)
- **Location Information**: LAC/TAC (Location/Tracking Area Code), Cell ID/PCI
- **Cell Details**: eNodeB ID, Sector information, Physical Cell Identity
- **Signal Metrics**: RSRP, RSRQ, SINR signal strength measurements
- **Multi-Technology Support**: 2G/3G/4G/5G network analysis
- **Neighbor Cell Tracking**: Monitor surrounding cell towers

#### 📍 **GPS Integration**
- **Real-time location capture** with cellular captures
- **External GPS support** via REST API endpoints
- **Mobile app compatibility** (GPS2REST-Android)
- **Multiple export formats** (CSV, JSON, GPX)
- **Per-scan GPS files** with automatic timestamp correlation

#### 📡 **Web Interface**
- **Real-time monitoring** dashboard
- **Data download** in multiple formats (PCAP, QMDL, ZIP)
- **Analysis tools** for captured data
- **Mobile-responsive** design

### 🔧 System Requirements

#### **Supported Hardware**
- **Primary**: Orbic RC400L mobile hotspot
- **Secondary**: TP-Link M7310/M7350 devices
- **Chipset**: Qualcomm MDM9225 and compatible modems
- **Connection**: USB or ADB access to device

#### **Development Environment**
- **Rust**: Latest stable toolchain
- **Target**: ARM cross-compilation (`armv7-unknown-linux-musleabihf`)
- **Node.js**: v16+ and npm (for web interface)
- **ADB**: Android Debug Bridge for device communication

#### **Operating System**
- **Linux**: Primary development platform (Ubuntu 20.04+, Docker)
- **macOS**: Supported for development
- **Windows**: Supported via WSL or Docker

### 📋 Installation

#### **Option 1: Docker Environment (Recommended for New Users)**

The Docker environment provides a complete, isolated build environment with all dependencies pre-configured:

```bash
# Clone the repository
git clone https://github.com/your-repo/rayhunter-enhanced.git
cd rayhunter-enhanced

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

#### **Option 2: Ubuntu Users (Automated Setup)**

For Ubuntu systems, use the automated setup scripts:

```bash
# Clone the repository
git clone https://github.com/your-repo/rayhunter-enhanced.git
cd rayhunter-enhanced

# Set up build environment (one-time setup)
./setup_ubuntu_ci.sh     # Automated setup for CI/CD
# OR
./setup_ubuntu_build_env.sh  # Interactive setup for development

# Build everything and deploy
./build_all.sh && ./deploy.sh
```

#### **Option 3: Local Dependencies (No Root Required)**

Install all dependencies locally without affecting your system:

```bash
# Clone the repository
git clone https://github.com/your-repo/rayhunter-enhanced.git
cd rayhunter-enhanced

# Install all dependencies locally (no root access needed)
./setup_local_deps.sh

# Build everything and deploy
./build_all.sh && ./deploy.sh
```

**Local Dependencies Benefits:**
- ✅ **No root access required**
- ✅ **Isolated environment** - doesn't affect system
- ✅ **Reproducible builds** - exact versions for everyone
- ✅ **Easy cleanup** - just delete `./build_deps` directory

#### **Option 4: Manual Installation**

For other systems or custom setups:

```bash
# Clone the repository
git clone https://github.com/your-repo/rayhunter-enhanced.git
cd rayhunter-enhanced

# 1. Setup Rust cross-compilation
rustup target add armv7-unknown-linux-musleabihf

# 2. Install ARM cross-compilation toolchain
# Ubuntu/Debian: sudo apt install gcc-arm-linux-gnueabihf
# macOS: brew install arm-linux-gnueabihf-gcc
# Other systems: see BUILD_GUIDE.md

# 3. Build web interface
cd bin/web && npm install && npm run build && cd ../..

# 4. Build for device
cargo build --profile firmware --target armv7-unknown-linux-musleabihf

# 5. Deploy to device via ADB
./deploy.sh
```

### 🧪 Verify Build Environment

Test that your cross-compilation environment is working correctly:

```bash
# Test cross-compilation setup
./test_cross_compilation.sh

# This verifies:
# - Build scripts compile for host architecture
# - Target binaries compile for ARM
# - No compiler conflicts
```

### 🔍 How It Works

#### **Data Collection**
Rayhunter interfaces with the cellular modem's diagnostic interface to capture detailed network information, including:

- **Cell tower information** and network parameters
- **Signal strength measurements** and quality metrics
- **Network registration events** and location updates
- **Neighbor cell discoveries** and handoff information

#### **Analysis Engine**
The system analyzes captured data to detect:

1. **Inconsistent network behavior** that may indicate rogue base stations
2. **Signal anomalies** that deviate from expected patterns
3. **Location tracking attempts** through area code monitoring
4. **Suspicious cell configurations** not matching known networks

#### **Output Formats**
- **PCAP files**: Network packet captures for Wireshark analysis
- **QMDL files**: Raw cellular diagnostic logs
- **CSV exports**: Structured data for spreadsheet analysis
- **JSON data**: Machine-readable format for custom analysis

### 📱 GPS API Usage

#### **Submit GPS Coordinates**
```bash
# Using curl (GET method - GPS2REST-Android compatible)
curl "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"

# Using curl (POST method)
curl -X POST "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"
```

#### **Download GPS Data**
```bash
# Get GPS data for a recording session
curl "http://192.168.1.1:8080/api/gps/1720080123/csv" -o gps_data.csv
```

### 🎯 Use Cases

#### **Security Research**
- **IMSI catcher detection** in high-risk environments
- **Network security auditing** for organizations
- **Mobile privacy assessment** for individuals

#### **Network Analysis**
- **Cell tower mapping** and coverage analysis
- **Signal quality assessment** for specific locations
- **Network performance monitoring** during travel

#### **Educational Purposes**
- **Cellular technology education** with real-world data
- **Security awareness training** about mobile threats
- **Research projects** on mobile network security

### 🛠️ Build System Enhancements

#### **Cross-Compilation Improvements**
The build system has been enhanced with comprehensive cross-compilation fixes:

- ✅ **Proper host/target separation** - Build scripts compile for host (x86_64), target binaries for ARM
- ✅ **PATH management** - Ensures correct compiler resolution
- ✅ **Environment isolation** - Prevents ARM compiler from interfering with build scripts
- ✅ **Automatic verification** - Tests cross-compilation setup before building

#### **Available Build Scripts**
- **`./build_all.sh`** - Comprehensive build with environment detection and verification
- **`./make.sh`** - Quick build for iterative development
- **`./clean.sh`** - Clean all build artifacts and prepare for fresh build
- **`./deploy.sh`** - Deploy to device via adb
- **`./test_cross_compilation.sh`** - Verify cross-compilation environment

#### **Environment Setup Scripts**
- **`./setup_ubuntu_ci.sh`** - Automated Ubuntu setup for CI/CD
- **`./setup_ubuntu_build_env.sh`** - Interactive Ubuntu setup for development
- **`./setup_local_deps.sh`** - Local dependency installation (no root required)
- **`./docker-build.sh`** - Docker environment management

### 🐛 Troubleshooting

#### **Cross-Compilation Issues**
If you encounter ARM linker errors:
```bash
# Test your environment
./test_cross_compilation.sh

# Clean and rebuild
./clean.sh && ./build_all.sh
```

#### **Environment Issues**
If commands are not found:
```bash
# For Docker environment
./docker-build.sh shell

# For local setup
./setup_local_deps.sh    # No root required
# OR
./setup_ubuntu_ci.sh     # System-wide (requires sudo)
```

#### **Build Failures**
For persistent build issues:
```bash
# Clean everything
./clean.sh

# Test environment
./test_cross_compilation.sh

# Rebuild
./build_all.sh
```

### 🔐 Privacy and Ethics

#### **Privacy Protection**
- **Local processing only** - no cloud connectivity
- **User-controlled data** retention and export
- **Open source transparency** for security verification

#### **Responsible Use**
This tool is intended for:
- ✅ **Security research and education**
- ✅ **Network analysis and troubleshooting**  
- ✅ **Personal privacy protection**
- ✅ **Academic research with proper consent**

**NOT intended for:**
- ❌ Illegal surveillance or interception
- ❌ Unauthorized monitoring of others
- ❌ Commercial espionage
- ❌ Violation of privacy laws

### 📚 Documentation

- **[BUILD_GUIDE.md](BUILD_GUIDE.md)** - Comprehensive build instructions with cross-compilation fixes
- **[DOCKER_BUILD_GUIDE.md](DOCKER_BUILD_GUIDE.md)** - Complete Docker environment guide
- **[UBUNTU_SETUP.md](UBUNTU_SETUP.md)** - Ubuntu-specific automated setup
- **[GPS_API_DOCUMENTATION.md](GPS_API_DOCUMENTATION.md)** - Complete GPS API reference
- **[doc/installing-from-source.md](doc/installing-from-source.md)** - Detailed setup instructions
- **[doc/supported-devices.md](doc/supported-devices.md)** - Hardware compatibility information
- **[doc/analyzing-a-capture.md](doc/analyzing-a-capture.md)** - How to interpret captured data
- **[doc/configuration.md](doc/configuration.md)** - System configuration options
- **[DOCUMENTATION_INDEX.md](DOCUMENTATION_INDEX.md)** - Complete documentation index

### 🤝 Contributing

We welcome contributions to improve Rayhunter Enhanced:

- **Device support**: Add compatibility for new hardware
- **Analysis algorithms**: Improve detection capabilities
- **User interface**: Enhance web interface and usability
- **Documentation**: Help others understand and use the system
- **Build system**: Improve cross-compilation and environment setup

### ⚖️ Legal Notice

This software is provided for educational and research purposes. Users are responsible for complying with all applicable laws and regulations in their jurisdiction. The authors are not responsible for any misuse of this software.

### 🔗 Links

- **Original Rayhunter**: [https://github.com/EFForg/rayhunter](https://github.com/EFForg/rayhunter)
- **GPS2REST-Android**: Compatible mobile app for GPS coordinate submission
- **Documentation**: See `DOCUMENTATION_INDEX.md` for complete documentation library
