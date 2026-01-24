# LTE ML1 Serving Cell Measurement (0xB193)

This document describes the Qualcomm DIAG log code 0xB193 (LTE ML1 Serving Cell Measurement Response), which provides detailed LTE signal strength measurements including RSRP, RSRQ, and RSSI.

## Overview

Log code 0xB193 (`LOG_LTE_ML1_SERVING_CELL_MEAS_RESPONSE`) is emitted by the Qualcomm modem's Layer 1 (ML1) component and contains periodic measurements of the serving cell's signal characteristics. Rayhunter captures these measurements and includes the RSRP value in GSMTAP headers for PCAP output.

## Packet Structure

The 0xB193 log uses a subpacket architecture common to many Qualcomm DIAG logs:

```
+------------------+
| Main Header      |  4 bytes
+------------------+
| Subpacket Header |  4 bytes
+------------------+
| Subpacket Data   |  Variable (version-dependent)
+------------------+
```

### Main Header (4 bytes)

| Offset | Size | Field           | Description                           |
|--------|------|-----------------|---------------------------------------|
| 0      | 1    | main_version    | Main packet version (observed: 1)     |
| 1      | 1    | num_subpackets  | Number of subpackets (typically 1)    |
| 2      | 2    | reserved        | Reserved/padding                      |

### Subpacket Header (4 bytes)

| Offset | Size | Field             | Description                         |
|--------|------|-------------------|-------------------------------------|
| 0      | 1    | subpacket_id      | Subpacket identifier                |
| 1      | 1    | subpacket_version | Subpacket version (see below)       |
| 2      | 2    | subpacket_size    | Size of subpacket including header  |

### Known Subpacket Versions

Different modem firmware versions emit different subpacket versions. The field offsets within the subpacket data vary by version:

| Version | PCI Offset | EARFCN Offset | RSRP Offset | Notes                    |
|---------|------------|---------------|-------------|--------------------------|
| 4       | 0          | 2             | 12          | Early format (SCAT)      |
| 7       | 0          | 4             | 14          | Intermediate format      |
| 18-24   | 0          | 4             | 24          | Common on Orbic RC400L   |
| 35-40   | 0          | 4             | 28          | Newer modems             |

The Orbic RC400L device used for development emits **subpacket version 18**.

## Signal Measurement Fields

### RSRP (Reference Signal Received Power)

RSRP is the primary signal strength indicator for LTE. The raw 12-bit value is converted to dBm:

```
RSRP (dBm) = -180.0 + (raw_value & 0xFFF) * 0.0625
```

Typical range: -140 dBm (very weak) to -44 dBm (very strong)

### PCI (Physical Cell ID)

The Physical Cell ID identifies the serving cell. Stored as a 16-bit little-endian value at the PCI offset.

Range: 0-503

### EARFCN (E-UTRA Absolute Radio Frequency Channel Number)

The EARFCN identifies the carrier frequency. Stored as a 32-bit little-endian value at the EARFCN offset.

## Implementation Notes

1. **Caching Strategy**: Since 0xB193 messages arrive independently from RRC OTA messages, rayhunter caches the most recent RSRP value and applies it to subsequent GSMTAP headers.

2. **Signal Conversion**: The `signal_dbm` field in GSMTAP headers is an `i8`, so the RSRP value is clamped to the range -128 to 0 dBm.

3. **Version Detection**: The subpacket version determines field offsets. Unknown versions fall back to the v7 layout.

## References

### SCAT (Signaling Collection and Analysis Tool)

The [SCAT project](https://github.com/fgsect/scat) by the Firmware Security (fgsect) research group at TU Berlin provides Qualcomm DIAG log parsers.

Relevant file: `parsers/qualcomm/diagltelogparser.py`

```python
# SCAT v4/v5 parser structure (simplified)
# pci = struct.unpack('<H', payload[0:2])
# earfcn = struct.unpack('<H', payload[2:4])  # or <L for 32-bit
# rsrp_raw = struct.unpack('<L', payload[offset:offset+4])
```

### Mobile Insight

The [Mobile Insight project](https://github.com/mobile-insight/mobileinsight-core) from UCLA WiNG Lab provides comprehensive Qualcomm DIAG parsing with extensive version support.

Relevant file: `mobile_insight/analyzer/msg_logger.py` and related LTE analyzers

Mobile Insight documents subpacket versions 4, 7, 18, 19, 22, 24, 35, 36, and 40, with version-specific field layouts.

### QCSuper

The [QCSuper project](https://github.com/P1sec/QCSuper) by P1 Security provides another implementation of Qualcomm DIAG protocol handling.

### 3GPP Specifications

- **3GPP TS 36.214**: Physical layer measurements (defines RSRP, RSRQ, RSSI)
- **3GPP TS 36.133**: Requirements for support of radio resource management

## Example Output

When rayhunter captures a 0xB193 log, debug output shows:

```
ML1 0xB193 v18: RSRP=-94.8dBm, PCI=446, EARFCN=975
```

The corresponding GSMTAP packets in Wireshark will display `Signal dBm: -95` (rounded to i8).
