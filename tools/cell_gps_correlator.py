#!/usr/bin/env python3
"""
Cell Tower GPS Correlator

This tool correlates cell tower observations from QMDL and NDJSON files
with timestamped GPS coordinates to map cellular network activity to locations.
"""

import json
import sys
import argparse
import re
from datetime import datetime, timezone
from pathlib import Path
import csv
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass
import struct

@dataclass
class GpsPoint:
    timestamp: int  # Unix timestamp
    latitude: float
    longitude: float

@dataclass  
class CellObservation:
    timestamp: int  # Unix timestamp
    cell_id: Optional[int] = None
    lac: Optional[int] = None
    tac: Optional[int] = None
    mcc: Optional[int] = None
    mnc: Optional[int] = None
    pci: Optional[int] = None
    rsrp: Optional[int] = None
    rsrq: Optional[int] = None
    rssi: Optional[int] = None
    rat: Optional[str] = None  # Radio Access Technology
    source: str = "unknown"

@dataclass
class CorrelatedObservation:
    cell_obs: CellObservation
    gps_point: Optional[GpsPoint]
    time_diff: float  # Seconds difference between cell and GPS timestamps

class CellGpsCorrelator:
    def __init__(self, time_threshold: int = 30):
        """
        Initialize correlator
        
        Args:
            time_threshold: Maximum time difference in seconds for correlation
        """
        self.time_threshold = time_threshold
        self.gps_points: List[GpsPoint] = []
        self.cell_observations: List[CellObservation] = []
        
    def load_gps_file(self, gps_file: Path) -> None:
        """Load GPS coordinates from .gps file (format: timestamp, lat, lon)"""
        print(f"Loading GPS data from {gps_file}")
        
        with open(gps_file, 'r') as f:
            for line_num, line in enumerate(f, 1):
                line = line.strip()
                if not line:
                    continue
                    
                try:
                    parts = line.split(',')
                    if len(parts) != 3:
                        print(f"Warning: Invalid GPS line format at line {line_num}: {line}")
                        continue
                        
                    timestamp = int(float(parts[0].strip()))
                    latitude = float(parts[1].strip())
                    longitude = float(parts[2].strip())
                    
                    self.gps_points.append(GpsPoint(timestamp, latitude, longitude))
                    
                except (ValueError, IndexError) as e:
                    print(f"Warning: Could not parse GPS line {line_num}: {line} - {e}")
                    
        self.gps_points.sort(key=lambda x: x.timestamp)
        print(f"Loaded {len(self.gps_points)} GPS points")
        
    def load_ndjson_file(self, ndjson_file: Path) -> None:
        """Load cellular observations from NDJSON file"""
        print(f"Loading cellular data from {ndjson_file}")
        
        with open(ndjson_file, 'r') as f:
            for line_num, line in enumerate(f, 1):
                line = line.strip()
                if not line:
                    continue
                    
                try:
                    data = json.loads(line)
                    
                    # Extract timestamp if available
                    timestamp = None
                    if 'timestamp' in data:
                        # Try to parse various timestamp formats
                        ts_str = data['timestamp']
                        if isinstance(ts_str, (int, float)):
                            timestamp = int(ts_str)
                        elif isinstance(ts_str, str):
                            try:
                                # Try parsing as ISO format
                                dt = datetime.fromisoformat(ts_str.replace('Z', '+00:00'))
                                timestamp = int(dt.timestamp())
                            except ValueError:
                                try:
                                    # Try parsing as Unix timestamp
                                    timestamp = int(float(ts_str))
                                except ValueError:
                                    print(f"Warning: Could not parse timestamp: {ts_str}")
                    
                    # If we have timestamp, create cell observation
                    if timestamp:
                        obs = CellObservation(
                            timestamp=timestamp,
                            source="ndjson"
                        )
                        
                        # Extract cellular parameters if available
                        if 'cell_id' in data:
                            obs.cell_id = data['cell_id']
                        if 'lac' in data:
                            obs.lac = data['lac'] 
                        if 'tac' in data:
                            obs.tac = data['tac']
                        if 'mcc' in data:
                            obs.mcc = data['mcc']
                        if 'mnc' in data:
                            obs.mnc = data['mnc']
                        if 'pci' in data:
                            obs.pci = data['pci']
                        if 'rsrp' in data:
                            obs.rsrp = data['rsrp']
                        if 'rsrq' in data:
                            obs.rsrq = data['rsrq']
                        if 'rssi' in data:
                            obs.rssi = data['rssi']
                        if 'rat' in data:
                            obs.rat = data['rat']
                            
                        self.cell_observations.append(obs)
                        
                except json.JSONDecodeError as e:
                    print(f"Warning: Could not parse JSON line {line_num}: {e}")
                    
        print(f"Loaded {len(self.cell_observations)} cellular observations from NDJSON")

    def parse_qmdl_with_scat(self, qmdl_file: Path) -> None:
        """Parse QMDL using SCAT for more reliable cellular info extraction"""
        print(f"Parsing QMDL with SCAT: {qmdl_file}")
        
        import subprocess
        import tempfile
        
        # Create temporary files for SCAT output
        with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as temp_json:
            temp_json_path = temp_json.name
            
        try:
            # Run SCAT to parse QMDL
            scat_cmd = [
                '/Users/beisenmann/miniconda3/bin/scat',
                '-t', 'qc',
                '-d', str(qmdl_file),
                '--json-file', temp_json_path,
                '--events',
                '--msgs'
            ]
            
            print(f"Running SCAT command: {' '.join(scat_cmd)}")
            result = subprocess.run(scat_cmd, capture_output=True, text=True, timeout=30)
            
            if result.returncode != 0:
                print(f"SCAT error: {result.stderr}")
                # Fall back to basic parsing
                self.parse_qmdl_basic(qmdl_file)
                return
                
            # Parse SCAT JSON output
            with open(temp_json_path, 'r') as f:
                scat_data = json.load(f)
                
            observations_found = 0
            
            # Process raw messages from SCAT
            for msg in scat_data.get('raw_messages', []):
                try:
                    # Parse timestamp
                    timestamp_str = msg.get('timestamp', '')
                    if timestamp_str:
                        # Convert ISO timestamp to Unix timestamp
                        dt = datetime.fromisoformat(timestamp_str.replace('Z', '+00:00'))
                        unix_timestamp = int(dt.timestamp())
                        
                        # Create observation
                        obs = CellObservation(
                            timestamp=unix_timestamp,
                            source="scat_qmdl"
                        )
                        
                        # Extract cellular info from hex data
                        data_hex = msg.get('data', '')
                        if data_hex:
                            cellular_info = self.extract_cellular_from_hex(data_hex)
                            if cellular_info:
                                obs.cell_id = cellular_info.get('cell_id')
                                obs.mcc = cellular_info.get('mcc')
                                obs.mnc = cellular_info.get('mnc')
                                obs.tac = cellular_info.get('tac')
                                obs.lac = cellular_info.get('lac')
                                obs.pci = cellular_info.get('pci')
                                
                        self.cell_observations.append(obs)
                        observations_found += 1
                        
                except Exception as e:
                    print(f"Warning: Could not parse SCAT message: {e}")
                    continue
                    
            print(f"Extracted {observations_found} observations from SCAT QMDL parsing")
            
        except subprocess.TimeoutExpired:
            print("SCAT timeout - falling back to basic parsing")
            self.parse_qmdl_basic(qmdl_file)
        except Exception as e:
            print(f"SCAT parsing failed: {e} - falling back to basic parsing")
            self.parse_qmdl_basic(qmdl_file)
        finally:
            # Cleanup temp file
            try:
                Path(temp_json_path).unlink()
            except:
                pass
                
    def extract_cellular_from_hex(self, data_hex: str) -> Optional[Dict]:
        """Extract cellular information from hex data"""
        try:
            # Look for PLMN patterns
            plmn_match = re.search(r'130184|1330f1|130013', data_hex, re.IGNORECASE)
            cellular_info = {}
            
            if plmn_match:
                plmn_hex = plmn_match.group()
                mcc, mnc = self.decode_plmn(plmn_hex)
                if mcc and mnc:
                    cellular_info['mcc'] = mcc
                    cellular_info['mnc'] = mnc
                    
            # Look for cell IDs - convert hex to bytes and search for patterns
            data_bytes = bytes.fromhex(data_hex)
            
            # Search for potential cell IDs (4-byte patterns)
            for i in range(0, len(data_bytes) - 3):
                # Try different encodings
                cell_id_le = struct.unpack('<L', data_bytes[i:i+4])[0]
                if 1000000 <= cell_id_le <= 9999999:  # Valid cell ID range
                    cellular_info['cell_id'] = cell_id_le
                    break
                    
            # Search for TAC/LAC values (2-byte patterns)
            for i in range(0, len(data_bytes) - 1):
                tac_le = struct.unpack('<H', data_bytes[i:i+2])[0]
                if 100 <= tac_le <= 65534:  # Valid TAC range
                    cellular_info['tac'] = tac_le
                    break
                    
            return cellular_info if cellular_info else None
            
        except Exception:
            return None
            
    def decode_plmn(self, plmn_hex: str) -> Tuple[Optional[int], Optional[int]]:
        """Decode PLMN from hex string to MCC/MNC"""
        try:
            if len(plmn_hex) != 6:
                return None, None
                
            bytes_data = bytes.fromhex(plmn_hex)
            
            # PLMN encoding
            mcc_digit_1 = bytes_data[0] & 0x0F
            mcc_digit_2 = (bytes_data[0] & 0xF0) >> 4
            mcc_digit_3 = (bytes_data[1] & 0xF0) >> 4
            
            mnc_digit_1 = bytes_data[2] & 0x0F
            mnc_digit_2 = (bytes_data[2] & 0xF0) >> 4
            mnc_digit_3 = bytes_data[1] & 0x0F
            
            mcc = mcc_digit_1 * 100 + mcc_digit_2 * 10 + mcc_digit_3
            
            if mnc_digit_3 == 0xF:
                mnc = mnc_digit_1 * 10 + mnc_digit_2
            else:
                mnc = mnc_digit_1 * 100 + mnc_digit_2 * 10 + mnc_digit_3
                
            if 100 <= mcc <= 999 and 0 <= mnc <= 999:
                return mcc, mnc
                
        except Exception:
            pass
        return None, None

    def parse_qmdl_basic(self, qmdl_file: Path) -> None:
        """Basic QMDL parsing to extract timestamps and cellular info (fallback)"""
        print(f"Using basic QMDL parsing from {qmdl_file}")
        
        with open(qmdl_file, 'rb') as f:
            data = f.read()
            
        offset = 0
        observations_found = 0
        
        while offset < len(data) - 16:
            try:
                if data[offset:offset+2] == b'\x7E\x00':
                    msg_offset = offset + 4
                    if msg_offset + 12 <= len(data):
                        timestamp_bytes = data[msg_offset:msg_offset+8]
                        timestamp_low = struct.unpack('<L', timestamp_bytes[0:4])[0]
                        timestamp_high = struct.unpack('<L', timestamp_bytes[4:8])[0]
                        
                        full_timestamp = (timestamp_high << 32) | timestamp_low
                        unix_timestamp = int(full_timestamp / 1000000) + 946684800
                        
                        if unix_timestamp > 946684800 and unix_timestamp < 2147483647:
                            obs = CellObservation(
                                timestamp=unix_timestamp,
                                source="qmdl_basic"
                            )
                            self.cell_observations.append(obs)
                            observations_found += 1
                            
                            if observations_found > 1000:
                                break
                                
            except (struct.error, ValueError):
                pass
                
            offset += 1
            
        print(f"Extracted {observations_found} timestamped observations from basic QMDL parsing")
        
    def find_closest_gps(self, timestamp: int) -> Optional[Tuple[GpsPoint, float]]:
        """Find the closest GPS point to a given timestamp"""
        if not self.gps_points:
            return None
            
        min_diff = float('inf')
        closest_gps = None
        
        for gps_point in self.gps_points:
            diff = abs(gps_point.timestamp - timestamp)
            if diff < min_diff:
                min_diff = diff
                closest_gps = gps_point
                
        if min_diff <= self.time_threshold:
            return closest_gps, min_diff
        return None
        
    def correlate_data(self) -> List[CorrelatedObservation]:
        """Correlate cell observations with GPS points"""
        print(f"Correlating {len(self.cell_observations)} cell observations with {len(self.gps_points)} GPS points")
        
        correlated = []
        matched_count = 0
        
        for cell_obs in self.cell_observations:
            gps_result = self.find_closest_gps(cell_obs.timestamp)
            
            if gps_result:
                gps_point, time_diff = gps_result
                correlated.append(CorrelatedObservation(cell_obs, gps_point, time_diff))
                matched_count += 1
            else:
                correlated.append(CorrelatedObservation(cell_obs, None, float('inf')))
                
        print(f"Successfully correlated {matched_count}/{len(self.cell_observations)} observations")
        return correlated
        
    def export_csv(self, correlations: List[CorrelatedObservation], output_file: Path) -> None:
        """Export correlated data to CSV"""
        print(f"Exporting correlated data to {output_file}")
        
        with open(output_file, 'w', newline='') as f:
            writer = csv.writer(f)
            
            # Write header
            header = [
                'cell_timestamp', 'cell_datetime', 'gps_timestamp', 'gps_datetime',
                'latitude', 'longitude', 'time_diff_seconds',
                'cell_id', 'lac', 'tac', 'mcc', 'mnc', 'pci',
                'rsrp', 'rsrq', 'rssi', 'rat', 'source'
            ]
            writer.writerow(header)
            
            # Write data
            for corr in correlations:
                cell = corr.cell_obs
                gps = corr.gps_point
                
                cell_datetime = datetime.fromtimestamp(cell.timestamp, tz=timezone.utc).isoformat()
                
                if gps:
                    gps_datetime = datetime.fromtimestamp(gps.timestamp, tz=timezone.utc).isoformat()
                    row = [
                        cell.timestamp, cell_datetime,
                        gps.timestamp, gps_datetime,
                        gps.latitude, gps.longitude, corr.time_diff,
                        cell.cell_id, cell.lac, cell.tac, cell.mcc, cell.mnc, cell.pci,
                        cell.rsrp, cell.rsrq, cell.rssi, cell.rat, cell.source
                    ]
                else:
                    row = [
                        cell.timestamp, cell_datetime,
                        '', '', '', '', corr.time_diff,
                        cell.cell_id, cell.lac, cell.tac, cell.mcc, cell.mnc, cell.pci,
                        cell.rsrp, cell.rsrq, cell.rssi, cell.rat, cell.source
                    ]
                    
                writer.writerow(row)
                
        print(f"Exported {len(correlations)} correlated observations")

def main():
    parser = argparse.ArgumentParser(description='Correlate cell tower observations with GPS coordinates')
    parser.add_argument('--gps', required=True, help='GPS file (.gps format)')
    parser.add_argument('--ndjson', help='NDJSON file with cellular data')
    parser.add_argument('--qmdl', help='QMDL file with cellular data') 
    parser.add_argument('--output', '-o', default='correlated_data.csv', help='Output CSV file')
    parser.add_argument('--time-threshold', '-t', type=int, default=30, 
                       help='Maximum time difference in seconds for correlation (default: 30)')
    
    args = parser.parse_args()
    
    if not args.ndjson and not args.qmdl:
        print("Error: Must specify either --ndjson or --qmdl file")
        sys.exit(1)
        
    correlator = CellGpsCorrelator(time_threshold=args.time_threshold)
    
    # Load GPS data
    gps_file = Path(args.gps)
    if not gps_file.exists():
        print(f"Error: GPS file {gps_file} not found")
        sys.exit(1)
    correlator.load_gps_file(gps_file)
    
    # Load cellular data
    if args.ndjson:
        ndjson_file = Path(args.ndjson)
        if not ndjson_file.exists():
            print(f"Error: NDJSON file {ndjson_file} not found")
            sys.exit(1)
        correlator.load_ndjson_file(ndjson_file)
        
    if args.qmdl:
        qmdl_file = Path(args.qmdl)
        if not qmdl_file.exists():
            print(f"Error: QMDL file {qmdl_file} not found")
            sys.exit(1)
        correlator.parse_qmdl_with_scat(qmdl_file)
    
    # Correlate and export
    correlations = correlator.correlate_data()
    
    output_file = Path(args.output)
    correlator.export_csv(correlations, output_file)
    
    print(f"\nCorrelation complete! Results saved to {output_file}")
    print(f"Matched {len([c for c in correlations if c.gps_point])} observations with GPS coordinates")

if __name__ == "__main__":
    main()
