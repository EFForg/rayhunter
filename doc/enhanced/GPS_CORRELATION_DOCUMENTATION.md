# GPS Correlation System Documentation

## Overview

The GPS Correlation System enables users to download GPS coordinate data that corresponds to specific Rayhunter recording sessions. This feature matches GPS timestamps with recording timeframes to provide location context for cellular data analysis.

## Features

### 🔗 Automatic Correlation
- Matches GPS data to recording sessions based on timestamps
- Includes 5-minute buffer before/after recording for timing tolerance
- Handles missing or incomplete GPS data gracefully

### 📱 Multiple Download Formats
- **CSV**: Simple comma-separated format for spreadsheet analysis
- **JSON**: Structured data with metadata for programmatic use  
- **GPX**: Standard GPS exchange format for mapping software

### 🖥️ Web Interface Integration
- GPS download buttons appear alongside existing PCAP/QMDL/ZIP downloads
- Dropdown menu for format selection
- Works on both desktop table view and mobile card view

## API Endpoints

### Download GPS Data
```http
GET /api/gps/{recording_id}
GET /api/gps/{recording_id}/{format}
```

**Parameters:**
- `recording_id`: The ID of the recording session
- `format`: Optional format (`csv`, `json`, `gpx`). Defaults to `csv`

**Example URLs:**
```
GET /api/gps/1720080123/csv
GET /api/gps/1720080123/json  
GET /api/gps/1720080123/gpx
```

### Upload GPS Coordinates
```http
GET|POST /api/v1/gps/{lat,lon}
```

**Parameters:**
- `lat,lon`: Latitude and longitude coordinates (e.g., `37.7749,-122.4194`)

**Methods:**
- `GET`: Compatible with GPS2REST-Android app
- `POST`: Original method for API calls

**Note:** Server generates timestamps automatically (ignores any client timestamps)

## File Formats

### CSV Format
```csv
timestamp,latitude,longitude
2025-07-04T14:30:15.123Z,37.7749,-122.4194
2025-07-04T14:32:20.456Z,37.7849,-122.4094
```

### JSON Format
```json
{
  "recording_id": "1720080123",
  "start_time": "2025-07-04T14:30:00Z",
  "end_time": "2025-07-04T14:45:00Z",
  "total_entries": 2,
  "gps_entries": [
    {
      "timestamp": "2025-07-04T14:30:15.123Z",
      "latitude": 37.7749,
      "longitude": -122.4194
    }
  ]
}
```

### GPX Format
```xml
<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="Rayhunter">
  <trk>
    <name>Recording 1720080123</name>
    <trkseg>
      <trkpt lat="37.7749" lon="-122.4194">
        <time>2025-07-04T14:30:15.123Z</time>
      </trkpt>
    </trkseg>
  </trk>
</gpx>
```

## Correlation Logic

### Time Matching
1. **Recording Timeframe**: Uses `start_time` and `last_message_time` from recording manifest
2. **Buffer Zone**: Adds ±5 minutes to account for timing differences between GPS and cellular logs
3. **Filtering**: Includes only GPS points within the buffered timeframe
4. **Sorting**: GPS entries are sorted chronologically and deduplicated

### Data Sources
- **Primary**: `/data/rayhunter/captures/{scan_id}.gps`
- **Secondary**: `/data/rayhunter/gps-data/gps_coordinates.csv` (legacy)
- **Tertiary**: `/data/rayhunter/gps-data/gps_coordinates.json` (legacy)
- **Fallback**: Empty result if no GPS data exists (expected behavior)

## Usage Examples

### Web Interface
1. Navigate to the Rayhunter web interface
2. Find a recording session in the History table
3. Click the "GPS" dropdown button
4. Select desired format (CSV/JSON/GPX)
5. File downloads automatically

### Command Line Testing
```bash
# Add GPS coordinates (POST)
curl -X POST "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"

# Add GPS coordinates (GET - GPS2REST-Android compatible)
curl -X GET "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"

# Download GPS data for recording
curl "http://192.168.1.1:8080/api/gps/1720080123/csv" -o gps_data.csv
```

### Programmatic Access
```javascript
// Fetch GPS data for analysis
const response = await fetch('/api/gps/1720080123/json');
const gpsData = await response.json();

console.log(`Found ${gpsData.total_entries} GPS points for recording ${gpsData.recording_id}`);
gpsData.gps_entries.forEach(entry => {
  console.log(`${entry.timestamp}: ${entry.latitude}, ${entry.longitude}`);
});
```

## Integration with Analysis

### Workflow
1. **Start Recording**: Begin cellular data capture
2. **Log GPS**: Use external GPS device/app to submit coordinates via API
   - GPS2REST-Android app: Sends GET requests automatically
   - Custom apps: Can use GET or POST requests
   - Manual testing: Use curl with GET or POST
3. **Stop Recording**: End cellular data capture
4. **Download Data**: Get correlated PCAP + GPS files for analysis
5. **Analyze**: Use GPS context to understand location-based cellular behavior

### Use Cases
- **Signal Mapping**: Correlate signal strength with geographic location
- **Cell Tower Analysis**: Identify which towers serve specific areas
- **Movement Tracking**: Analyze handoff behavior during travel
- **IMSI Catcher Detection**: Detect anomalous behavior in specific locations

## Error Handling

### Common Scenarios
- **No GPS Data**: Returns empty result set (not an error)
- **Invalid Recording ID**: Returns 404 with error message
- **Invalid Format**: Returns 400 with supported format list
- **Missing Files**: Gracefully handles missing GPS data files

### Response Codes
- `200 OK`: Successful download with GPS data
- `200 OK`: Successful download with empty data (no GPS points found)
- `400 Bad Request`: Invalid format parameter
- `404 Not Found`: Recording ID not found
- `500 Internal Server Error`: System error processing request

## File Storage

### Directory Structure
```
/data/rayhunter/
├── captures/
│   ├── gps_coordinates.csv
│   ├── gps_coordinates.json
│   └── gps_logs.txt
├── captures/
└── qmdl/
```

### Automatic Creation
- GPS data directory is created during installation
- Missing directories are created automatically when needed
- CSV headers are written on first GPS coordinate upload

## Security Considerations

### Access Control
- GPS endpoints inherit same access controls as other API endpoints
- No authentication required (matches existing Rayhunter behavior)
- Consider network isolation for sensitive deployments

### Data Privacy
- GPS coordinates are stored locally on device
- No external transmission of location data
- Consider data retention policies for GPS logs

## Troubleshooting

### No GPS Downloads Available
- Verify GPS data directory exists: `/data/rayhunter/captures/`
- Check if GPS coordinates have been uploaded via API
- Ensure recording times overlap with GPS timestamps

### Empty GPS Files
- Check time synchronization between GPS source and device
- Verify GPS timestamps fall within recording timeframe ±5 minutes
- Review GPS data format in source files

### Download Errors
- Confirm recording ID exists in manifest
- Check browser console for network errors
- Verify device connectivity and web server status

## Version Information

- **Added in**: Rayhunter v0.4.1 - Enhanced
- **Dependencies**: None (uses existing Rayhunter infrastructure)
- **Compatibility**: Works with all existing Rayhunter features
