# JSON Output Format Examples

This document shows real examples of the JSON output format from rayhunter-check.

## Complete NDJSON Output Example

Below is a complete example of what `rayhunter-check --format json` produces:

```json
{"analyzers":[{"name":"IMSI Requested","description":"Detects when a 2G or 3G network requests the IMSI from the device. This can be a sign that an IMSI catcher is present, as they often lack legitimate user databases and must request the IMSI to function. However, there are some legitimate cases where a network may request the IMSI, such as when a device first connects to a network or when the network is upgrading its infrastructure.","version":1},{"name":"Connection Redirect to 2G","description":"Detects when an LTE connection is redirected to a 2G network. This can be a sign of an IMSI catcher, as they often operate on 2G networks to exploit weaker security. However, this can also happen legitimately if the device moves out of LTE coverage.","version":1},{"name":"LTE Priority 2G Downgrade","description":"Detects when an LTE network broadcasts SIB6 or SIB7 messages that configure UE to deprioritize LTE in favor of 2G networks. This can be a sign of an IMSI catcher attempting to push devices onto weaker 2G networks.","version":1},{"name":"Null/Weak Cipher","description":"Detects when a network configures a null or weak cipher for encrypting communications. IMSI catchers commonly use null ciphers to intercept traffic.","version":1},{"name":"NAS Null/Weak Cipher","description":"Detects when a network configures a null or weak cipher for Non-Access Stratum (NAS) messages. This can indicate an attempt to intercept sensitive signaling data.","version":1},{"name":"Incomplete SIB","description":"Detects when System Information Block (SIB) messages are incomplete or malformed. IMSI catchers may broadcast incomplete SIBs as they often don't implement the full cellular protocol correctly.","version":1}],"rayhunter":{"rayhunter_version":"0.9.0","system_os":"Darwin","arch":"aarch64"},"report_version":2}
{"packet_timestamp":"2024-11-15T10:30:45.123456+00:00","skipped_message_reason":null,"events":[null,null,null,null,null,null]}
{"packet_timestamp":"2024-11-15T10:30:45.234567+00:00","skipped_message_reason":null,"events":[{"event_type":"High","message":"IMSI was requested by the network (packet 2)"},null,null,null,null,null]}
{"packet_timestamp":"2024-11-15T10:30:45.345678+00:00","skipped_message_reason":"Failed to parse GSMTAP header: InvalidData","events":[]}
{"packet_timestamp":"2024-11-15T10:30:45.456789+00:00","skipped_message_reason":null,"events":[null,{"event_type":"Medium","message":"Connection redirect to 2G detected (packet 4)"},null,null,null,null]}
{"packet_timestamp":"2024-11-15T10:30:45.567890+00:00","skipped_message_reason":null,"events":[null,null,null,{"event_type":"High","message":"Null cipher configured for encryption (packet 5)"},null,null]}
{"packet_timestamp":"2024-11-15T10:30:45.678901+00:00","skipped_message_reason":null,"events":[null,null,null,null,null,{"event_type":"Low","message":"Incomplete SIB message detected (packet 6)"}]}
{"packet_timestamp":"2024-11-15T10:30:45.789012+00:00","skipped_message_reason":null,"events":[{"event_type":"Informational","message":"Network identity information (packet 7)"},null,null,null,null,null]}
```

## Pretty-Printed Examples

### Metadata Object (Line 1)
```json
{
  "analyzers": [
    {
      "name": "IMSI Requested",
      "description": "Detects when a 2G or 3G network requests the IMSI from the device...",
      "version": 1
    },
    {
      "name": "Connection Redirect to 2G",
      "description": "Detects when an LTE connection is redirected to a 2G network...",
      "version": 1
    }
    // ... more analyzers
  ],
  "rayhunter": {
    "rayhunter_version": "0.9.0",
    "system_os": "Darwin",
    "arch": "aarch64"
  },
  "report_version": 2
}
```

### Normal Packet (No Issues)
```json
{
  "packet_timestamp": "2024-11-15T10:30:45.123456+00:00",
  "skipped_message_reason": null,
  "events": [null, null, null, null, null, null]
}
```
- All analyzers ran but found nothing to report
- Events array has one entry per analyzer
- `null` means that analyzer had no findings

### Packet with High Severity Warning
```json
{
  "packet_timestamp": "2024-11-15T10:30:45.234567+00:00",
  "skipped_message_reason": null,
  "events": [
    {
      "event_type": "High",
      "message": "IMSI was requested by the network (packet 2)"
    },
    null,
    null,
    null,
    null,
    null
  ]
}
```
- First analyzer (IMSI Requested) triggered with high severity
- Other analyzers found nothing

### Skipped Packet
```json
{
  "packet_timestamp": "2024-11-15T10:30:45.345678+00:00",
  "skipped_message_reason": "Failed to parse GSMTAP header: InvalidData",
  "events": []
}
```
- Packet couldn't be analyzed due to parsing error
- Events array is empty
- `skipped_message_reason` contains the error

### Multiple Warnings in One Packet
```json
{
  "packet_timestamp": "2024-11-15T10:30:46.123456+00:00",
  "skipped_message_reason": null,
  "events": [
    {
      "event_type": "High",
      "message": "IMSI was requested by the network (packet 10)"
    },
    {
      "event_type": "Medium",
      "message": "Connection redirect to 2G detected (packet 10)"
    },
    null,
    {
      "event_type": "High",
      "message": "Null cipher configured for encryption (packet 10)"
    },
    null,
    null
  ]
}
```
- Multiple analyzers triggered on the same packet
- Different severity levels
- Still maintains one entry per analyzer in order

## Event Severity Levels

```json
// Informational - No threat, just information
{
  "event_type": "Informational",
  "message": "Network identity information (packet 42)"
}

// Low - Minor concern, may be benign
{
  "event_type": "Low",
  "message": "Incomplete SIB message detected (packet 42)"
}

// Medium - Suspicious activity worth investigating
{
  "event_type": "Medium",
  "message": "Connection redirect to 2G detected (packet 42)"
}

// High - Strong indicator of IMSI catcher
{
  "event_type": "High",
  "message": "IMSI was requested by the network (packet 42)"
}
```

## File Output Format

When using `-p` flag with a directory, each capture gets its own `.ndjson` file:

**Input:**
```
captures/
├── morning_commute.qmdl
├── evening_commute.qmdl
└── airport.pcapng
```

**After running `rayhunter-check -p captures/ --format json`:**
```
captures/
├── morning_commute.qmdl
├── morning_commute.ndjson    ← JSON analysis
├── evening_commute.qmdl
├── evening_commute.ndjson    ← JSON analysis
├── airport.pcapng
└── airport.ndjson            ← JSON analysis
```

Each `.ndjson` file has the same structure: metadata on line 1, followed by one row per analyzed packet.

## Parsing Examples

### Count packets by severity (bash + jq)
```bash
tail -n +2 capture.ndjson | \
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

### Extract all high-severity messages (bash + jq)
```bash
tail -n +2 capture.ndjson | \
  jq -r 'select(.events[]?.event_type == "High") |
         "\(.packet_timestamp): \(.events[] | select(.event_type == "High") | .message)"'
```

Output:
```
2024-11-15T10:30:45.234567+00:00: IMSI was requested by the network (packet 2)
2024-11-15T10:30:45.567890+00:00: Null cipher configured for encryption (packet 5)
2024-11-15T10:30:46.123456+00:00: IMSI was requested by the network (packet 10)
```

### Python parsing example
```python
import json
from collections import Counter

with open('capture.ndjson', 'r') as f:
    lines = f.readlines()

metadata = json.loads(lines[0])
print(f"Analyzers: {[a['name'] for a in metadata['analyzers']]}")

severities = Counter()
for line in lines[1:]:
    row = json.loads(line)
    for event in row['events']:
        if event:
            severities[event['event_type']] += 1

print(f"\nWarnings by severity:")
for severity, count in severities.most_common():
    print(f"  {severity}: {count}")
```

## Notes

- The format is **Newline Delimited JSON (NDJSON)**, not a single JSON array
- Each line is a valid JSON object
- Line 1 is always metadata
- Lines 2+ are analysis rows
- The `events` array always has one entry per analyzer, in the same order as the metadata
- `null` in the events array means that analyzer didn't trigger
- Empty `events` array only appears when a packet was skipped
- Timestamps use ISO 8601 format with timezone
- The format is identical to rayhunter daemon's output
