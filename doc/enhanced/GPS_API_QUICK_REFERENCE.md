# GPS API Quick Reference Card

## 🚀 Quick Start
```bash
# Submit GPS coordinates (POST)
curl -X POST "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"

# Submit GPS coordinates (GET - GPS2REST-Android compatible)
curl -X GET "http://192.168.1.1:8080/api/v1/gps/37.7749,-122.4194"
```

## 📍 Endpoint
- **URL**: `GET|POST /api/v1/gps/{lat},{lon}`
- **Methods**: GET or POST (both supported)
- **Format**: `latitude,longitude` (comma-separated)
- **Range**: lat: -90 to 90, lon: -180 to 180
- **GPS2REST-Android**: Compatible with GET requests

## 📄 Responses
### ✅ Success (200)
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

### ❌ Error (400/500)
```json
{
    "status": "error",
    "error": "Latitude must be between -90 and 90 degrees"
}
```

## 💾 Data Storage
- **Per-Scan GPS**: `/data/rayhunter/captures/{scan_id}.gps`
- **Legacy CSV**: `/data/rayhunter/gps-data/gps_coordinates.csv` (deprecated)
- **Legacy JSON**: `/data/rayhunter/gps-data/gps_coordinates.json` (deprecated)

## 🔍 Check Data
```bash
# View stored coordinates
adb shell 'rootshell -c "ls -la /data/rayhunter/captures/*.gps"'

# Check latest entries
adb shell 'rootshell -c "tail -5 /data/rayhunter/captures/$(ls -t /data/rayhunter/captures/*.gps | head -1)"'
```

## 🐍 Python Example
```python
import requests

# POST method (original)
def submit_gps_post(lat, lon):
    url = f"http://192.168.1.1:8080/api/v1/gps/{lat},{lon}"
    return requests.post(url).json()

# GET method (GPS2REST-Android compatible)
def submit_gps_get(lat, lon):
    url = f"http://192.168.1.1:8080/api/v1/gps/{lat},{lon}"
    return requests.get(url).json()

result_post = submit_gps_post(37.7749, -122.4194)
result_get = submit_gps_get(37.7749, -122.4194)
print(result_post)
print(result_get)
```

## 🌐 JavaScript Example
```javascript
// POST method (original)
async function submitGPS_POST(lat, lon) {
    const response = await fetch(
        `http://192.168.1.1:8080/api/v1/gps/${lat},${lon}`,
        { method: 'POST' }
    );
    return response.json();
}

// GET method (GPS2REST-Android compatible)
async function submitGPS_GET(lat, lon) {
    const response = await fetch(
        `http://192.168.1.1:8080/api/v1/gps/${lat},${lon}`,
        { method: 'GET' }
    );
    return response.json();
}

submitGPS_POST(37.7749, -122.4194).then(console.log);
submitGPS_GET(37.7749, -122.4194).then(console.log);
```

## 🛠️ Common Use Cases
- **Mobile App**: Real-time location tracking
- **Vehicle/Drone**: External GPS logger integration  
- **Post-Processing**: Correlate location with cellular data
- **Analysis**: Cell tower coverage mapping

---
📖 **[Full Documentation →](GPS_API_DOCUMENTATION.md)**
