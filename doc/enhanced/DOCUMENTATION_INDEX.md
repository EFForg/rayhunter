# Rayhunter Enhanced - Documentation Index

## 📚 Complete Documentation Library

### Core Documentation
- **[README_ENHANCED.md](README_ENHANCED.md)** - Main project overview and enhanced features
- **[BUILD_GUIDE.md](BUILD_GUIDE.md)** - Comprehensive build instructions with cross-compilation fixes
- **[CHANGELOG_ENHANCED.md](CHANGELOG_ENHANCED.md)** - Version history and changes
- **[OFFLINE_ANALYSIS.md](OFFLINE_ANALYSIS.md)** - Offline cellular data analysis workflow

### Setup & Environment Documentation
- **[docker-build/DOCKER_BUILD_GUIDE.md](docker-build/DOCKER_BUILD_GUIDE.md)** - Complete Docker environment guide (3-step process)
- **[UBUNTU_SETUP.md](UBUNTU_SETUP.md)** - Ubuntu-specific automated setup guide
- **[test_cross_compilation.sh](test_cross_compilation.sh)** - Cross-compilation environment verification script

### API Documentation
- **[GPS_API_DOCUMENTATION.md](GPS_API_DOCUMENTATION.md)** - Complete GPS REST API reference
- **[GPS_API_QUICK_REFERENCE.md](GPS_API_QUICK_REFERENCE.md)** - Quick GPS API usage guide  
- **[GPS_CORRELATION_DOCUMENTATION.md](GPS_CORRELATION_DOCUMENTATION.md)** - GPS correlation system guide

### Original Documentation (Upstream)
- **[README.md](README.md)** - Original Rayhunter documentation
- **[doc/](doc/)** - Comprehensive installation and usage guides
  - [installation.md](doc/installation.md) - Installation instructions
  - [using-rayhunter.md](doc/using-rayhunter.md) - Basic usage guide
  - [configuration.md](doc/configuration.md) - Configuration options
  - [supported-devices.md](doc/supported-devices.md) - Device compatibility
  - [heuristics.md](doc/heuristics.md) - IMSI catcher detection methods

### Device-Specific Guides
- **[doc/orbic.md](doc/orbic.md)** - Orbic RC400L setup and usage
- **[doc/tplink-m7310.md](doc/tplink-m7310.md)** - TP-Link M7310 guide
- **[doc/tplink-m7350.md](doc/tplink-m7350.md)** - TP-Link M7350 guide
- **[doc/wingtech-ct2mhs01.md](doc/wingtech-ct2mhs01.md)** - Wingtech device guide

### Quick References

#### 🚀 Quick Start Options

**Docker Environment (Recommended for New Users):**
```bash
./docker-build.sh up && ./docker-build.sh shell
./setup_ubuntu_ci.sh && ./fetch_source.sh && ./build_and_deploy.sh
```

**Ubuntu Automated Setup:**
```bash
./setup_ubuntu_ci.sh && ./build_all.sh && ./deploy.sh
```

**Local Dependencies (No Root Required):**
```bash
./setup_local_deps.sh && ./build_all.sh && ./deploy.sh
```

#### 🔧 Build & Development
- **[BUILD_GUIDE.md](BUILD_GUIDE.md)** - Complete build instructions with cross-compilation fixes
- **[Docker Environment](docker-build/DOCKER_BUILD_GUIDE.md)** - Isolated build environment
- **[Cross-Compilation Testing](test_cross_compilation.sh)** - Verify build environment
- **[Local Dependencies Setup](setup_local_deps.sh)** - No-root-required setup

#### 🐛 Troubleshooting
- **Cross-Compilation Issues**: Run `./test_cross_compilation.sh` to diagnose
- **Environment Problems**: Use Docker environment or `./setup_local_deps.sh`
- **Build Failures**: Clean with `./clean.sh` then rebuild with `./build_all.sh`
- **Device Issues**: Check `adb devices` and USB debugging settings
- **Docker USB Access**: Use `./restart_container_with_usb.sh` for device access

#### 📡 API References
- **[REST Endpoints](GPS_API_DOCUMENTATION.md#api-endpoint)** - GPS coordinate submission
- **[Response Formats](GPS_API_DOCUMENTATION.md#response-format)** - API response structure
- **[Integration Examples](GPS_API_DOCUMENTATION.md#integration-examples)** - Code examples

#### 🛡️ Security & Analysis
- **[IMSI Catcher Detection](README_ENHANCED.md#-imsi-catcher-detection-features)** - Enhanced security features
- **[Offline Analysis](OFFLINE_ANALYSIS.md)** - Post-capture processing workflow
- **[Privacy Considerations](GPS_API_DOCUMENTATION.md#security-considerations)** - Data protection

### Development & Contributing
- **[Enhanced Build System](BUILD_GUIDE.md#-build-scripts)** - Modern build scripts with cross-compilation fixes
- **[Environment Setup Scripts](BUILD_GUIDE.md#environment-setup-scripts)** - Automated dependency installation
- **[Docker Development](docker-build/DOCKER_BUILD_GUIDE.md)** - Container-based development workflow
- **[Contributing Guidelines](README_ENHANCED.md#-contributing)** - How to contribute

---

## 📖 Documentation Quick Links

| Topic | Document | Description |
|-------|----------|-------------|
| **Getting Started** | [README_ENHANCED.md](README_ENHANCED.md) | Project overview and installation options |
| **Docker Environment** | [docker-build/DOCKER_BUILD_GUIDE.md](docker-build/DOCKER_BUILD_GUIDE.md) | Simple 3-step Docker build process |
| **Build Instructions** | [BUILD_GUIDE.md](BUILD_GUIDE.md) | Comprehensive build guide with cross-compilation fixes |
| **Ubuntu Setup** | [UBUNTU_SETUP.md](UBUNTU_SETUP.md) | Automated Ubuntu environment setup |
| **Cross-Compilation Test** | [test_cross_compilation.sh](test_cross_compilation.sh) | Verify build environment is working |
| **GPS Integration** | [GPS_API_DOCUMENTATION.md](GPS_API_DOCUMENTATION.md) | Complete GPS API reference |
| **Device Setup** | [doc/supported-devices.md](doc/supported-devices.md) | Device-specific instructions |
| **Configuration** | [doc/configuration.md](doc/configuration.md) | Advanced configuration options |
| **Usage Guide** | [doc/using-rayhunter.md](doc/using-rayhunter.md) | Basic operation instructions |
| **Security Features** | [doc/heuristics.md](doc/heuristics.md) | IMSI catcher detection methods |
| **Offline Analysis** | [OFFLINE_ANALYSIS.md](OFFLINE_ANALYSIS.md) | Post-capture data processing |
| **Changes & Updates** | [CHANGELOG_ENHANCED.md](CHANGELOG_ENHANCED.md) | Version history and improvements |

---

## 🛠️ Enhanced Build System

### Key Improvements (Latest Version)
- ✅ **Cross-Compilation Fixes** - Proper host/target separation eliminates ARM linker errors
- ✅ **Docker Environment** - Complete isolated build environment with persistent storage and USB access
- ✅ **Environment Auto-Detection** - Automatically finds and uses local or system dependencies
- ✅ **Build Verification** - Tests cross-compilation setup before building
- ✅ **Multiple Setup Options** - Docker, Ubuntu automated, local deps, or manual
- ✅ **GPS Integration** - Comprehensive GPS API and correlation features

### Available Scripts
- **`./docker-build.sh`** - Docker environment management (build, up, shell, down, clean)
- **`./build_all.sh`** - Main build script with cross-compilation fixes and environment detection
- **`./make.sh`** - Quick build script for iterative development
- **`./clean.sh`** - Enhanced cleanup script with npm cache cleaning
- **`./deploy.sh`** - Device deployment via adb with GPS API endpoints
- **`./test_cross_compilation.sh`** - **NEW**: Verify cross-compilation environment
- **`./setup_ubuntu_ci.sh`** - Automated Ubuntu setup for CI/CD
- **`./setup_ubuntu_build_env.sh`** - Interactive Ubuntu setup for development
- **`./setup_local_deps.sh`** - Local dependency installation (no root required)
- **`./fetch_source.sh`** - Download latest source code (Docker environment)
- **`./restart_container_with_usb.sh`** - **NEW**: Restart Docker container with enhanced USB access

### Environment Options
1. **Docker** (Recommended): `./docker-build.sh up && ./docker-build.sh shell`
2. **Ubuntu System**: `./setup_ubuntu_ci.sh` (requires sudo)
3. **Local Dependencies**: `./setup_local_deps.sh` (no root required)
4. **Manual**: See [BUILD_GUIDE.md](BUILD_GUIDE.md) for custom setup

---

## 🔄 Recent Major Enhancements

### Build System Overhaul (Latest)
- **FIXED**: Cross-compilation ARM linker errors (`/usr/arm-linux-gnueabihf/bin/ld: unrecognised emulation mode: elf_x86_64`)
- **NEW**: Docker environment with 3-step build process and USB device access
- **NEW**: Cross-compilation verification script (`test_cross_compilation.sh`)
- **ENHANCED**: All build scripts with proper host/target separation
- **IMPROVED**: Environment detection and automatic setup
- **NEW**: GPS integration with REST API endpoints

### GPS REST API (v0.4.0-enhanced)
- **NEW**: External GPS coordinate submission via REST API
- **NEW**: Real-time location correlation with cellular captures
- **NEW**: Dual-format data storage (CSV + JSON)
- **NEW**: Mobile app integration capabilities (GPS2REST-Android)
- **NEW**: Multiple export formats (CSV, JSON, GPX)
- **NEW**: Per-scan GPS files with automatic timestamp correlation

### Enhanced Cellular Extraction (v0.4.0-enhanced)
- **EXPANDED**: 3x more log codes for comprehensive cellular data
- **NEW**: Advanced cellular parameter extraction (MCC, MNC, LAC, TAC, Cell ID)
- **NEW**: Multi-RAT support (2G/3G/4G/5G)
- **NEW**: Offline analysis workflow for OpenCellID integration
- **NEW**: Neighbor cell tracking and analysis

### Docker Environment (Latest)
- **NEW**: Complete isolated build environment with Ubuntu 22.04
- **NEW**: Persistent storage that survives container restarts
- **NEW**: Full USB device access for direct deployment
- **NEW**: Pre-configured ARM cross-compilation toolchain
- **NEW**: adb support for device communication
- **NEW**: 3-step automated build process

---

## 🐛 Common Issues & Solutions

### Cross-Compilation Errors
**Issue**: `unrecognised emulation mode: elf_x86_64`
**Solution**: Run `./test_cross_compilation.sh` to verify environment, then `./clean.sh && ./build_all.sh`

### Missing Dependencies
**Issue**: `cargo: command not found` or missing tools
**Solution**: Use Docker (`./docker-build.sh shell`) or local setup (`./setup_local_deps.sh`)

### Build Failures
**Issue**: Compilation or linking errors
**Solution**: Clean environment (`./clean.sh`) and verify setup (`./test_cross_compilation.sh`)

### Device Connection
**Issue**: Device not accessible via adb
**Solution**: Check `adb devices`, enable USB debugging, authorize connection

### Docker USB Access
**Issue**: Docker container can't access USB devices
**Solution**: Use `./restart_container_with_usb.sh` to restart container with enhanced USB access

### GPS API Issues
**Issue**: GPS coordinates not being recorded
**Solution**: Check API endpoint `http://192.168.1.1:8080/api/v1/gps/` and device connectivity

---

## 📱 GPS Integration Features

### API Endpoints
- **GET/POST**: `http://192.168.1.1:8080/api/v1/gps/{lat},{lon}` - Submit GPS coordinates
- **GET**: `http://192.168.1.1:8080/api/gps/{session_id}/csv` - Download GPS data
- **GET**: `http://192.168.1.1:8080/api/gps/{session_id}/json` - Download GPS data (JSON)

### Mobile App Integration
- **GPS2REST-Android**: Compatible mobile app for automatic GPS coordinate submission
- **Real-time tracking**: Continuous location updates during cellular captures
- **Automatic correlation**: GPS data automatically linked with cellular captures

### Export Formats
- **CSV**: Standard spreadsheet format for analysis
- **JSON**: Machine-readable format for custom processing
- **GPX**: GPS Exchange Format for mapping applications

---

*For the most up-to-date information, always refer to the individual documentation files linked above.*
