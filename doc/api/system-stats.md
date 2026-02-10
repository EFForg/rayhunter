# System Stats

Display the system/device statistics.

**URL**: `/api/system-stats`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content Example**:

```json
{
	"disk_stats": {
		"partition": "ubi0:usrfs",
		"total_size": "214.7M",
		"used_size": "6.4M",
		"available_size": "208.3M",
		"used_percent": "3%",
		"mounted_on": "/data"
	},
	"memory_stats": {
		"total": "159.9M",
		"used": "151.4M",
		"free": "8.5M"
	},
	"runtime_metadata": {
		"rayhunter_version": "0.10.1",
		"system_os": "Linux 3.18.48",
		"arch": "armv7l"
	},
	"battery_status": {
		"level": 100,
		"is_plugged_in": false
	}
}
```

## Error Response

### Code 500

**Condition**: System stats unavailable

**Content Example**:

```
error getting system stats
```
