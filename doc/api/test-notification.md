# Test Notification

Send a test notification to the ntfy URL in the running configuration.

**URL**: `/api/test-notification`

**Method**: `POST`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content**:

```
Test notification sent successfully
```

## Error Response

### Code 400

**Condition**: No notification URL is stored in the running configuration.

**Content Example**:

```
Notification URL is empty
```
```
No notification URL configured
```

**Solution**: Set and save an ntfy_url to your device config.

### Code 500

**Condition**: Internal server error on notification request.

**Content Example**:

```
Failed to send test notification: HTTP request failed: error sending request
```

**Solution**: Ensure your device can reach the internet.
