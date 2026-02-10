# Analysis Report

Display the processed analysis report of the QMDL file `{name}`, as well as which types (and versions) of analyzers were used. Response is chunked as application/x-ndjson.

**URL**: `/api/analysis-report/{name}`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content Example**:

```json
{
	"analyzers": [
		{
			"name": "Identity (IMSI or IMEI) requested in suspicious manner",
			"description": "Tests whether the ME sends an Identity Request NAS message without either an associated attach request or auth accept message",
			"version": 3
		},
		{
			"name": "Connection Release/Redirected Carrier 2G Downgrade",
			"description": "Tests if a cell releases our connection and redirects us to a 2G cell.",
			"version": 1
		},
		{
			"name": "LTE SIB 6/7 Downgrade",
			"description": "Tests for LTE cells broadcasting a SIB type 6 and 7 which include 2G/3G frequencies with higher priorities.",
			"version": 2
		},
		{
			"name": "Null Cipher",
			"description": "Tests whether the cell suggests using a null cipher (EEA0)",
			"version": 1
		},
		{
			"name": "NAS Null Cipher Requested",
			"description": "Tests whether the MME requests to use a null cipher in the NAS security mode command",
			"version": 1
		},
		{
			"name": "Incomplete SIB",
			"description": "Tests whether a SIB1 message contains a full chain of followup sibs",
			"version": 2
		}
	],
	"rayhunter": {
		"rayhunter_version": "0.10.1",
		"system_os": "Linux 3.18.48",
		"arch": "armv7l"
	},
	"report_version": 2
}
```

## Error Response

### Code 404

**Condition**: File not found.

**Content Example**:

```
Couldn't find QMDL entry with name 1770697005
```

**Solution**: Check the [QMDL manifest](qmdl-manifest.md) for the available file names.

### Code 503

**Condition**: No QMDL data on the device.

**Content**:

```
No QMDL data's being recorded to analyze, try starting a new recording!
```

**Solution**: [Start a recording](start-recording.md).
