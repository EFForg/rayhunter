# Analysis

Show analysis status for all QMDL files. Arrays of queued and finished files are returned along with the name of the file currently being analyzed, if any.

**URL**: `/api/analysis`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content Example**:

```json
{
	"queued": [],
	"running": null,
	"finished": [
		"1770697004",
		"1770739068",
		"1770739380",
		"1770740467",
		"1770740523",
		"1770740852"
	]
}
```
