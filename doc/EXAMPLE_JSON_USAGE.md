# Example: Using rayhunter-check with JSON Output

This example demonstrates the new JSON output feature for rayhunter-check.

## Running the Tool

```bash
# Text output (default)
rayhunter-check -p /path/to/captures

# JSON output to stdout
rayhunter-check -p /path/to/captures --format json

# With quiet mode (suppresses info logs)
rayhunter-check -p /path/to/captures --format json --quiet
```

## Example JSON Output Structure

When you run `rayhunter-check --format json`, the output follows this structure:

### Line 1: Metadata
```json
{
  "analyzers": [
    {
      "name": "IMSI Requested",
      "description": "Detects when a 2G or 3G network requests the IMSI...",
      "version": 1
    },
    {
      "name": "Connection Redirect to 2G",
      "description": "Detects when LTE connection is downgraded to 2G...",
      "version": 1
    }
    // ... more analyzers
  ],
  "rayhunter": {
    "rayhunter_version": "0.9.0",
    "system_os": "Linux",
    "arch": "x86_64"
  },
  "report_version": 2
}
```

### Lines 2+: Analysis Rows

Each line represents one analyzed packet:

#### Packet with warning:
```json
{
  "packet_timestamp": "2024-01-15T14:30:45.123456+00:00",
  "skipped_message_reason": null,
  "events": [
    {
      "event_type": "High",
      "message": "IMSI was requested by the network (packet 42)"
    },
    null,
    null
  ]
}
```

#### Skipped packet:
```json
{
  "packet_timestamp": "2024-01-15T14:30:46.234567+00:00",
  "skipped_message_reason": "Failed to parse GSMTAP header: InvalidData",
  "events": []
}
```

#### Packet with multiple events:
```json
{
  "packet_timestamp": "2024-01-15T14:30:47.345678+00:00",
  "skipped_message_reason": null,
  "events": [
    null,
    {
      "event_type": "Medium",
      "message": "Connection redirect to 2G detected (packet 43)"
    },
    {
      "event_type": "Low",
      "message": "Incomplete SIB information (packet 43)"
    }
  ]
}
```

## Processing JSON Output

### Extract high-severity warnings
```bash
rayhunter-check -p captures/ --format json --quiet | \
  tail -n +2 | \
  jq -r 'select(.events[]?.event_type == "High") |
         "\(.packet_timestamp): \(.events[].message)"'
```

### Count warnings by severity
```bash
rayhunter-check -p capture.qmdl --format json --quiet | \
  tail -n +2 | \
  jq -r '.events[] | select(. != null) | .event_type' | \
  sort | uniq -c
```

Output:
```
  15 High
  23 Medium
  42 Low
  156 Informational
```

### Generate summary statistics
```bash
rayhunter-check -p capture.qmdl --format json --quiet | \
  tail -n +2 | \
  jq -s '{
    total_packets: length,
    skipped: [.[] | select(.skipped_message_reason != null)] | length,
    warnings: [.[] | .events[] | select(. != null and .event_type != "Informational")] | length
  }'
```

Output:
```json
{
  "total_packets": 1523,
  "skipped": 48,
  "warnings": 80
}
```

## File Output

When processing directories, individual `.ndjson` files are created:

```bash
rayhunter-check -p captures/ --format json
```

Creates:
```
captures/
├── capture1.qmdl
├── capture1.ndjson      # JSON analysis
├── capture2.pcapng
└── capture2.ndjson      # JSON analysis
```

Each `.ndjson` file has the same structure as stdout output.

## Integrating with Other Tools

### Python
```python
import json

with open('capture.ndjson', 'r') as f:
    lines = f.readlines()

metadata = json.loads(lines[0])
print(f"Analyzed with {len(metadata['analyzers'])} analyzers")

for line in lines[1:]:
    row = json.loads(line)
    if row['skipped_message_reason']:
        continue

    for event in row['events']:
        if event and event['event_type'] in ['High', 'Medium']:
            print(f"{row['packet_timestamp']}: {event['message']}")
```

### JavaScript/Node.js
```javascript
const fs = require('fs');

const lines = fs.readFileSync('capture.ndjson', 'utf-8').split('\n');
const metadata = JSON.parse(lines[0]);
const rows = lines.slice(1).filter(l => l).map(l => JSON.parse(l));

const warnings = rows.flatMap(row =>
  row.events.filter(e => e && e.event_type !== 'Informational')
);

console.log(`Found ${warnings.length} warnings`);
```

## Compatibility

The JSON format is identical to the format used by the rayhunter daemon, making it easy to:
- Process on-device and off-device analyses with the same tools
- Build UIs that work with both sources
- Share analysis results in a standard format
- Archive and version control analysis data
