# Stop Recording

Stop a running data capture.

**URL**: `/api/stop-recording`

**Method**: `POST`

**Data Constraints**: none

## Success Response

**Code**: 202

**Content**:

```
ok
```

## Error Response

### Code 403

**Condition**: System is in debug mode.

**Content**:

```
server is in debug mode
```

**Solution**: Restart the device in normal mode.

### Code 500

**Condition**: Recording action unsuccessful.

**Content Example**:

```
couldn't send stop recording message
```