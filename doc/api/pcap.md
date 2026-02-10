# Get PCAP

Stream the PCAP file `{name}` chunk-by-chunk to the client by reading the QMDL data written so far.

**URL**: `/api/pcap/{name}`

**Method**: `GET`

**Data Constraints**: none

## Success Response

**Code**: 200

**Content**:

The client would receive a streamed PCAP file in the body of the response, and the header would indicate the Content-Type to be `application/vnd.tcpdump.pcap`.

## Error Response

### Code 404

**Condition**: file not found

**Content Example**:

```
couldn't find manifest entry with name 1770697005
```

**Solution**: Check the [QMDL manifest](qmdl-manifest.md) for the available file names.

### Code 503

**Condition**: Source QMDL file is blank

**Content**:

```
QMDL file is empty, try again in a bit!
```

**Solution**: Try again in a bit.
