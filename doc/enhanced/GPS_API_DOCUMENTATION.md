# GPS REST API Documentation

## Overview

The Rayhunter GPS REST API allows external systems to submit GPS coordinates to the device. This is particularly useful for correlating cellular data captures with precise location information when the device itself doesn't have GPS capabilities.

## API Endpoint

### Submit GPS Coordinates

**Endpoint:** `GET|POST /api/v1/gps/{latitude},{longitude}`

**Description:** Submit GPS coordinates with automatic timestamp recording

**Methods:** GET or POST (both supported)

**URL Format:** 
```
http://{device_ip}:8080/api/v1/gps/{lat},{lon}
```

**Parameters:**
- `latitude` (float): Latitude coordinate (-90.0 to 90.0)
- `longitude` (float): Longitude coordinate (-180.0 to 180.0)

**Headers:**
- `Content-Type: application/json` (optional for POST requests)

**GPS2REST-Android Compatibility:**
- The endpoint accepts GET requests for compatibility with the GPS2REST-Android app
- GPS2REST-Android sends requests as: `GET /api/v1/gps/{lat},{lon}`
- Server ignores any timestamps from client requests and uses its own timestamps

## Request Examples

### Valid Requests

#### POST Requests (Original format)
```bash
# San Francisco coordinates
curl -X POST "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"

# New York coordinates  
curl -X POST "http://192.168.1.1:8080/api/v1/gps/40.7128,-74.0060"

# London coordinates
curl -X POST "http://192.168.1.1:8080/api/v1/gps/51.5074,-0.1278"

# Using curl with headers
curl -X POST "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194" \
     -H "Content-Type: application/json"
```

#### GET Requests (GPS2REST-Android compatible)
```bash
# San Francisco coordinates (GPS2REST-Android format)
curl -X GET "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"

# New York coordinates
curl -X GET "http://192.168.1.1:8080/api/v1/gps/40.7128,-74.0060"

# London coordinates
curl -X GET "http://192.168.1.1:8080/api/v1/gps/51.5074,-0.1278"

# Browser-friendly (can be opened directly in browser)
# http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194
```

### Programming Language Examples

#### Python
```python
import requests

def submit_gps_location(device_ip, latitude, longitude):
    url = f"http://{device_ip}:8080/api/v1/gps/{latitude},{longitude}"
    
    try:
        response = requests.post(url)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        print(f"Error submitting GPS data: {e}")
        return None

# Usage
result = submit_gps_location("192.168.1.1", 37.7749, -122.4194)
print(result)
```

#### JavaScript/Node.js
```javascript
async function submitGpsLocation(deviceIp, latitude, longitude) {
    const url = `http://${deviceIp}:8080/api/v1/gps/${latitude},${longitude}`;
    
    try {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        return await response.json();
    } catch (error) {
        console.error('Error submitting GPS data:', error);
        return null;
    }
}

// Usage
submitGpsLocation("192.168.1.1", 37.7749, -122.4194)
    .then(result => console.log(result));
```

#### Bash/Shell Script
```bash
#!/bin/bash

submit_gps_location() {
    local device_ip="$1"
    local latitude="$2"
    local longitude="$3"
    
    curl -X POST "http://${device_ip}:8080/api/v1/gps/${latitude},${longitude}" \
         -H "Content-Type: application/json" \
         -w "\nHTTP Status: %{http_code}\n"
}

# Usage
submit_gps_location "192.168.1.1" "37.7749" "-122.4194"
```

## Response Format

### Success Response (HTTP 200)
```json
{
    "status": "success",
    "message": "GPS coordinate saved successfully",
    "data": {
        "timestamp": "2025-07-02T10:30:45.123Z",
        "latitude": 37.7749,
        "longitude": -122.4194
    }
}
```

### Error Responses

#### Invalid Coordinate Format (HTTP 400)
```json
{
    "status": "error",
    "error": "Invalid format. Expected: lat,lon"
}
```

#### Invalid Latitude (HTTP 400)
```json
{
    "status": "error",
    "error": "Latitude must be between -90 and 90 degrees"
}
```

#### Invalid Longitude (HTTP 400)
```json
{
    "status": "error",
    "error": "Longitude must be between -180 and 180 degrees"
}
```

#### Server Error (HTTP 500)
```json
{
    "status": "error",
    "error": "Failed to save GPS data: [error details]"
}
```

## Data Storage

GPS coordinates are automatically saved to the device in two formats:

### File Locations
- **Base Directory:** `/data/rayhunter/captures/`
- **Per-Scan GPS Files:** `/data/rayhunter/captures/{scan_id}.gps`
- **Legacy CSV File:** `/data/rayhunter/gps-data/gps_coordinates.csv` (deprecated)
- **Legacy JSON File:** `/data/rayhunter/gps-data/gps_coordinates.json` (deprecated)

### CSV Format
```csv
timestamp,latitude,longitude
2025-07-02 10:30:45.123 UTC,37.7749,-122.4194
2025-07-02 10:31:15.456 UTC,40.7128,-74.0060
2025-07-02 10:32:00.789 UTC,51.5074,-0.1278
```

### JSON Format
```json
[
    {
        "timestamp": "2025-07-02T10:30:45.123Z",
        "latitude": 37.7749,
        "longitude": -122.4194
    },
    {
        "timestamp": "2025-07-02T10:31:15.456Z",
        "latitude": 40.7128,
        "longitude": -74.0060
    }
]
```

## Data Validation

The API performs the following validations:

1. **Format Check:** Ensures coordinates are provided as `latitude,longitude`
2. **Numeric Validation:** Verifies both values are valid floating-point numbers
3. **Range Validation:**
   - Latitude: -90.0 ≤ lat ≤ 90.0
   - Longitude: -180.0 ≤ lon ≤ 180.0

## Integration Examples

### Mobile App Integration
```javascript
// Example for mobile app with GPS
navigator.geolocation.getCurrentPosition(
    (position) => {
        const lat = position.coords.latitude;
        const lon = position.coords.longitude;
        
        submitGpsLocation("192.168.1.1", lat, lon);
    },
    (error) => console.error("GPS error:", error),
    { enableHighAccuracy: true }
);
```

### Continuous Location Tracking
```python
import time
import requests
from datetime import datetime

def continuous_gps_tracking(device_ip, gps_source, interval_seconds=30):
    """
    Continuously submit GPS coordinates at specified intervals
    """
    while True:
        try:
            # Get GPS coordinates from your source
            lat, lon = gps_source.get_current_position()
            
            # Submit to Rayhunter device
            result = submit_gps_location(device_ip, lat, lon)
            
            if result and result.get('status') == 'success':
                print(f"GPS data submitted: {lat}, {lon} at {datetime.now()}")
            else:
                print(f"Failed to submit GPS data: {result}")
                
        except Exception as e:
            print(f"Error in GPS tracking loop: {e}")
        
        time.sleep(interval_seconds)
```

### GPS Log File Processing
```bash
#!/bin/bash
# Process existing GPS log file and submit to Rayhunter

DEVICE_IP="192.168.1.1"
GPS_LOG_FILE="/path/to/gps_log.txt"

# Assuming GPS log format: timestamp,latitude,longitude
while IFS=',' read -r timestamp latitude longitude; do
    if [[ $latitude =~ ^-?[0-9]+\.?[0-9]*$ ]] && [[ $longitude =~ ^-?[0-9]+\.?[0-9]*$ ]]; then
        echo "Submitting: $latitude, $longitude"
        curl -X POST "http://${DEVICE_IP}:8080/api/v1/gps/${latitude},${longitude}" -s
        sleep 1  # Rate limiting
    fi
done < "$GPS_LOG_FILE"
```

## Accessing Stored Data

### Via ADB Shell
```bash
# List GPS data files
adb shell 'rootshell -c "ls -la /data/rayhunter/gps-data/"'

# View CSV data
adb shell 'rootshell -c "cat /data/rayhunter/gps-data/gps_coordinates.csv"'

# View JSON data
adb shell 'rootshell -c "cat /data/rayhunter/gps-data/gps_coordinates.json"'

# Check recent entries (last 10)
adb shell 'rootshell -c "tail -10 /data/rayhunter/gps-data/gps_coordinates.csv"'
```

### Data Analysis
The GPS data can be correlated with cellular capture data for enhanced analysis:

1. **Time Correlation:** Match GPS timestamps with QMDL capture times
2. **Location Analysis:** Determine cell tower coverage areas
3. **Movement Tracking:** Analyze handoff patterns during movement
4. **IMSI Catcher Detection:** Correlate suspicious cellular behavior with location

## Rate Limiting and Performance

- **No Built-in Rate Limiting:** The API accepts requests as fast as they arrive
- **File I/O:** Each request performs disk I/O operations (CSV append + JSON rewrite)
- **Recommended Rate:** For continuous tracking, limit to 1 request per second or less
- **Concurrent Requests:** The API is thread-safe and can handle concurrent requests

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Verify the device IP address
   - Ensure the Rayhunter daemon is running
   - Check that port 8080 is accessible

2. **Permission Denied**
   - The `/data/rayhunter/captures/` directory will be created automatically
   - Ensure sufficient disk space is available

3. **Invalid Coordinates**
   - Check coordinate format: `latitude,longitude`
   - Verify latitude is between -90 and 90
   - Verify longitude is between -180 and 180

### Debug Commands
```bash
# Check if Rayhunter daemon is running
adb shell 'rootshell -c "ps aux | grep rayhunter"'

# Check network connectivity
adb shell 'rootshell -c "netstat -tln | grep 8080"'

# Check disk space
adb shell 'rootshell -c "df -h /data"'

# Check GPS data directory permissions
adb shell 'rootshell -c "ls -la /data/rayhunter/"'
```

## Security Considerations

- **No Authentication:** The API currently has no authentication mechanism
- **Network Access:** Ensure the device network is properly secured
- **Data Privacy:** GPS coordinates are stored in plaintext
- **Access Control:** Only devices with network access to the Rayhunter device can submit coordinates

## Future Enhancements

Potential improvements for future versions:

1. **Authentication:** API key or token-based authentication
2. **Batch Upload:** Submit multiple coordinates in a single request
3. **Data Retrieval:** GET endpoints to retrieve stored GPS data
4. **Real-time Streaming:** WebSocket support for real-time location updates
5. **Data Encryption:** Encrypt stored GPS data
6. **Metadata Support:** Additional fields like accuracy, altitude, speed
7. **Export Formats:** Support for GPX, KML export formats
