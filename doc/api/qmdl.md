# Get QMDL

Stream the QMDL file `{name}` to the client.

**URL**: `/api/qmdl/{name}`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content**:

The client would receive a streamed QMDL file in the body of the response, and the header would indicate the Content-Type to be `application/octet-stream`.

## Error Response

### Code 404

**Condition**: file not found

**Content Example**:

```
couldn't find qmdl file with name 1770697005
```

**Solution**: Check the [QMDL manifest](qmdl-manifest.md) for the available file names.

### Code 500

**Condition**: Error opening file

**Content Example**:

```
error opening QMDL file
```
