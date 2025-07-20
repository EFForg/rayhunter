# CHANGELOG - Enhanced Rayhunter Fork

## [0.4.0-enhanced] - 2025-07-01

### 🚀 Major Enhancements

#### GPS Integration System
- **NEW** GPS REST API for external coordinate submission
- **ADDED** Real-time GPS coordinate correlation with cellular captures
- **ENHANCED** Mobile app compatibility (GPS2REST-Android)
- **ADDED** Multiple export formats (CSV, JSON, GPX)
- **NEW** Per-scan GPS files with automatic timestamp correlation
- **ADDED** External GPS device support via API endpoints

#### GPS API Features
- **Endpoint**: `GET|POST /api/v1/gps/{latitude},{longitude}`
- **GPS2REST-Android Compatibility**: GET requests for mobile app integration
- **Automatic Timestamp Recording**: Server-side timestamp generation
- **Data Validation**: Coordinate range and format validation
- **Dual Storage**: CSV and JSON formats for flexibility

#### Docker Environment Support
- **NEW** Complete isolated build environment with Ubuntu 22.04
- **ADDED** Persistent storage that survives container restarts
- **ENHANCED** Full USB device access for direct deployment
- **ADDED** Pre-configured ARM cross-compilation toolchain
- **NEW** adb support for device communication
- **ADDED** 3-step automated build process

#### Enhanced Build System
- **FIXED** Cross-compilation ARM linker errors
- **NEW** Cross-compilation verification script (`test_cross_compilation.sh`)
- **ENHANCED** All build scripts with proper host/target separation
- **ADDED** Environment detection and automatic setup
- **NEW** Multiple setup options (Docker, Ubuntu automated, local deps, manual)

#### Cellular Data Extraction (3x Coverage Expansion)
- **ADDED** 22 new log codes for comprehensive cellular monitoring
- **EXPANDED** from 11 to 33 total log codes (300% increase)
- **ENHANCED** Multi-RAT support (2G/3G/4G/5G)

#### New Log Code Categories
- **LTE/4G Enhancements**:
  - `LOG_LTE_ML1_SERVING_CELL_INFO` (0xb0e4)
  - `LOG_LTE_ML1_NEIGHBOR_MEASUREMENTS` (0xb0e1)
  - `LOG_LTE_ML1_INTER_FREQ_MEAS` (0xb0e6)
  - `LOG_LTE_ML1_INTER_RAT_MEAS` (0xb0e7)
  - `LOG_LTE_RRC_CELL_INFO` (0xb0c2)
  - `LOG_LTE_RRC_STATE` (0xb0c3)

- **GSM/2G Enhancements**:
  - `LOG_GSM_L1_CELL_ID` (0x513a)
  - `LOG_GSM_RR_CELL_INFORMATION` (0x513b)
  - `LOG_GSM_L1_BURST_METRICS` (0x5134)
  - `LOG_GSM_POWER_SCAN` (0x5139)

- **WCDMA/3G Enhancements**:
  - `LOG_WCDMA_SERVING_CELL_INFO` (0x412a)
  - `LOG_WCDMA_NEIGHBOR_CELL_INFO` (0x412b)
  - `LOG_WCDMA_CELL_ID` (0x4127)

- **NAS/Registration Enhancements**:
  - `LOG_NAS_TRACKING_AREA_UPDATE` (0x7144)
  - `LOG_NAS_LOCATION_UPDATE` (0x7142)
  - `LOG_NAS_ATTACH_REQUEST` (0x7140)

#### Cellular Information Extraction Module
- **NEW** `cellular_info.rs` - Comprehensive cellular parameter extraction
- **ADDED** Support for:
  - MCC/MNC (Mobile Country/Network Code)
  - LAC/TAC (Location/Tracking Area Code)
  - Cell ID and Physical Cell Identity (PCI)
  - eNodeB ID and Sector information
  - Signal metrics (RSRP, RSRQ, SINR)
  - Neighbor cell analysis

#### Enhanced Data Structures
```rust
// NEW: Comprehensive cellular network information
pub struct CellularNetworkInfo {
    pub timestamp: DateTime<FixedOffset>,
    pub rat: RadioAccessTechnology,
    pub plmn_info: Option<PlmnInfo>,
    pub cell_info: Option<CellInfo>,
    pub location_info: Option<LocationInfo>,
    pub signal_info: Option<SignalInfo>,
    pub neighbor_cells: Vec<NeighborCellInfo>,
}
```

#### GSMTAP Parser Enhancements
- **ADDED** `parse_with_cellular_info()` function
- **ENHANCED** Packet processing with cellular parameter extraction
- **FIXED** Move semantics for LTE RRC packets

#### Diagnostic Device Enhancements
- **EXPANDED** `LOG_CODES_FOR_RAW_PACKET_LOGGING` array
- **INCREASED** from 11 to 33 entries (200% expansion)
- **OPTIMIZED** for MDM9225 and similar Qualcomm chipsets

### 🔧 Technical Improvements

#### Build System
- **ADDED** CSV dependency for data export
- **ENHANCED** Cross-compilation for ARM targets
- **IMPROVED** Web UI integration
- **NEW** Docker environment with persistent storage
- **ADDED** USB device access in Docker containers
- **ENHANCED** Environment detection and automatic setup

#### Code Quality
- **FIXED** Unused variable warnings
- **RESOLVED** Move semantics issues
- **IMPROVED** Error handling and logging
- **FIXED** Cross-compilation ARM linker errors
- **ADDED** GPS API error handling and validation

#### Dependencies
- **ADDED** `csv = "1.3"` for enhanced data export
- **UPDATED** Existing dependencies for compatibility
- **ADDED** GPS correlation and API dependencies

### 📊 Performance Improvements
- **3x increase** in cellular data capture coverage
- **Enhanced** real-time processing capabilities
- **Optimized** memory usage for continuous monitoring
- **IMPROVED** GPS data correlation performance
- **ENHANCED** Docker build environment efficiency

### 🛡️ Security Enhancements
- **IMPROVED** IMSI catcher detection algorithms
- **ENHANCED** Neighbor cell analysis
- **ADDED** Signal anomaly detection
- **NEW** GPS data validation and sanitization
- **ENHANCED** API endpoint security

### 📚 Documentation
- **CREATED** `README_ENHANCED.md` with comprehensive usage guide
- **ADDED** Installation instructions for enhanced features
- **DOCUMENTED** All new cellular parameters and extraction methods
- **ADDED** Device-specific installation guides
- **NEW** Docker build guide with 3-step process
- **ADDED** GPS API documentation and examples
- **ENHANCED** Build system documentation with cross-compilation fixes
- **UPDATED** All markdown files with current information

### 🧪 Testing
- **VERIFIED** Compilation on ARM targets
- **TESTED** Enhanced log code coverage
- **VALIDATED** Cellular parameter extraction
- **TESTED** GPS API functionality and integration
- **VERIFIED** Docker environment and USB access
- **VALIDATED** Cross-compilation fixes

### 🐳 Docker Environment Features
- **Ubuntu 22.04** container with persistent home directory
- **rayhunter user** with password `thehunted` and sudo access
- **All toolchains** installed locally in user space
- **Persistent storage** for source code and build artifacts
- **Full USB device access** for direct deployment
- **GPS integration** with REST API endpoints

### 📱 GPS Integration Features
- **Real-time GPS coordinate submission** via REST API
- **Mobile app compatibility** (GPS2REST-Android)
- **Multiple export formats** (CSV, JSON, GPX)
- **Per-scan GPS files** with automatic timestamp correlation
- **External GPS device support** via API endpoints
- **Automatic correlation** with cellular captures
- **Location-based analysis** for cell tower mapping

---

## Original Rayhunter v0.4.0 Features
- QMDL log parsing and analysis
- Web-based user interface
- Multi-device support (Orbic, TP-Link, Wingtech)
- Real-time cellular monitoring
- IMSI catcher detection capabilities

---

## Installation Notes
1. **Web UI must be built before firmware compilation**
2. **ARM cross-compilation target required**: `armv7-unknown-linux-musleabihf`
3. **Device-specific rooting may be required**
4. **Enhanced features require re-installation of daemon**
5. **Docker environment recommended for new users**
6. **GPS API endpoints automatically deployed with firmware**

## Breaking Changes
- None - fully backward compatible with original Rayhunter v0.4.0

## Migration Guide
Users of original Rayhunter can upgrade seamlessly:
1. Build enhanced firmware using provided instructions
2. Install using existing installation methods
3. Enjoy 3x expanded cellular data coverage automatically
4. Access GPS API endpoints for location correlation
5. Use Docker environment for simplified builds

## Quick Start Options

### Docker Environment (Recommended for New Users)
```bash
./docker-build.sh up && ./docker-build.sh shell
./setup_ubuntu_ci.sh && ./fetch_source.sh && ./build_and_deploy.sh
```

### Ubuntu Automated Setup
```bash
./setup_ubuntu_ci.sh && ./build_all.sh && ./deploy.sh
```

### Local Dependencies (No Root Required)
```bash
./setup_local_deps.sh && ./build_all.sh && ./deploy.sh
```

---

**Enhanced by**: @drinkingc0ffee  
**Base Version**: Rayhunter v0.4.0 (EFF)  
**Enhancement Date**: July 1, 2025  
**Repository**: https://github.com/drinkingc0ffee/rayhunter-enhanced
