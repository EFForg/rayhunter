#!/usr/bin/env python3
"""
Enhanced Cellular Data Extractor

Comprehensive tool to extract detailed cellular information from QMDL files
including cell tower details, signal measurements, and network parameters.
"""

import struct
import json
import sys
import argparse
from pathlib import Path
from typing import List, Dict, Optional, Any
from dataclasses import dataclass, asdict
from datetime import datetime, timezone
import binascii

@dataclass
class DetailedCellInfo:
    timestamp: int
    datetime_str: str
    
    # Primary Cell Identity
    cell_id: Optional[int] = None
    physical_cell_id: Optional[int] = None  # PCI
    tracking_area_code: Optional[int] = None  # TAC
    location_area_code: Optional[int] = None  # LAC
    
    # Network Identity
    mobile_country_code: Optional[int] = None  # MCC
    mobile_network_code: Optional[int] = None  # MNC
    operator_name: Optional[str] = None
    
    # Radio Access Technology
    radio_access_tech: Optional[str] = None  # LTE, UMTS, GSM, NR
    frequency_band: Optional[int] = None
    channel_number: Optional[int] = None  # EARFCN/UARFCN/ARFCN
    bandwidth: Optional[str] = None
    
    # Signal Measurements (LTE)
    rsrp_dbm: Optional[float] = None  # Reference Signal Received Power
    rsrq_db: Optional[float] = None   # Reference Signal Received Quality
    rssi_dbm: Optional[float] = None  # Received Signal Strength Indicator
    sinr_db: Optional[float] = None   # Signal to Interference plus Noise Ratio
    cqi: Optional[int] = None         # Channel Quality Indicator
    
    # Signal Measurements (UMTS/3G)
    rscp_dbm: Optional[float] = None  # Received Signal Code Power
    ecno_db: Optional[float] = None   # Energy per Chip to Noise ratio
    
    # Signal Measurements (GSM/2G)
    rxlev_dbm: Optional[float] = None # Received Level
    rxqual: Optional[int] = None      # Received Quality
    
    # 5G NR Measurements
    ss_rsrp_dbm: Optional[float] = None  # SS-RSRP
    ss_rsrq_db: Optional[float] = None   # SS-RSRQ
    ss_sinr_db: Optional[float] = None   # SS-SINR
    
    # Neighbor Cells
    neighbor_cells: Optional[List[Dict]] = None
    
    # Additional Parameters
    transmission_mode: Optional[str] = None
    mimo_layers: Optional[int] = None
    ca_bands: Optional[List[int]] = None  # Carrier Aggregation bands
    
    # Network State
    connection_state: Optional[str] = None  # RRC state
    attach_status: Optional[str] = None
    registration_state: Optional[str] = None
    
    # Location Services
    timing_advance: Optional[int] = None
    
    # Raw data for debugging
    message_type: Optional[str] = None
    raw_data_hex: Optional[str] = None

class EnhancedQMDLParser:
    def __init__(self):
        self.cell_data: List[DetailedCellInfo] = []
        
        # Known QMDL message types that contain cellular info
        self.cellular_message_types = {
            # LTE RRC messages
            0x0017: "LTE_RRC_OTA_MSG",
            0x0018: "LTE_RRC_MEAS_REPORT", 
            0x0019: "LTE_RRC_SERVING_CELL_INFO",
            0x001A: "LTE_ML1_SERVING_CELL_MEAS",
            0x001B: "LTE_ML1_NEIGHBOR_MEAS",
            0x001C: "LTE_ML1_CELL_RESEL",
            
            # NAS/EMM messages
            0x0020: "NAS_EMM_OTA_INCOMING_MSG",
            0x0021: "NAS_EMM_OTA_OUTGOING_MSG", 
            0x0022: "NAS_ESM_OTA_INCOMING_MSG",
            0x0023: "NAS_ESM_OTA_OUTGOING_MSG",
            
            # UMTS/WCDMA messages
            0x0030: "WCDMA_RRC_OTA_MSG",
            0x0031: "WCDMA_CELL_ID",
            0x0032: "WCDMA_RRC_STATES",
            0x0033: "WCDMA_MEASUREMENT_REPORT",
            
            # GSM messages
            0x0040: "GSM_RR_CELL_INFO",
            0x0041: "GSM_MEASUREMENT_REPORT",
            0x0042: "GSM_CELL_SELECTION",
            
            # 5G NR messages
            0x0050: "NR_RRC_OTA_MSG",
            0x0051: "NR_ML1_SERVING_CELL_MEAS",
            0x0052: "NR_ML1_NEIGHBOR_MEAS",
            
            # Physical layer measurements
            0x0060: "LTE_PHY_CONNECTED_MODE_MEAS",
            0x0061: "LTE_PHY_IDLE_MODE_MEAS",
            0x0062: "LTE_PHY_NEIGHBOR_CELL_MEAS",
            
            # Additional cellular messages found in logs
            0x0048: "LTE_CPHY_SERVING_CELL_MEAS",
            0x004A: "LTE_CPHY_NEIGHBOR_CELL_MEAS"
        }
        
    def parse_timestamp(self, timestamp_bytes: bytes) -> int:
        """Parse QMDL timestamp to Unix timestamp"""
        if len(timestamp_bytes) < 8:
            return 0
            
        try:
            # QMDL uses a 64-bit timestamp
            timestamp_low = struct.unpack('<L', timestamp_bytes[0:4])[0]
            timestamp_high = struct.unpack('<L', timestamp_bytes[4:8])[0]
            full_timestamp = (timestamp_high << 32) | timestamp_low
            
            # Convert from QMDL time base (seems to be in microseconds from some epoch)
            # This is an approximation and may need calibration
            if full_timestamp > 1000000000000000:  # Microseconds
                unix_timestamp = int(full_timestamp / 1000000) - 631152000  # Adjust epoch
            else:
                unix_timestamp = int(full_timestamp / 1000) + 946684800  # Alternative conversion
                
            # Validate timestamp is reasonable (between 2020-2030)
            if unix_timestamp < 1577836800 or unix_timestamp > 1893456000:
                # Try alternative conversion
                unix_timestamp = int(full_timestamp / 1000000) + 1262304000
                
            return unix_timestamp
            
        except (struct.error, ValueError):
            return 0
            
    def parse_lte_serving_cell(self, data: bytes, offset: int) -> Optional[DetailedCellInfo]:
        """Parse LTE serving cell information"""
        try:
            if offset + 64 > len(data):
                return None
                
            timestamp = self.parse_timestamp(data[offset:offset+8])
            if timestamp == 0:
                return None
                
            cell_info = DetailedCellInfo(
                timestamp=timestamp,
                datetime_str=datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat(),
                message_type="LTE_SERVING_CELL",
                radio_access_tech="LTE"
            )
            
            # Parse cellular parameters (simplified - real parsing is more complex)
            try:
                # Cell ID (4 bytes)
                if offset + 12 <= len(data):
                    cell_info.cell_id = struct.unpack('<L', data[offset+8:offset+12])[0] & 0xFFFFFF
                    
                # PCI (2 bytes)
                if offset + 14 <= len(data):
                    cell_info.physical_cell_id = struct.unpack('<H', data[offset+12:offset+14])[0] & 0x1FF
                    
                # TAC (2 bytes)  
                if offset + 16 <= len(data):
                    cell_info.tracking_area_code = struct.unpack('<H', data[offset+14:offset+16])[0]
                    
                # EARFCN (2 bytes)
                if offset + 18 <= len(data):
                    cell_info.channel_number = struct.unpack('<H', data[offset+16:offset+18])[0]
                    
                # Signal measurements (approximated locations)
                if offset + 24 <= len(data):
                    rsrp_raw = struct.unpack('<h', data[offset+20:offset+22])[0]
                    if rsrp_raw != 0x8000:  # Invalid value marker
                        cell_info.rsrp_dbm = (rsrp_raw - 140.0) / 4.0  # Convert to dBm
                        
                if offset + 26 <= len(data):
                    rsrq_raw = struct.unpack('<h', data[offset+22:offset+24])[0]
                    if rsrq_raw != 0x8000:
                        cell_info.rsrq_db = (rsrq_raw - 40.0) / 8.0  # Convert to dB
                        
                # Store raw data for debugging
                cell_info.raw_data_hex = binascii.hexlify(data[offset:offset+32]).decode()
                
            except (struct.error, IndexError):
                pass
                
            return cell_info
            
        except Exception:
            return None
            
    def parse_measurement_report(self, data: bytes, offset: int) -> Optional[DetailedCellInfo]:
        """Parse measurement report with neighbor cell info"""
        try:
            timestamp = self.parse_timestamp(data[offset:offset+8])
            if timestamp == 0:
                return None
                
            cell_info = DetailedCellInfo(
                timestamp=timestamp,
                datetime_str=datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat(),
                message_type="MEASUREMENT_REPORT",
                neighbor_cells=[]
            )
            
            # Parse neighbor cell measurements (simplified)
            neighbors_offset = offset + 16
            neighbor_count = min(8, (len(data) - neighbors_offset) // 12)  # Max 8 neighbors
            
            for i in range(neighbor_count):
                neighbor_offset = neighbors_offset + (i * 12)
                if neighbor_offset + 12 <= len(data):
                    try:
                        neighbor_pci = struct.unpack('<H', data[neighbor_offset:neighbor_offset+2])[0] & 0x1FF
                        neighbor_rsrp = struct.unpack('<h', data[neighbor_offset+2:neighbor_offset+4])[0]
                        neighbor_rsrq = struct.unpack('<h', data[neighbor_offset+4:neighbor_offset+6])[0]
                        
                        if neighbor_pci != 0x1FF:  # Valid PCI
                            neighbor = {
                                'pci': neighbor_pci,
                                'rsrp_dbm': (neighbor_rsrp - 140.0) / 4.0 if neighbor_rsrp != 0x8000 else None,
                                'rsrq_db': (neighbor_rsrq - 40.0) / 8.0 if neighbor_rsrq != 0x8000 else None
                            }
                            cell_info.neighbor_cells.append(neighbor)
                    except (struct.error, IndexError):
                        continue
                        
            return cell_info
            
        except Exception:
            return None
            
    def parse_qmdl_file(self, qmdl_file: Path) -> None:
        """Parse QMDL file and extract detailed cellular information"""
        print(f"Parsing QMDL file for detailed cellular data: {qmdl_file}")
        
        with open(qmdl_file, 'rb') as f:
            data = f.read()
            
        offset = 0
        messages_found = 0
        
        while offset < len(data) - 16:
            try:
                # Look for QMDL message frames
                if data[offset:offset+2] == b'\x7E\x00':  # QMDL frame start
                    # Parse message header
                    if offset + 16 <= len(data):
                        msg_len = struct.unpack('<H', data[offset+2:offset+4])[0]
                        msg_type = struct.unpack('<H', data[offset+4:offset+6])[0]
                        
                        # Check if this is a cellular message type we're interested in
                        if msg_type in self.cellular_message_types:
                            msg_name = self.cellular_message_types[msg_type]
                            
                            cell_info = None
                            if "SERVING_CELL" in msg_name or "CELL_INFO" in msg_name:
                                cell_info = self.parse_lte_serving_cell(data, offset + 6)
                            elif "MEAS" in msg_name:
                                cell_info = self.parse_measurement_report(data, offset + 6)
                            else:
                                # Generic cellular message parsing
                                timestamp = self.parse_timestamp(data[offset+6:offset+14])
                                if timestamp > 0:
                                    cell_info = DetailedCellInfo(
                                        timestamp=timestamp,
                                        datetime_str=datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat(),
                                        message_type=msg_name,
                                        raw_data_hex=binascii.hexlify(data[offset:offset+min(32, msg_len)]).decode()
                                    )
                            
                            if cell_info:
                                self.cell_data.append(cell_info)
                                messages_found += 1
                                
                                # Limit results to prevent memory issues
                                if messages_found >= 5000:
                                    break
                                    
                        offset += max(4, msg_len)
                    else:
                        offset += 1
                else:
                    offset += 1
                    
            except (struct.error, ValueError, IndexError):
                offset += 1
                
        print(f"Extracted {messages_found} detailed cellular messages from QMDL")
        
        # Sort by timestamp
        self.cell_data.sort(key=lambda x: x.timestamp)
        
    def export_detailed_json(self, output_file: Path) -> None:
        """Export detailed cellular data to JSON"""
        print(f"Exporting {len(self.cell_data)} detailed cellular records to {output_file}")
        
        export_data = {
            'extraction_metadata': {
                'timestamp': datetime.now(timezone.utc).isoformat(),
                'total_records': len(self.cell_data),
                'message_types': list(set(record.message_type for record in self.cell_data if record.message_type)),
                'time_range': {
                    'start': self.cell_data[0].datetime_str if self.cell_data else None,
                    'end': self.cell_data[-1].datetime_str if self.cell_data else None
                }
            },
            'cellular_records': [asdict(record) for record in self.cell_data]
        }
        
        with open(output_file, 'w') as f:
            json.dump(export_data, f, indent=2, default=str)
            
        print(f"Detailed cellular data exported to {output_file}")

def main():
    parser = argparse.ArgumentParser(description='Extract detailed cellular information from QMDL files')
    parser.add_argument('--qmdl', required=True, help='QMDL file to parse')
    parser.add_argument('--output', '-o', default='detailed_cellular_data.json', help='Output JSON file')
    
    args = parser.parse_args()
    
    qmdl_file = Path(args.qmdl)
    if not qmdl_file.exists():
        print(f"Error: QMDL file {qmdl_file} not found")
        sys.exit(1)
        
    parser = EnhancedQMDLParser()
    parser.parse_qmdl_file(qmdl_file)
    
    output_file = Path(args.output)
    parser.export_detailed_json(output_file)
    
    print(f"\nExtraction complete! {len(parser.cell_data)} cellular records extracted")

if __name__ == "__main__":
    main()
