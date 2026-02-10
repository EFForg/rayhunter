# Delete Recording

Remove a data capture file with name `{name}` from the device

**URL**: `/api/delete-recording/{name}`

**Method**: `POST`

**Data Constraints**: none

## Success Response

**Code**: 202

**Content**:

```
ok
```

## Error Response

### Code 400

**Condition**: Bad recording name or no such recording

**Content Example**:

```
no recording with name 1770697005
```

**Solution**: Check the [QMDL manifest](qmdl-manifest.md) for the available file names.

### Code 403

**Condition**: System is in debug mode

**Content**:

```
server is in debug mode
```

**Solution**: Restart the device in normal mode.

### Code 500

**Condition**: Failed to delete recording

**Content Example**:

```
couldn't send delete entry message
```
```
failed to receive delete response
```
```
couldn't delete recording
```