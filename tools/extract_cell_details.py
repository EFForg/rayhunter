#!/usr/bin/env python3
"""
Cell Details Extractor

Extract detailed cellular network information for specific cell IDs from QMDL files.
Focus on MCC, MNC, LAC, TAC and other network identifiers.
"""

import struct
import argparse
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List, Optional, Tuple

class CellDetailsExtractor:
    def __init__(self):
        self.cell_details = {}
        
    def extract_lte_rrc_info(self, data: bytes, offset: int) -> Optional[Dict]:
        """Extract LTE RRC information including PLMN (MCC/MNC)"""
        try:
            # Look for System Information Block (SIB) patterns
            # SIB1 contains PLMN information
            if b'\x00\x01' in data[offset:offset+20]:  # SIB1 indicator
                # Extract PLMN information
                plmn_offset = offset + 8
                if plmn_offset + 6 <= len(data):
                    # PLMN is encoded in 3 bytes: MCC digit 2 | MCC digit 1 | MNC digit 3 | MCC digit 3 | MNC digit 2 | MNC digit 1
                    plmn_bytes = data[plmn_offset:plmn_offset+3]
                    
                    # Decode MCC/MNC from PLMN
                    mcc_digit_1 = plmn_bytes[0] & 0x0F
                    mcc_digit_2 = (plmn_bytes[0] & 0xF0) >> 4
                    mcc_digit_3 = (plmn_bytes[1] & 0xF0) >> 4
                    
                    mnc_digit_1 = plmn_bytes[2] & 0x0F
                    mnc_digit_2 = (plmn_bytes[2] & 0xF0) >> 4
                    mnc_digit_3 = plmn_bytes[1] & 0x0F
                    
                    # Handle 2-digit vs 3-digit MNC
                    if mnc_digit_3 == 0xF:
                        mnc = mnc_digit_1 * 10 + mnc_digit_2
                    else:
                        mnc = mnc_digit_1 * 100 + mnc_digit_2 * 10 + mnc_digit_3
                        
                    mcc = mcc_digit_1 * 100 + mcc_digit_2 * 10 + mcc_digit_3
                    
                    return {
                        'mcc': mcc,
                        'mnc': mnc,
                        'plmn_bytes': plmn_bytes.hex()
                    }
        except:
            pass
        return None
        
    def extract_gsm_info(self, data: bytes, offset: int) -> Optional[Dict]:
        """Extract GSM LAI (Location Area Identity) information"""
        try:
            # GSM LAI structure: PLMN (3 bytes) + LAC (2 bytes)
            if offset + 5 <= len(data):
                plmn_bytes = data[offset:offset+3]
                lac_bytes = data[offset+3:offset+5]
                
                # Decode PLMN
                mcc_digit_1 = plmn_bytes[0] & 0x0F
                mcc_digit_2 = (plmn_bytes[0] & 0xF0) >> 4
                mcc_digit_3 = (plmn_bytes[1] & 0xF0) >> 4
                
                mnc_digit_1 = plmn_bytes[2] & 0x0F
                mnc_digit_2 = (plmn_bytes[2] & 0xF0) >> 4
                mnc_digit_3 = plmn_bytes[1] & 0x0F
                
                # Handle 2-digit vs 3-digit MNC
                if mnc_digit_3 == 0xF:
                    mnc = mnc_digit_1 * 10 + mnc_digit_2
                else:
                    mnc = mnc_digit_1 * 100 + mnc_digit_2 * 10 + mnc_digit_3
                    
                mcc = mcc_digit_1 * 100 + mcc_digit_2 * 10 + mcc_digit_3
                
                # Decode LAC
                lac = struct.unpack('>H', lac_bytes)[0]  # Big endian
                
                return {
                    'mcc': mcc,
                    'mnc': mnc,
                    'lac': lac,
                    'plmn_bytes': plmn_bytes.hex(),
                    'lac_bytes': lac_bytes.hex()
                }
        except:
            pass
        return None
        
    def search_cell_id_patterns(self, data: bytes, target_cell_id: int) -> List[Dict]:
        """Search for patterns around the target cell ID"""
        results = []
        cell_id_bytes = struct.pack('<L', target_cell_id)  # Little endian
        cell_id_bytes_be = struct.pack('>L', target_cell_id)  # Big endian
        
        # Search for little endian cell ID
        offset = 0
        while True:
            offset = data.find(cell_id_bytes, offset)
            if offset == -1:
                break
                
            # Extract context around cell ID
            context_start = max(0, offset - 50)
            context_end = min(len(data), offset + 50)
            context = data[context_start:context_end]
            
            # Try to extract network information
            lte_info = self.extract_lte_rrc_info(data, offset - 20)
            gsm_info = self.extract_gsm_info(data, offset - 10)
            
            result = {
                'offset': offset,
                'encoding': 'little_endian',
                'context_hex': context.hex(),
                'lte_info': lte_info,
                'gsm_info': gsm_info
            }
            results.append(result)
            offset += 4
            
        # Search for big endian cell ID
        offset = 0
        while True:
            offset = data.find(cell_id_bytes_be, offset)
            if offset == -1:
                break
                
            # Extract context around cell ID
            context_start = max(0, offset - 50)
            context_end = min(len(data), offset + 50)
            context = data[context_start:context_end]
            
            # Try to extract network information
            lte_info = self.extract_lte_rrc_info(data, offset - 20)
            gsm_info = self.extract_gsm_info(data, offset - 10)
            
            result = {
                'offset': offset,
                'encoding': 'big_endian',
                'context_hex': context.hex(),
                'lte_info': lte_info,
                'gsm_info': gsm_info
            }
            results.append(result)
            offset += 4
            
        return results
        
    def extract_common_network_patterns(self, data: bytes) -> Dict:
        """Extract common cellular network patterns that might indicate MCC/MNC"""
        patterns = {
            'potential_plmns': [],
            'potential_lacs': [],
            'potential_tacs': []
        }
        
        # Common US MCC values: 310, 311, 312, 313, 314, 315, 316
        us_mccs = [310, 311, 312, 313, 314, 315, 316]
        
        # Search for PLMN patterns
        for offset in range(0, len(data) - 3):
            try:
                plmn_bytes = data[offset:offset+3]
                
                # Decode potential MCC/MNC
                mcc_digit_1 = plmn_bytes[0] & 0x0F
                mcc_digit_2 = (plmn_bytes[0] & 0xF0) >> 4
                mcc_digit_3 = (plmn_bytes[1] & 0xF0) >> 4
                
                mnc_digit_1 = plmn_bytes[2] & 0x0F
                mnc_digit_2 = (plmn_bytes[2] & 0xF0) >> 4
                mnc_digit_3 = plmn_bytes[1] & 0x0F
                
                mcc = mcc_digit_1 * 100 + mcc_digit_2 * 10 + mcc_digit_3
                
                # Check if this looks like a valid US MCC
                if mcc in us_mccs and all(d <= 9 for d in [mcc_digit_1, mcc_digit_2, mcc_digit_3]):
                    # Handle 2-digit vs 3-digit MNC
                    if mnc_digit_3 == 0xF:
                        mnc = mnc_digit_1 * 10 + mnc_digit_2
                    else:
                        mnc = mnc_digit_1 * 100 + mnc_digit_2 * 10 + mnc_digit_3
                        
                    if mnc <= 999:  # Valid MNC range
                        patterns['potential_plmns'].append({
                            'offset': offset,
                            'mcc': mcc,
                            'mnc': mnc,
                            'plmn_hex': plmn_bytes.hex()
                        })
            except:
                continue
                
        # Remove duplicates
        seen_plmns = set()
        unique_plmns = []
        for plmn in patterns['potential_plmns']:
            key = (plmn['mcc'], plmn['mnc'])
            if key not in seen_plmns:
                seen_plmns.add(key)
                unique_plmns.append(plmn)
        patterns['potential_plmns'] = unique_plmns
        
        return patterns
        
    def analyze_qmdl(self, qmdl_file: Path, target_cell_id: int = None) -> Dict:
        """Analyze QMDL file for cellular network details"""
        print(f"Analyzing QMDL file: {qmdl_file}")
        
        with open(qmdl_file, 'rb') as f:
            data = f.read()
            
        print(f"File size: {len(data)} bytes")
        
        results = {
            'file_info': {
                'path': str(qmdl_file),
                'size': len(data),
                'analysis_time': datetime.now(timezone.utc).isoformat()
            },
            'cell_specific_results': {},
            'general_patterns': {}
        }
        
        # If target cell ID specified, search for it specifically
        if target_cell_id:
            print(f"Searching for Cell ID: {target_cell_id}")
            cell_patterns = self.search_cell_id_patterns(data, target_cell_id)
            results['cell_specific_results'][target_cell_id] = cell_patterns
            print(f"Found {len(cell_patterns)} occurrences of Cell ID {target_cell_id}")
        
        # Extract general network patterns
        print("Extracting general network patterns...")
        general_patterns = self.extract_common_network_patterns(data)
        results['general_patterns'] = general_patterns
        print(f"Found {len(general_patterns['potential_plmns'])} potential PLMN identifiers")
        
        return results

def main():
    parser = argparse.ArgumentParser(description='Extract detailed cellular network information from QMDL files')
    parser.add_argument('--qmdl', required=True, help='QMDL file to analyze')
    parser.add_argument('--cell-id', type=int, help='Specific cell ID to search for')
    parser.add_argument('--output', '-o', help='Output JSON file')
    
    args = parser.parse_args()
    
    qmdl_file = Path(args.qmdl)
    if not qmdl_file.exists():
        print(f"Error: QMDL file {qmdl_file} not found")
        return 1
        
    extractor = CellDetailsExtractor()
    results = extractor.analyze_qmdl(qmdl_file, args.cell_id)
    
    if args.output:
        output_file = Path(args.output)
        with open(output_file, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"Results saved to: {output_file}")
    else:
        print("\n" + "="*60)
        print("CELLULAR NETWORK ANALYSIS RESULTS")
        print("="*60)
        
        if args.cell_id and args.cell_id in results['cell_specific_results']:
            cell_results = results['cell_specific_results'][args.cell_id]
            print(f"\n🗼 CELL ID {args.cell_id} ANALYSIS:")
            print(f"   Found {len(cell_results)} occurrences in QMDL file")
            
            for i, result in enumerate(cell_results[:5]):  # Show first 5
                print(f"\n   Occurrence {i+1}:")
                print(f"   - Offset: 0x{result['offset']:08x}")
                print(f"   - Encoding: {result['encoding']}")
                
                if result['lte_info']:
                    info = result['lte_info']
                    print(f"   - LTE Info: MCC={info['mcc']}, MNC={info['mnc']}")
                    
                if result['gsm_info']:
                    info = result['gsm_info']
                    print(f"   - GSM Info: MCC={info['mcc']}, MNC={info['mnc']}, LAC={info['lac']}")
        
        patterns = results['general_patterns']
        if patterns['potential_plmns']:
            print(f"\n🌐 NETWORK IDENTIFIERS FOUND:")
            seen = set()
            for plmn in patterns['potential_plmns'][:10]:  # Show first 10 unique
                key = (plmn['mcc'], plmn['mnc'])
                if key not in seen:
                    seen.add(key)
                    print(f"   - MCC: {plmn['mcc']}, MNC: {plmn['mnc']}")
        
        print(f"\n📄 Full results available in JSON format using --output option")
        
    return 0

if __name__ == "__main__":
    exit(main())
