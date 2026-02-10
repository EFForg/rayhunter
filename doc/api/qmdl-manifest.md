# QMDL Manifest

List QMDL files available on the device and some of their statistics.

**URL**: `/api/qmdl-manifest`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content Example**:

```json
{
	"entries": [
		{
			"name": "1770697004",
			"start_time": "2026-02-09T23:16:44.718417288-05:00",
			"last_message_time": "2026-02-10T10:57:45.356811119-05:00",
			"qmdl_size_bytes": 88451,
			"rayhunter_version": "0.10.1",
			"system_os": "Linux 3.18.48",
			"arch": "armv7l"
		},
		...
	],
	"current_entry": {
		"name": "1770744506",
		"start_time": "2026-02-10T12:28:26.139895763-05:00",
		"last_message_time": "2026-02-10T12:48:46.196211652-05:00",
		"qmdl_size_bytes": 4226,
		"rayhunter_version": "0.10.1",
		"system_os": "Linux 3.18.48",
		"arch": "armv7l"
	}
}
```
