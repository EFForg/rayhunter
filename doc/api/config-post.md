# Config

Write a new configuration element to Rayhunter and trigger a restart

**URL**: `/api/config`

**Method**: `POST`

**Headers**: `Content-Type: application/json`

**Data Constraints**:

Any or all configuration elements from the valid config schema to be altered may be passed. Invalid keys will be discarded. Invalid values or value types will return an error. See the success response from the [GET method of /api/config/](config-get.md) for most configuration points.

**Data Example**:

```json
{
	"ui_level": 2,
	"enabled_notifications": [
		"Warning"
	],
	"invalid_key": "ignored_value"
}
```

### Success Response

**Code**: 202

**Content**:

```
wrote config and triggered restart
```
