# Get ZIP

Stream a zip file to the client which contains QMDL file `{name}` and a PCAP generated from the same file.

**URL**: `/api/zip/{name}`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content**:

The client would receive a streamed ZIP file in the body of the response, and the header would indicate the Content-Type to be `application/zip`. It is possible that if a PCAP fails to be generated, the ZIP will still be sent with a successful response containing only the QMDL file.

## Error Response

### Code 404

**Condition**: file not found

**Content Example**:

```
couldn't find entry with name 1770697005
```

**Solution**: Check the [QMDL manifest](qmdl-manifest.md) for the available file names.

### Code 503

**Condition**: Source QMDL file is blank

**Content**:

```
QMDL file is empty, try again in a bit!
```

**Solution**: Try again in a bit.
