# GPS API Documentation

This document describes the GPS location tracking functionality added to Rayhunter.

## Overview

Rayhunter now includes GPS location tracking capabilities that allow you to:
- Track device location via REST API
- Log GPS coordinates to a file with timestamps
- Start/stop GPS tracking on demand
- Retrieve current GPS status and location

## Configuration

Add the following GPS configuration options to your `config.toml`:

```toml
# GPS Configuration
enable_gps = true                           # Enable GPS functionality (default: false)
gps_log_path = "/data/rayhunter/gps.log"   # Path for GPS log file (default: "/data/rayhunter/gps.log")
gps_update_interval_ms = 1000              # GPS update interval in milliseconds (default: 1000)
```

## REST API Endpoints

### Get GPS Status
**GET** `/api/gps/status`

Returns the current GPS status including whether it's enabled and active.

**Response:**
```json
{
  "enabled": true,
  "active": false,
  "last_location": null
}
```

### Start GPS Tracking
**POST** `/api/gps/start`

Starts GPS location tracking. Requires GPS to be enabled in configuration.

**Response:** 
- `200 OK` - GPS tracking started
- `503 Service Unavailable` - GPS disabled in configuration

### Stop GPS Tracking  
**POST** `/api/gps/stop`

Stops GPS location tracking.

**Response:**
- `200 OK` - GPS tracking stopped

### Get Current Location
**GET** `/api/gps/location`

Retrieves the current GPS location. Returns `null` if GPS is not active.

**Response when active:**
```json
{
  "latitude": 37.7749,
  "longitude": -122.4194,
  "altitude": 100.0,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

**Response when inactive:**
```json
null
```

## GPS Log Format

When GPS tracking is active, location data is automatically logged to the specified log file in CSV format:

```csv
timestamp,latitude,longitude,altitude
2024-01-15T10:30:00Z,37.7749,-122.4194,100.0
2024-01-15T10:30:01Z,37.7750,-122.4195,101.0
```

### Log File Fields

- **timestamp**: ISO 8601 formatted timestamp in UTC
- **latitude**: Latitude in decimal degrees
- **longitude**: Longitude in decimal degrees  
- **altitude**: Altitude in meters (may be empty if unavailable)

## Usage Examples

### Using cURL

```bash
# Check GPS status
curl http://localhost:8080/api/gps/status

# Start GPS tracking
curl -X POST http://localhost:8080/api/gps/start

# Get current location
curl http://localhost:8080/api/gps/location

# Stop GPS tracking
curl -X POST http://localhost:8080/api/gps/stop
```

### Using JavaScript/Fetch

```javascript
// Start GPS tracking
const startGPS = async () => {
  const response = await fetch('/api/gps/start', { method: 'POST' });
  if (response.ok) {
    console.log('GPS started successfully');
  }
};

// Get current location
const getLocation = async () => {
  const response = await fetch('/api/gps/location');
  const location = await response.json();
  if (location) {
    console.log(`Location: ${location.latitude}, ${location.longitude}`);
  } else {
    console.log('GPS not active');
  }
};
```

## Implementation Notes

### Current GPS Source
The current implementation uses mock GPS coordinates (San Francisco) for demonstration purposes. In a production deployment, this would be replaced with actual GPS hardware integration.

### Logging Behavior
- GPS data is logged only when GPS tracking is active
- Log entries are appended to the file (existing data is preserved)
- Each location request triggers a log entry if GPS is active
- Log files are created automatically if they don't exist

### Error Handling
- GPS functionality gracefully handles file I/O errors
- Configuration errors are reported via HTTP status codes
- Invalid GPS states return appropriate error responses

## Integration with Enhanced Features

This GPS API provides the foundation for more advanced location-based features that can be incrementally added:

- Geofencing capabilities
- Location-based analysis correlation
- Device movement tracking
- Location history and analytics

## Security Considerations

- GPS data contains sensitive location information
- Consider implementing authentication for GPS endpoints in production
- Log files should have appropriate filesystem permissions
- GPS functionality can be completely disabled via configuration 