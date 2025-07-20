#!/usr/bin/env python3
"""
Enhanced QMDL Cell Tower Parser for Rayhunter

This tool specifically handles the log codes found in your QMDL file
and attempts to extract cellular network information for GPS correlation.
"""

import struct
import json
import csv
from datetime import datetime, timezone
from pathlib import Path
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass

@dataclass
class CellularEvent:
    timestamp: int  # Unix timestamp
    log_code: int
    raw_data: bytes
    parsed_info: Dict = None

# Known log codes from your QMDL analysis
QMDL_LOG_CODES = {
    0x0017: "Unknown_0x0017",
    0x0026: "Unknown_0x0026", 
    0x0028: "Unknown_0x0028",
    0x0029: "Unknown_0x0029",
    0x002a: "Unknown_0x002a",
    0x0031: "Unknown_0x0031",
    0x0039: "Unknown_0x0039",
    0x0048: "Unknown_0x0048"
}

class EnhancedQMDLParser:
    def __init__(self, qmdl_file: Path):
        self.qmdl_file = qmdl_file
        self.cellular_events: List[CellularEvent] = []
        
    def parse_qmdl_file(self) -> List[CellularEvent]:
        """Parse QMDL file with enhanced log code recognition"""
        print(f"Parsing QMDL file: {self.qmdl_file}")
        
        with open(self.qmdl_file, 'rb') as f:
            data = f.read()
            
        # Look for frame delimiters (0x7E)
        frame_positions = []
        for i in range(len(data) - 1):
            if data[i] == 0x7E:
                frame_positions.append(i)
                
        print(f"Found {len(frame_positions)} potential frames")
        
        for i, frame_start in enumerate(frame_positions[:-1]):
            frame_end = frame_positions[i + 1]
            frame_data = data[frame_start:frame_end]
            
            if len(frame_data) < 16:  # Minimum frame size
                continue
                
            try:
                event = self.parse_frame(frame_data)
                if event:
                    self.cellular_events.append(event)
            except Exception as e:
                continue  # Skip problematic frames
                
        print(f"Extracted {len(self.cellular_events)} cellular events")
        return self.cellular_events
        
    def parse_frame(self, frame_data: bytes) -> Optional[CellularEvent]:
        """Parse individual QMDL frame"""
        if len(frame_data) < 16:
            return None
            
        # Skip frame delimiter
        offset = 1
        
        try:
            # Extract length and log code (based on inspector output)
            length = struct.unpack('<H', frame_data[offset:offset+2])[0]
            log_code = struct.unpack('<H', frame_data[offset+2:offset+4])[0]
            
            # Check if this is a known log code
            if log_code not in QMDL_LOG_CODES:
                return None
                
            # Skip to timestamp area (varies by format)
            ts_offset = offset + 8
            if ts_offset + 8 <= len(frame_data):
                # Try to extract timestamp (this is approximate)
                ts_low = struct.unpack('<L', frame_data[ts_offset:ts_offset+4])[0]
                ts_high = struct.unpack('<L', frame_data[ts_offset+4:ts_offset+8])[0]
                
                # Convert to approximate Unix timestamp
                # This conversion is based on QMDL timestamp format
                qmdl_timestamp = (ts_high << 32) | ts_low
                
                # Rough conversion (may need calibration)
                # QMDL often uses microseconds since some epoch
                unix_timestamp = int(qmdl_timestamp / 1000000) + 946684800  # Approx conversion
                
                # Validate timestamp is reasonable (between 2020-2030)
                if unix_timestamp < 1577836800 or unix_timestamp > 1893456000:
                    # Try alternative timestamp extraction
                    unix_timestamp = int(qmdl_timestamp / 1000) + 1577836800  # Alternative
                    
                if unix_timestamp < 1577836800 or unix_timestamp > 1893456000:
                    return None  # Skip unreasonable timestamps
                    
                return CellularEvent(
                    timestamp=unix_timestamp,
                    log_code=log_code,
                    raw_data=frame_data[offset:],
                    parsed_info={"log_type": QMDL_LOG_CODES[log_code]}
                )
                
        except struct.error:
            pass
            
        return None
        
    def correlate_with_gps(self, gps_file: Path, time_threshold: int = 30) -> List[Dict]:
        """Correlate cellular events with GPS coordinates"""
        print(f"Loading GPS data from {gps_file}")
        
        # Load GPS data
        gps_points = []
        with open(gps_file, 'r') as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    parts = line.split(',')
                    if len(parts) == 3:
                        timestamp = int(float(parts[0].strip()))
                        lat = float(parts[1].strip())
                        lon = float(parts[2].strip())
                        gps_points.append((timestamp, lat, lon))
                except (ValueError, IndexError):
                    continue
                    
        print(f"Loaded {len(gps_points)} GPS points")
        
        # Correlate events with GPS
        correlations = []
        for event in self.cellular_events:
            best_gps = None
            min_diff = float('inf')
            
            for gps_ts, lat, lon in gps_points:
                diff = abs(event.timestamp - gps_ts)
                if diff < min_diff:
                    min_diff = diff
                    best_gps = (gps_ts, lat, lon)
                    
            if min_diff <= time_threshold:
                correlation = {
                    'cellular_timestamp': event.timestamp,
                    'cellular_datetime': datetime.fromtimestamp(event.timestamp, tz=timezone.utc).isoformat(),
                    'gps_timestamp': best_gps[0],
                    'gps_datetime': datetime.fromtimestamp(best_gps[0], tz=timezone.utc).isoformat(),
                    'latitude': best_gps[1],
                    'longitude': best_gps[2],
                    'time_diff_seconds': min_diff,
                    'log_code': f"0x{event.log_code:04x}",
                    'log_type': event.parsed_info.get('log_type', 'unknown'),
                    'raw_data_length': len(event.raw_data)
                }
            else:
                correlation = {
                    'cellular_timestamp': event.timestamp,
                    'cellular_datetime': datetime.fromtimestamp(event.timestamp, tz=timezone.utc).isoformat(),
                    'gps_timestamp': None,
                    'gps_datetime': None,
                    'latitude': None,
                    'longitude': None,
                    'time_diff_seconds': None,
                    'log_code': f"0x{event.log_code:04x}",
                    'log_type': event.parsed_info.get('log_type', 'unknown'),
                    'raw_data_length': len(event.raw_data)
                }
                
            correlations.append(correlation)
            
        print(f"Correlated {len([c for c in correlations if c['latitude']])} events with GPS")
        return correlations
        
    def export_correlations(self, correlations: List[Dict], output_file: Path):
        """Export correlations to CSV"""
        print(f"Exporting to {output_file}")
        
        with open(output_file, 'w', newline='') as f:
            if correlations:
                writer = csv.DictWriter(f, fieldnames=correlations[0].keys())
                writer.writeheader()
                writer.writerows(correlations)
                
        print(f"Exported {len(correlations)} correlations")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description='Enhanced QMDL GPS Correlator')
    parser.add_argument('--qmdl', required=True, help='QMDL file path')
    parser.add_argument('--gps', required=True, help='GPS file path')
    parser.add_argument('--output', '-o', default='enhanced_correlation.csv', help='Output CSV file')
    parser.add_argument('--time-threshold', '-t', type=int, default=30, help='Time threshold in seconds')
    
    args = parser.parse_args()
    
    qmdl_file = Path(args.qmdl)
    gps_file = Path(args.gps)
    output_file = Path(args.output)
    
    if not qmdl_file.exists():
        print(f"Error: QMDL file {qmdl_file} not found")
        return 1
        
    if not gps_file.exists():
        print(f"Error: GPS file {gps_file} not found")
        return 1
        
    parser = EnhancedQMDLParser(qmdl_file)
    events = parser.parse_qmdl_file()
    
    if not events:
        print("No cellular events found in QMDL file")
        return 1
        
    correlations = parser.correlate_with_gps(gps_file, args.time_threshold)
    parser.export_correlations(correlations, output_file)
    
    print(f"\nCorrelation complete!")
    print(f"Total cellular events: {len(events)}")
    print(f"Successfully correlated: {len([c for c in correlations if c['latitude']])}")
    print(f"Results saved to: {output_file}")
    
    return 0

if __name__ == "__main__":
    exit(main())
