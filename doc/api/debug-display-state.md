# Display State

Change the display state (color bar or otherwise) of the device for debugging purposes.

**URL**: `/api/debug/display-state`

**Method**: `POST`

**Data Constraints**:

```json
{
	"display_state": [one of "Recording", "Paused", or "WarningDetected"]
}
```

## Success Response

**Code**: 200

**Content**: 

```
display state updated successfully
```

## Error Response

### Code 500

**Condition**: Error sending update to the display

**Content Example**:
```
failed to send display state update
```

### Code 503

**Condition**: Display service unavailable

**Content Example**:
```
display system not available
```
