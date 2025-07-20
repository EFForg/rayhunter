# Offline OpenCellID Analysis Guide

## Overview

Due to memory constraints on the Orbic device (159MB total RAM), OpenCellID analysis is performed offline in post-processing. The enhanced rayhunter firmware collects all necessary cellular parameters for comprehensive offline analysis.

## Required Data Collection

The enhanced rayhunter captures the following parameters essential for OpenCellID lookups:

### Primary Cell Identification
- **MCC** (Mobile Country Code) - 3 digits
- **MNC** (Mobile Network Code) - 2-3 digits  
- **LAC** (Location Area Code) - 2G/3G networks
- **TAC** (Tracking Area Code) - LTE/4G networks
- **Cell ID** (Cell Identity/Global Cell ID)
- **PCI** (Physical Cell ID) - LTE specific

### Network Parameters
- **eNodeB ID** - Base station identifier
- **Sector ID** - Cell sector within base station
- **PLMN ID** - Combined network identifier

### Signal Quality Metrics
- **RSRP** - Reference Signal Received Power (dBm)
- **RSRQ** - Reference Signal Received Quality (dB)
- **RSSI** - Received Signal Strength Indicator (dBm)
- **SINR** - Signal to Interference plus Noise Ratio (dB)

### Frequency Information
- **EARFCN** - LTE frequency channel
- **UARFCN** - UMTS frequency channel
- **ARFCN** - GSM frequency channel
- **Band** - Frequency band identifier

## Data Export Format

The enhanced firmware exports cellular data in CSV format to `/data/rayhunter/captures/` with the following structure:

```csv
timestamp,rat,mcc,mnc,lac,tac,cell_id,pci,enodeb_id,sector_id,rsrp,rsrq,rssi,sinr,earfcn,uarfcn,arfcn,band
2025-07-01T12:00:00Z,LTE,310,260,1234,5678,987654321,123,456,1,-85,-10,-80,15,1900,,,20
2025-07-01T12:00:05Z,GSM,310,260,1234,,987654321,,,,,-90,,,,,1800,850,1800
```

## Offline Analysis Process

### Step 1: Download OpenCellID Database
```bash
# Download latest OpenCellID database
wget https://download.opencellid.org/ocid/downloads/cell_towers.csv.gz
gunzip cell_towers.csv.gz
```

### Step 2: Extract Data from Device
```bash
# Copy captured data from device
adb pull /data/rayhunter/captures/ ./cellular_data/
```

### Step 3: Post-Processing Analysis
```python
import pandas as pd
import requests

# Load OpenCellID database
opencellid_db = pd.read_csv('cell_towers.csv')

# Load rayhunter captures
rayhunter_data = pd.read_csv('cellular_data/capture_YYYYMMDD.csv')

# Perform offline lookups
def lookup_cell_info(mcc, mnc, lac_tac, cell_id):
    """Lookup cell information in OpenCellID database"""
    matches = opencellid_db[
        (opencellid_db['mcc'] == mcc) &
        (opencellid_db['mnc'] == mnc) &
        (opencellid_db['area'] == lac_tac) &
        (opencellid_db['cell'] == cell_id)
    ]
    
    if not matches.empty:
        cell = matches.iloc[0]
        return {
            'latitude': cell['lat'],
            'longitude': cell['lon'],
            'range': cell['range'],
            'samples': cell['samples'],
            'changeable': cell['changeable'],
            'created': cell['created'],
            'updated': cell['updated']
        }
    return None

# Enrich rayhunter data with OpenCellID lookups
enriched_data = []
for _, row in rayhunter_data.iterrows():
    cell_info = lookup_cell_info(
        row['mcc'], 
        row['mnc'], 
        row['tac'] if row['rat'] == 'LTE' else row['lac'],
        row['cell_id']
    )
    
    enriched_row = row.to_dict()
    if cell_info:
        enriched_row.update(cell_info)
    
    enriched_data.append(enriched_row)

# Save enriched data
enriched_df = pd.DataFrame(enriched_data)
enriched_df.to_csv('enriched_cellular_analysis.csv', index=False)
```

## Analysis Capabilities

### IMSI Catcher Detection
With offline OpenCellID analysis, you can detect:

1. **Unknown Cell Towers**
   - Cells not in OpenCellID database
   - Potential rogue base stations

2. **Location Anomalies**
   - Cells appearing in wrong geographical locations
   - Impossible cell movements

3. **Signal Anomalies**
   - Abnormal signal patterns
   - Suspicious neighbor cell configurations

4. **Temporal Analysis**
   - Cell appearance/disappearance patterns
   - Tracking area update anomalies

### Example Analysis Queries

```python
# Find unknown cells (potential IMSI catchers)
unknown_cells = enriched_df[enriched_df['latitude'].isna()]

# Find cells with abnormal signal patterns
signal_anomalies = enriched_df[
    (enriched_df['rsrp'] > -50) |  # Too strong signal
    (enriched_df['rsrp'] < -120)   # Too weak signal
]

# Find rapid location changes (impossible movements)
location_changes = enriched_df.groupby('cell_id').apply(
    lambda x: x.sort_values('timestamp').diff()
)
```

## Memory Usage Optimization

### On-Device (Orbic)
- **No OpenCellID database loading** - saves ~100MB+ RAM
- **Efficient data structures** for real-time capture
- **CSV streaming** to storage
- **Minimal in-memory buffering**

### Offline Analysis
- **Full OpenCellID database** available
- **Advanced algorithms** for pattern detection
- **Geographic analysis** with mapping
- **Historical trend analysis**

## File Locations

### On Device
- **Captures**: `/data/rayhunter/captures/`
- **Logs**: `/var/log/rayhunter.log`
- **Config**: `/etc/rayhunter/config.toml`

### Post-Processing
- **OpenCellID DB**: `./opencellid/cell_towers.csv`
- **Raw Data**: `./cellular_data/`
- **Analysis Results**: `./analysis_results/`

## Benefits of Offline Analysis

1. **Resource Efficiency**: Device focuses on data collection
2. **Comprehensive Analysis**: Full OpenCellID database available
3. **Advanced Algorithms**: Complex analysis without memory constraints
4. **Historical Analysis**: Process multiple capture sessions
5. **Detailed Reporting**: Generate comprehensive reports with maps

This approach maximizes both data collection efficiency on the resource-constrained device and analysis depth in post-processing.
