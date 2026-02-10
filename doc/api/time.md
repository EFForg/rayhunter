# Time

Get the current time and offset (in seconds) of the device

**URL**: `/api/time`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content Example**:

```json
{
	"system_time": "2026-02-10T12:00:07.988526519-05:00",
	"adjusted_time": "2026-02-10T11:59:54.988591728-05:00",
	"offset_seconds": -12
}
```
