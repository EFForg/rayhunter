# Rayhunter Enhanced 🦎

**Enhanced fork of [EFF's Rayhunter](https://github.com/EFForg/rayhunter) with GPS integration, comprehensive cellular data extraction, and advanced security analysis.**

## 🚀 **Enhanced Features**

### 📍 **GPS Integration**
- **Real-time GPS correlation** with cellular events
- **GPS timestamp tracking** for precise threat location
- **GPS accuracy metrics** for data quality assessment
- **Location-based threat analysis** and historical incident tracking

### 📊 **Complete SCAT Compatibility**
- **All SCAT fields** included in NDJSON export
- **Signal quality measurements** (RSRP, RSRQ, RSSI, SINR, SNR)
- **Neighbor cell information** from SIB4-SIB7
- **Network identification** (MCC, MNC, LAC, Cell ID, TAC)

### 🛡️ **Enhanced Security Analysis**
- **All 5 EFF suspicious cell algorithms** integrated
- **Threat level assessment** (None, Low, Medium, High, Critical)
- **Attack type classification** (ImsiCatcher, Stingray, ManInTheMiddle, etc.)
- **Confidence scoring** (0.0 to 1.0) for threat assessment
- **Security recommendations** and mitigation strategies

### 📁 **Advanced Data Export**
- **NDJSON format** with Unix timestamps
- **Two export files**: Complete cellular data + Security threats only
- **GPS correlation** in all exports
- **Comprehensive neighbor cell data** with signal quality

### 🔧 **Deployment Automation**
- **ARM hard float** cross-compilation support
- **Automated deployment** to ARM devices
- **GPS API integration** with REST endpoints
- **Enhanced configuration** management

## 📋 **What's Enhanced from Original**

| Feature | Original | Enhanced |
|---------|----------|----------|
| **GPS Integration** | ❌ None | ✅ Real-time correlation |
| **SCAT Compatibility** | ❌ Basic | ✅ Complete field mapping |
| **Security Analysis** | ✅ 5 algorithms | ✅ Enhanced with threat levels |
| **Data Export** | ❌ Basic | ✅ NDJSON with GPS |
| **Neighbor Cells** | ❌ Limited | ✅ SIB4-SIB7 extraction |
| **Signal Quality** | ❌ Basic | ✅ Complete measurements |
| **Deployment** | ❌ Manual | ✅ Automated ARM deployment |

## 🏷️ **Version: v0.4.5**

**Tag:** `v0.4.5`  
**Branch:** `feature/gps-api-integration`  
**Based on:** EFF Rayhunter main branch

## 🔗 **Repository Structure**

```
rayhunter-enhanced/
├── lib/src/analysis/
│   ├── cellular_data.rs      # Enhanced cellular data extraction
│   ├── analyzer.rs           # GPS-integrated analysis engine
│   └── [EFF algorithms]      # All original + enhanced security
├── bin/src/
│   ├── server.rs             # GPS API endpoints
│   ├── config.rs             # Enhanced configuration
│   └── daemon.rs             # GPS integration
├── deploy_v0.4.5.sh          # ARM deployment automation
├── NDJSON_COMPARISON.md      # Complete feature comparison
└── DEPLOYMENT_STATUS.md      # Deployment documentation
```

## 🚀 **Quick Start**

### **1. Build for ARM:**
```bash
./build_macos.sh
```

### **2. Deploy to Device:**
```bash
./deploy_v0.4.5.sh
```

### **3. Access Web Interface:**
```
http://localhost:8080
```

## 📊 **NDJSON Output Example**

```json
{
  "timestamp": 1753051049,
  "mcc": 310,
  "mnc": 260,
  "cell_identity": 12345678,
  "gps_location": {
    "latitude": 40.744612,
    "longitude": -74.2524389,
    "gps_timestamp": "2025-07-20T22:37:29Z",
    "accuracy": 5.0,
    "source": "gps_log"
  },
  "security_analysis": {
    "threat_level": "Medium",
    "attack_type": "ImsiCatcher",
    "confidence": 0.75,
    "indicators": ["IMSI request detected"],
    "recommendations": ["Monitor cell behavior"]
  },
  "neighbor_cells": [...],
  "rsrp": -85.5,
  "rsrq": -12.3
}
```

## 🛡️ **EFF Algorithms Included**

1. **IMSI Requested Detection** - NAS IMSI identity requests
2. **Null Cipher Detection** - EEA0 ciphering algorithm
3. **Connection Redirect 2G Downgrade** - Forced 2G redirections
4. **LTE SIB6/7 Downgrade** - SIB6/SIB7 downgrade attempts
5. **IMSI Provided Detection** - IMSI provision in messages

## 📈 **Performance Improvements**

- **+1,482 lines** of enhanced functionality
- **12 files** enhanced with new capabilities
- **100% SCAT compatibility** maintained
- **Real-time GPS correlation** added
- **Automated ARM deployment** implemented

## 🤝 **Contributing**

This is an enhanced fork of the original EFF Rayhunter. For the original project, see:
- **Original:** [https://github.com/EFForg/rayhunter](https://github.com/EFForg/rayhunter)
- **Enhanced Fork:** [https://github.com/drinkingc0ffee/rayhunter](https://github.com/drinkingc0ffee/rayhunter)

## 📄 **License**

Same license as original EFF Rayhunter project.

---

**Enhanced by:** @drinkingc0ffee  
**Original by:** Electronic Frontier Foundation  
**Version:** v0.4.5  
**Status:** Production Ready 🚀 