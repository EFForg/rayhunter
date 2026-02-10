# API Documentation

As with the rest of Rayhunter, endpoints require no authentication.

## Configuration

* [Show config](config-get.md) : `GET /api/config`
* [Write config](config-post.md) : `POST /api/config`
* [Show system time](time.md) : `GET /api/time`
* [Set time offset](time-offset.md) : `POST /api/time-offset`
* [Send test notification](test-notification.md) : `POST /api/test-notification`
* [Set display state](debug-display-state.md) : `POST /api/debug/display-state`

## Statistics

* [System stats](system-stats.md) : `GET /api/system-stats`
* [System logs](log.md) : `GET /api/log`
* [List QMDL files](qmdl-manifest.md) : `GET /api/qmdl-manifest`

## Recordings

* [Start recording](start-recording.md) : `POST /api/start-recording`
* [Stop recording](stop-recording.md) : `POST /api/stop-recording`
* [Delete recording](delete-recording.md) : `POST /api/delete-recording/{name}`
* [Delete all recordings](delete-all-recordings.md) : `POST /api/delete-all-recordings`
* [Download PCAP file](pcap.md) : `GET /api/pcap/{name}`
* [Download QMDL file](qmdl.md) : `GET /api/qmdl/{name}`
* [Download ZIP file](zip.md) : `GET /api/zip/{name}`

## Analysis

* [Show analysis report](analysis-report.md) : `GET /api/analysis-report/{name}`
* [Show analysis status](analysis-get.md) : `GET /api/analysis`
* [Start analysis](analysis-post.md) : `POST /api/analysis/{name}`

## Common Error Responses

These error responses are common to all endpoints.

### Code 400

**Condition**: Body is empty, bad request, or unparsable JSON

**Content Example**:

```
Failed to parse the request body as JSON: EOF while parsing a value at line 1 column 0
```

**Solution**: Validate your json content or request body.

### Code 404

**Condition**: Endpoint not found

**Content**: none

**Solution**: Confirm the URL of your request

### Code 405

**Condition**: Invalid method

**Content**: none

**Solution**: Header 'allow' on the response will indicate the allowed methods.

### Code 415

**Condition**: Invalid headers

**Content Example**:
```
Expected request with `Content-Type: application/json`
```

**Solution**: Add the indicated header to your request.

### Code 422

**Condition**: Invalid JSON content type

**Content Example**:

```
Failed to deserialize the JSON body into the target type: analyzers.incomplete_sib: invalid type: string "", expected a boolean at line 3 column 28
```

**Solution**: Confirm the input data constraints of your request method.
