# rayhunter-check JSON Output Format

## Overview

The `rayhunter-check` tool now supports JSON formatted output in addition to the default text format. This enhancement enables programmatic processing of analysis results and integration with automated workflows.

## Usage

### Basic Usage

```bash
# Text format (default)
rayhunter-check -p /path/to/captures

# JSON format to stdout
rayhunter-check -p /path/to/captures --format json

# JSON format with individual .ndjson files per capture
rayhunter-check -p /path/to/captures --format json
```

### Output Formats

#### Text Format (Default)
Traditional human-readable output with logs and summary statistics:
```
**** Beginning analysis of capture.qmdl
capture.qmdl: INFO - 2024-01-01T12:00:00Z Some information
capture.qmdl: WARNING (Severity: High) - 2024-01-01T12:00:01Z Potential IMSI catcher detected
capture.qmdl: 100 messages analyzed, 1 warnings, 5 messages skipped
```

#### JSON Format
Newline-delimited JSON (NDJSON) format matching the daemon's analysis output:

**First line:** Metadata
```json
{
  "analyzers": [
    {
      "name": "IMSI Requested",
      "description": "Detects when the network requests IMSI...",
      "version": 1
    }
  ],
  "rayhunter": {
    "rayhunter_version": "0.9.0",
    "system_os": "Linux",
    "arch": "x86_64"
  },
  "report_version": 2
}
```

**Subsequent lines:** Analysis rows
```json
{
  "packet_timestamp": "2024-01-01T12:00:00Z",
  "skipped_message_reason": null,
  "events": [
    {
      "event_type": "High",
      "message": "Potential IMSI catcher detected (packet 42)"
    },
    null
  ]
}
```

**Skipped packets:**
```json
{
  "packet_timestamp": "2024-01-01T12:00:05Z",
  "skipped_message_reason": "Failed to parse GSMTAP header",
  "events": []
}
```

## File Output

When using `--format json`, the tool writes `.ndjson` files alongside the analyzed captures:

```
captures/
├── capture1.qmdl
├── capture1.ndjson    # JSON analysis output
├── capture2.pcapng
└── capture2.ndjson    # JSON analysis output
```

This matches the behavior of the on-device rayhunter daemon, making it easy to process captures in a consistent way.

## Use Cases

### Automated Analysis Pipeline
```bash
# Analyze all captures in a directory
rayhunter-check -p captures/ --format json

# Process results with jq
for file in captures/*.ndjson; do
  echo "Processing $file"
  # Extract high-severity warnings
  tail -n +2 "$file" | jq 'select(.events[].event_type == "High")'
done
```

### Building a Wiki
```bash
# Generate HTML from JSON reports
for file in captures/*.ndjson; do
  python generate_wiki_page.py "$file" > "wiki/$(basename $file .ndjson).html"
done
```

### Statistics Collection
```bash
# Count warnings by severity
tail -n +2 capture.ndjson | jq '.events[] | select(. != null) | .event_type' | sort | uniq -c
```

## Format Specification

The JSON output follows the [NDJSON (Newline Delimited JSON)](http://ndjson.org/) specification:

- **Line 1:** `ReportMetadata` object containing analyzer information
- **Lines 2+:** `AnalysisRow` objects, one per analyzed packet

### ReportMetadata
- `analyzers`: Array of analyzer metadata (name, description, version)
- `rayhunter`: Runtime metadata (version, OS, architecture)
- `report_version`: Format version number

### AnalysisRow
- `packet_timestamp`: ISO 8601 timestamp or null
- `skipped_message_reason`: String if packet was skipped, null otherwise
- `events`: Array of events (can contain null values for analyzers that didn't trigger)

### Event
- `event_type`: One of "Informational", "Low", "Medium", "High"
- `message`: Human-readable description

## Compatibility

The JSON format is compatible with:
- rayhunter daemon v0.9.0+
- Any tool that can parse NDJSON
- Standard JSON parsers (process line-by-line)

## Examples

### Extract All Warnings
```bash
tail -n +2 capture.ndjson | jq -r '
  .events[] |
  select(. != null and .event_type != "Informational") |
  "\(.event_type): \(.message)"
'
```

### Count Messages by Type
```bash
tail -n +2 capture.ndjson | jq -s '
  map(select(.skipped_message_reason != null)) |
  group_by(.skipped_message_reason) |
  map({reason: .[0].skipped_message_reason, count: length})
'
```

### Convert to CSV
```bash
echo "timestamp,severity,message" > warnings.csv
tail -n +2 capture.ndjson | jq -r '
  .packet_timestamp as $ts |
  .events[] |
  select(. != null and .event_type != "Informational") |
  "\($ts),\(.event_type),\(.message)"
' >> warnings.csv
```

## Testing

The implementation includes comprehensive test coverage:

```bash
# Run all tests
cargo test --package rayhunter-check

# Run only JSON reporter tests
cargo test --package rayhunter-check --test json_reporter_tests

# Run only integration tests
cargo test --package rayhunter-check --test integration_tests
```

Test coverage includes:
- JSON serialization/deserialization
- NDJSON format structure
- File I/O operations
- CLI flag validation
- Multiple event types
- Skipped message handling
- Metadata generation

## Implementation Notes

- Default format remains `text` for backward compatibility
- JSON output to stdout when not in `-p` mode
- Individual `.ndjson` files created when analyzing directories
- Format matches daemon's analysis output exactly
- Zero performance impact on text mode
- Fully tested with 14 unit and integration tests
