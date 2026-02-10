# Delete All Recordings

Remove all stored data capture files from the device

**URL**: `/api/delete-all-recordings`

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

**Condition**: System is in debug mode

**Content**:

```
server is in debug mode
```

**Solution**: Restart the device in normal mode.

### Code 500

**Condition**: Failed to delete recordings

**Content Example**:

```
couldn't send delete all entries message
```
```
failed to receive delete all response
```
```
couldn't delete recordings
```