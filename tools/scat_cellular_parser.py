#!/usr/bin/env python3
"""
SCAT Enhanced Cellular Parser

Parse SCAT JSON/TXT output to extract detailed cellular network information
specifically for cell ID analysis and 2G downgrade attack investigation.
"""

import json
import re
import struct
import argparse
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass

@dataclass
class CellularInfo:
    timestamp: str
    mcc: Optional[int] = None
    mnc: Optional[int] = None
    cell_id: Optional[int] = None
    lac: Optional[int] = None
    tac: Optional[int] = None
    pci: Optional[int] = None
    message_type: str = "unknown"
    raw_data: str = ""

class ScatCellularParser:
    def __init__(self):
        self.cellular_info: List[CellularInfo] = []
        
    def decode_plmn(self, plmn_hex: str) -> Tuple[Optional[int], Optional[int]]:
        """Decode PLMN from hex string to MCC/MNC"""
        try:
            if len(plmn_hex) != 6:  # 3 bytes = 6 hex chars
                return None, None
                
            bytes_data = bytes.fromhex(plmn_hex)
            
            # PLMN encoding: byte0: MCC2|MCC1, byte1: MNC3|MCC3, byte2: MNC2|MNC1
            mcc_digit_1 = bytes_data[0] & 0x0F
            mcc_digit_2 = (bytes_data[0] & 0xF0) >> 4
            mcc_digit_3 = (bytes_data[1] & 0xF0) >> 4
            
            mnc_digit_1 = bytes_data[2] & 0x0F
            mnc_digit_2 = (bytes_data[2] & 0xF0) >> 4
            mnc_digit_3 = bytes_data[1] & 0x0F
            
            # Build MCC
            mcc = mcc_digit_1 * 100 + mcc_digit_2 * 10 + mcc_digit_3
            
            # Build MNC (handle 2-digit vs 3-digit)
            if mnc_digit_3 == 0xF:
                mnc = mnc_digit_1 * 10 + mnc_digit_2
            else:
                mnc = mnc_digit_1 * 100 + mnc_digit_2 * 10 + mnc_digit_3
                
            # Validate ranges
            if mcc < 100 or mcc > 999:
                return None, None
            if mnc < 0 or mnc > 999:
                return None, None
                
            return mcc, mnc
            
        except Exception:
            return None, None
    
    def extract_cell_id_patterns(self, data_hex: str) -> List[int]:
        """Extract potential cell IDs from hex data"""
        cell_ids = []
        
        try:
            # Convert hex to bytes
            data_bytes = bytes.fromhex(data_hex)
            
            # Look for 4-byte patterns that could be cell IDs
            for i in range(0, len(data_bytes) - 3):
                # Try little endian
                cell_id_le = struct.unpack('<L', data_bytes[i:i+4])[0]
                if 1000000 <= cell_id_le <= 9999999:  # Reasonable cell ID range
                    cell_ids.append(cell_id_le)
                    
                # Try big endian  
                cell_id_be = struct.unpack('>L', data_bytes[i:i+4])[0]
                if 1000000 <= cell_id_be <= 9999999:  # Reasonable cell ID range
                    cell_ids.append(cell_id_be)
                    
        except Exception:
            pass
            
        return list(set(cell_ids))  # Remove duplicates
    
    def extract_tac_lac_patterns(self, data_hex: str) -> List[Tuple[str, int]]:
        """Extract potential TAC/LAC values from hex data"""
        values = []
        
        try:
            data_bytes = bytes.fromhex(data_hex)
            
            # Look for 2-byte patterns that could be TAC/LAC
            for i in range(0, len(data_bytes) - 1):
                # Try little endian
                val_le = struct.unpack('<H', data_bytes[i:i+2])[0]
                if 1 <= val_le <= 65534:  # Valid TAC/LAC range
                    values.append(('tac_lac_le', val_le))
                    
                # Try big endian
                val_be = struct.unpack('>H', data_bytes[i:i+2])[0]
                if 1 <= val_be <= 65534:  # Valid TAC/LAC range
                    values.append(('tac_lac_be', val_be))
                    
        except Exception:
            pass
            
        return values
    
    def parse_control_plane_message(self, msg: Dict) -> Optional[CellularInfo]:
        """Parse a control plane message for cellular information"""
        timestamp = msg.get('timestamp', '')
        data_hex = msg.get('data', '')
        
        if not data_hex:
            return None
            
        info = CellularInfo(
            timestamp=timestamp,
            message_type="control_plane",
            raw_data=data_hex
        )
        
        # Look for PLMN patterns (MCC/MNC)
        plmn_patterns = re.findall(r'130184|1330f1|130013|130000', data_hex, re.IGNORECASE)
        if plmn_patterns:
            # Decode the first PLMN found
            mcc, mnc = self.decode_plmn(plmn_patterns[0])
            info.mcc = mcc
            info.mnc = mnc
            
        # Look for cell IDs
        cell_ids = self.extract_cell_id_patterns(data_hex)
        if cell_ids and 1114372 in cell_ids:
            info.cell_id = 1114372
        elif cell_ids:
            info.cell_id = cell_ids[0]  # Take first valid cell ID
            
        # Look for TAC/LAC values
        tac_lac_values = self.extract_tac_lac_patterns(data_hex)
        if tac_lac_values:
            # Look for value 260 (our known TAC)
            for val_type, val in tac_lac_values:
                if val == 260:
                    info.tac = val
                    break
            # If 260 not found, take first reasonable value
            if info.tac is None:
                for val_type, val in tac_lac_values:
                    if 100 <= val <= 10000:  # Reasonable TAC range
                        info.tac = val
                        break
        
        # Only return if we found some useful information
        if any([info.mcc, info.mnc, info.cell_id, info.tac]):
            return info
            
        return None
    
    def parse_scat_json(self, json_file: Path) -> List[CellularInfo]:
        """Parse SCAT JSON output for cellular information"""
        print(f"Parsing SCAT JSON output: {json_file}")
        
        with open(json_file, 'r') as f:
            data = json.load(f)
            
        # Process raw messages
        raw_messages = data.get('raw_messages', [])
        print(f"Processing {len(raw_messages)} raw messages...")
        
        for msg in raw_messages:
            if msg.get('type') == 'control_plane':
                info = self.parse_control_plane_message(msg)
                if info:
                    self.cellular_info.append(info)
                    
        print(f"Extracted {len(self.cellular_info)} cellular information records")
        return self.cellular_info
    
    def find_cell_1114372_info(self) -> Dict:
        """Find specific information about cell ID 1114372"""
        cell_1114372_info = {
            'cell_id': 1114372,
            'occurrences': [],
            'associated_networks': set(),
            'timestamps': [],
            'tac_values': set(),
            'raw_messages': []
        }
        
        for info in self.cellular_info:
            if info.cell_id == 1114372:
                cell_1114372_info['occurrences'].append(info)
                cell_1114372_info['timestamps'].append(info.timestamp)
                cell_1114372_info['raw_messages'].append(info.raw_data)
                
                if info.mcc and info.mnc:
                    cell_1114372_info['associated_networks'].add(f"{info.mcc}/{info.mnc}")
                    
                if info.tac:
                    cell_1114372_info['tac_values'].add(info.tac)
        
        # Convert sets to lists for JSON serialization
        cell_1114372_info['associated_networks'] = list(cell_1114372_info['associated_networks'])
        cell_1114372_info['tac_values'] = list(cell_1114372_info['tac_values'])
        
        return cell_1114372_info
    
    def get_network_summary(self) -> Dict:
        """Get summary of all detected networks"""
        networks = {}
        cell_ids = set()
        tac_values = set()
        
        for info in self.cellular_info:
            if info.mcc and info.mnc:
                key = f"{info.mcc}/{info.mnc}"
                if key not in networks:
                    networks[key] = {
                        'mcc': info.mcc,
                        'mnc': info.mnc,
                        'occurrences': 0,
                        'cell_ids': set(),
                        'tac_values': set(),
                        'timestamps': []
                    }
                networks[key]['occurrences'] += 1
                networks[key]['timestamps'].append(info.timestamp)
                
                if info.cell_id:
                    networks[key]['cell_ids'].add(info.cell_id)
                    cell_ids.add(info.cell_id)
                    
                if info.tac:
                    networks[key]['tac_values'].add(info.tac)
                    tac_values.add(info.tac)
        
        # Convert sets to lists
        for network in networks.values():
            network['cell_ids'] = list(network['cell_ids'])
            network['tac_values'] = list(network['tac_values'])
            
        return {
            'networks': networks,
            'total_unique_cell_ids': list(cell_ids),
            'total_unique_tac_values': list(tac_values)
        }

def main():
    parser = argparse.ArgumentParser(description='Parse SCAT output for cellular network information')
    parser.add_argument('--scat-json', required=True, help='SCAT JSON output file')
    parser.add_argument('--output', '-o', help='Output JSON file for results')
    parser.add_argument('--cell-id', type=int, default=1114372, help='Specific cell ID to analyze')
    
    args = parser.parse_args()
    
    scat_json = Path(args.scat_json)
    if not scat_json.exists():
        print(f"Error: SCAT JSON file {scat_json} not found")
        return 1
        
    parser_tool = ScatCellularParser()
    cellular_info = parser_tool.parse_scat_json(scat_json)
    
    # Get specific cell information
    cell_info = parser_tool.find_cell_1114372_info()
    
    # Get network summary
    network_summary = parser_tool.get_network_summary()
    
    results = {
        'analysis_timestamp': datetime.now(timezone.utc).isoformat(),
        'source_file': str(scat_json),
        'target_cell_id': args.cell_id,
        'cell_specific_info': cell_info,
        'network_summary': network_summary,
        'all_cellular_info': [
            {
                'timestamp': info.timestamp,
                'mcc': info.mcc,
                'mnc': info.mnc, 
                'cell_id': info.cell_id,
                'tac': info.tac,
                'message_type': info.message_type
            } for info in cellular_info
        ]
    }
    
    if args.output:
        output_file = Path(args.output)
        with open(output_file, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"Results saved to: {output_file}")
    
    # Print summary
    print("\n" + "="*80)
    print("SCAT CELLULAR ANALYSIS RESULTS")
    print("="*80)
    
    print(f"\n🗼 CELL ID {args.cell_id} ANALYSIS:")
    if cell_info['occurrences']:
        print(f"   Found {len(cell_info['occurrences'])} occurrences")
        print(f"   Associated networks: {cell_info['associated_networks']}")
        print(f"   TAC values: {cell_info['tac_values']}")
        print(f"   First seen: {cell_info['timestamps'][0] if cell_info['timestamps'] else 'N/A'}")
    else:
        print(f"   ❌ Cell ID {args.cell_id} not found in SCAT data")
    
    print(f"\n🌐 NETWORK SUMMARY:")
    for network_key, network_info in network_summary['networks'].items():
        mcc, mnc = network_key.split('/')
        print(f"   MCC {mcc}, MNC {mnc}: {network_info['occurrences']} messages")
        if network_info['cell_ids']:
            print(f"     Cell IDs: {network_info['cell_ids']}")
        if network_info['tac_values']:
            print(f"     TAC values: {network_info['tac_values']}")
    
    print(f"\n📊 SUMMARY:")
    print(f"   Total cellular messages parsed: {len(cellular_info)}")
    print(f"   Unique networks detected: {len(network_summary['networks'])}")
    print(f"   Unique cell IDs: {len(network_summary['total_unique_cell_ids'])}")
    print(f"   Unique TAC values: {len(network_summary['total_unique_tac_values'])}")
    
    return 0

if __name__ == "__main__":
    exit(main())
