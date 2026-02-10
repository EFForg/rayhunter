# Config

Show the saved configuration for Rayhunter

**URL**: `/api/config`

**Method**: `GET`

**Data Constraints**: none

### Success Response

**Code**: 200

**Content Example**:

```json
{
	"qmdl_store_path": "/data/rayhunter/qmdl",
	"port": 8080,
	"debug_mode": false,
	"device": "orbic",
	"ui_level": 1,
	"colorblind_mode": false,
	"key_input_mode": 0,
	"ntfy_url": "https://ntfy.sh/my_notification_endpoint_a1b2c3d4e5",
	"enabled_notifications": [
		"Warning",
		"LowBattery"
	],
	"analyzers": {
		"imsi_requested": true,
		"connection_redirect_2g_downgrade": true,
		"lte_sib6_and_7_downgrade": true,
		"null_cipher": true,
		"nas_null_cipher": true,
		"incomplete_sib": true,
		"test_analyzer": false
	}
}
```
