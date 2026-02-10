# Log

Download the current device log in plain text (UTF-8).

**URL**: `/api/log`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content Example**: 

```
R A Y H U N T E R 🐳
[2026-02-10T03:22:15Z INFO  rayhunter_daemon] Using configuration for device: Orbic
[2026-02-10T03:22:15Z INFO  rayhunter::diag_device] Diag device initialization succeeded after 0 retries
...
```

## Error Response

### Code 500

**Condition**: Could not read /data/rayhunter/rayhunter.log file
