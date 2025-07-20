#!/usr/bin/env python3
"""
2G Downgrade Analyzer

Specialized tool to analyze QMDL files for 2G downgrade events and identify
the cell towers that performed the downgrade attack.
"""

import struct
import json
import sys
import argparse
from pathlib import Path
from typing import List, Dict, Optional, Any, Tuple
from dataclasses import dataclass
from datetime import datetime, timezone
import binascii

@dataclass
class DowngradeEvent:
    timestamp: int
    datetime_str: str
    event_type: str  # "connection_release", "sib_downgrade", "redirect_2g"
    source_cell_id: Optional[int] = None
    source_pci: Optional[int] = None
    source_tac: Optional[int] = None
    source_mcc: Optional[int] = None
    source_mnc: Optional[int] = None
    source_rat: Optional[str] = None  # LTE/UMTS before downgrade
    target_rat: str = "GSM"  # Usually GSM/2G
    target_arfcn: Optional[int] = None
    target_lac: Optional[int] = None
    downgrade_reason: Optional[str] = None
    signal_strength: Optional[float] = None
    raw_data: Optional[str] = None

class DowngradeAnalyzer:
    def __init__(self):
        self.downgrade_events: List[DowngradeEvent] = []
        
        # Known message types that indicate downgrade events
        self.downgrade_message_types = {
            # LTE RRC Connection Release with redirection
            0x1001: "LTE_RRC_CONNECTION_RELEASE",
            0x1002: "LTE_RRC_CONNECTION_RELEASE_REDIRECT",
            
            # System Information Block messages
            0x2001: "LTE_SIB_TYPE_6",  # UTRAN neighbor frequencies
            0x2002: "LTE_SIB_TYPE_7",  # GERAN neighbor frequencies
            
            # NAS EMM messages
            0x3001: "NAS_EMM_ATTACH_REJECT",
            0x3002: "NAS_EMM_TAU_REJECT",
            0x3003: "NAS_EMM_DETACH_REQUEST",
            
            # Cell selection/reselection
            0x4001: "LTE_CELL_RESELECTION",
            0x4002: "LTE_INTER_RAT_RESELECTION",
            
            # Measurement reports indicating weak LTE signal
            0x5001: "LTE_MEASUREMENT_REPORT_INTER_RAT",
            0x5002: "LTE_NEIGHBOR_MEAS_GERAN",
        }
        
        # 2G frequency bands (ARFCN ranges)
        self.gsm_bands = {
            "GSM900": (0, 124),      # 890-915 MHz uplink
            "DCS1800": (512, 885),   # 1710-1785 MHz uplink
            "PCS1900": (512, 810),   # 1850-1910 MHz uplink
            "GSM850": (128, 251),    # 824-849 MHz uplink
        }
        
    def parse_timestamp(self, data: bytes, offset: int) -> int:
        """Parse QMDL timestamp"""
        try:
            if offset + 8 > len(data):
                return 0
                
            timestamp_bytes = data[offset:offset+8]
            timestamp_low = struct.unpack('<L', timestamp_bytes[0:4])[0]
            timestamp_high = struct.unpack('<L', timestamp_bytes[4:8])[0]
            
            full_timestamp = (timestamp_high << 32) | timestamp_low
            
            # Multiple timestamp conversion attempts
            conversions = [
                int(full_timestamp / 1000000) + 946684800,  # GPS epoch adjust
                int(full_timestamp / 1000) + 1262304000,    # Alternative epoch
                int(full_timestamp / 19200) + 946684800,    # CDMA time base
            ]
            
            # Return first reasonable timestamp (2020-2030 range)
            for ts in conversions:
                if 1577836800 <= ts <= 1893456000:
                    return ts
                    
            return 0
            
        except (struct.error, ValueError):
            return 0
            
    def detect_rrc_connection_release(self, data: bytes, offset: int) -> Optional[DowngradeEvent]:
        """Detect RRC Connection Release with redirection to 2G"""
        try:
            if offset + 32 > len(data):
                return None
                
            timestamp = self.parse_timestamp(data, offset)
            if timestamp == 0:
                return None
                
            # Look for RRC Connection Release message patterns
            # This is simplified - real parsing would decode ASN.1
            
            # Check for redirection information element
            redirect_found = False
            target_rat = None
            target_freq = None
            
            # Scan for typical 2G redirection patterns
            for i in range(offset, min(offset + 100, len(data) - 4)):
                try:
                    # Look for GERAN frequency information
                    word = struct.unpack('<L', data[i:i+4])[0]
                    
                    # ARFCN values typically in certain ranges
                    if 0 <= (word & 0x3FF) <= 1023:  # ARFCN range
                        arfcn = word & 0x3FF
                        if self.is_gsm_frequency(arfcn):
                            redirect_found = True
                            target_rat = "GSM"
                            target_freq = arfcn
                            break
                            
                except struct.error:
                    continue
                    
            if redirect_found:
                return DowngradeEvent(
                    timestamp=timestamp,
                    datetime_str=datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat(),
                    event_type="connection_release_redirect",
                    target_rat=target_rat,
                    target_arfcn=target_freq,
                    downgrade_reason="RRC Connection Release with 2G redirect",
                    raw_data=binascii.hexlify(data[offset:offset+32]).decode()
                )
                
        except Exception:
            pass
            
        return None
        
    def detect_sib_downgrade(self, data: bytes, offset: int) -> Optional[DowngradeEvent]:
        """Detect SIB 6/7 messages promoting 2G frequencies"""
        try:
            timestamp = self.parse_timestamp(data, offset)
            if timestamp == 0:
                return None
                
            # Look for System Information Block Type 6 or 7
            # These contain neighbor frequency information
            
            high_priority_2g = False
            target_arfcn = None
            
            # Scan for frequency priority information
            for i in range(offset, min(offset + 200, len(data) - 2)):
                try:
                    # Look for priority values and ARFCN
                    word = struct.unpack('<H', data[i:i+2])[0]
                    
                    # Check for GERAN frequency with high priority
                    if 0 <= word <= 1023 and self.is_gsm_frequency(word):
                        # Check if next bytes indicate high priority
                        if i + 4 < len(data):
                            priority_bytes = data[i+2:i+4]
                            # High priority typically > current LTE priority
                            if len(priority_bytes) >= 2 and priority_bytes[0] > 3:
                                high_priority_2g = True
                                target_arfcn = word
                                break
                                
                except struct.error:
                    continue
                    
            if high_priority_2g:
                return DowngradeEvent(
                    timestamp=timestamp,
                    datetime_str=datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat(),
                    event_type="sib_downgrade",
                    target_rat="GSM",
                    target_arfcn=target_arfcn,
                    downgrade_reason="SIB broadcast with high-priority 2G frequencies",
                    raw_data=binascii.hexlify(data[offset:offset+32]).decode()
                )
                
        except Exception:
            pass
            
        return None
        
    def detect_cell_info(self, data: bytes, offset: int) -> Dict[str, Any]:
        """Extract cell information from message"""
        cell_info = {}
        
        try:
            # Look for cell identity patterns
            for i in range(offset, min(offset + 50, len(data) - 4)):
                try:
                    dword = struct.unpack('<L', data[i:i+4])[0]
                    
                    # Cell ID typically in certain ranges
                    if 0x1000 <= (dword & 0xFFFFFF) <= 0xFFFFFE:
                        cell_info['cell_id'] = dword & 0xFFFFFF
                        
                    # PCI range 0-503
                    if 0 <= (dword & 0x1FF) <= 503:
                        cell_info['pci'] = dword & 0x1FF
                        
                    # TAC range
                    if 0x0001 <= (dword & 0xFFFF) <= 0xFFFE:
                        cell_info['tac'] = dword & 0xFFFF
                        
                except struct.error:
                    continue
                    
        except Exception:
            pass
            
        return cell_info
        
    def is_gsm_frequency(self, arfcn: int) -> bool:
        """Check if ARFCN is in GSM frequency bands"""
        for band, (start, end) in self.gsm_bands.items():
            if start <= arfcn <= end:
                return True
        return False
        
    def get_gsm_band(self, arfcn: int) -> Optional[str]:
        """Get GSM band name for ARFCN"""
        for band, (start, end) in self.gsm_bands.items():
            if start <= arfcn <= end:
                return band
        return None
        
    def analyze_qmdl_for_downgrades(self, qmdl_file: Path) -> None:
        """Analyze QMDL file for 2G downgrade events"""
        print(f"Analyzing QMDL file for 2G downgrade events: {qmdl_file}")
        
        with open(qmdl_file, 'rb') as f:
            data = f.read()
            
        print(f"QMDL file size: {len(data)} bytes")
        
        offset = 0
        events_found = 0
        messages_processed = 0
        
        while offset < len(data) - 16:
            try:
                # Look for QMDL message frames
                if offset + 6 <= len(data):
                    # Check for various frame starts
                    frame_patterns = [b'\x7E\x00', b'\x7E\x01', b'\x10\x00']
                    
                    frame_found = False
                    for pattern in frame_patterns:
                        if data[offset:offset+len(pattern)] == pattern:
                            frame_found = True
                            break
                            
                    if frame_found and offset + 16 <= len(data):
                        messages_processed += 1
                        
                        # Try to detect different types of downgrade events
                        events = []
                        
                        # Check for RRC Connection Release
                        rrc_event = self.detect_rrc_connection_release(data, offset + 6)
                        if rrc_event:
                            # Add cell information
                            cell_info = self.detect_cell_info(data, offset + 6)
                            rrc_event.source_cell_id = cell_info.get('cell_id')
                            rrc_event.source_pci = cell_info.get('pci')
                            rrc_event.source_tac = cell_info.get('tac')
                            events.append(rrc_event)
                            
                        # Check for SIB downgrade
                        sib_event = self.detect_sib_downgrade(data, offset + 6)
                        if sib_event:
                            cell_info = self.detect_cell_info(data, offset + 6)
                            sib_event.source_cell_id = cell_info.get('cell_id')
                            sib_event.source_pci = cell_info.get('pci')
                            sib_event.source_tac = cell_info.get('tac')
                            events.append(sib_event)
                            
                        for event in events:
                            self.downgrade_events.append(event)
                            events_found += 1
                            
                        offset += max(8, 16)  # Skip message
                    else:
                        offset += 1
                else:
                    offset += 1
                    
                # Progress indicator
                if messages_processed > 0 and messages_processed % 1000 == 0:
                    progress = (offset / len(data)) * 100
                    print(f"Progress: {progress:.1f}% - Messages processed: {messages_processed}, Events found: {events_found}")
                    
            except Exception as e:
                offset += 1
                continue
                
        print(f"Analysis complete!")
        print(f"Messages processed: {messages_processed}")
        print(f"Downgrade events found: {events_found}")
        
    def generate_downgrade_report(self, output_file: Path) -> None:
        """Generate comprehensive downgrade analysis report"""
        print(f"Generating downgrade analysis report: {output_file}")
        
        if not self.downgrade_events:
            report = {
                "analysis_summary": {
                    "timestamp": datetime.now(timezone.utc).isoformat(),
                    "downgrade_events_found": 0,
                    "conclusion": "No 2G downgrade events detected in QMDL file"
                },
                "events": []
            }
        else:
            # Group events by type
            event_types = {}
            for event in self.downgrade_events:
                event_type = event.event_type
                if event_type not in event_types:
                    event_types[event_type] = []
                event_types[event_type].append(event)
                
            # Identify likely attack cells
            attacking_cells = {}
            for event in self.downgrade_events:
                if event.source_cell_id:
                    cell_key = f"Cell_{event.source_cell_id}"
                    if cell_key not in attacking_cells:
                        attacking_cells[cell_key] = {
                            "cell_id": event.source_cell_id,
                            "pci": event.source_pci,
                            "tac": event.source_tac,
                            "downgrade_attempts": 0,
                            "event_types": []
                        }
                    attacking_cells[cell_key]["downgrade_attempts"] += 1
                    if event.event_type not in attacking_cells[cell_key]["event_types"]:
                        attacking_cells[cell_key]["event_types"].append(event.event_type)
                        
            report = {
                "analysis_summary": {
                    "timestamp": datetime.now(timezone.utc).isoformat(),
                    "downgrade_events_found": len(self.downgrade_events),
                    "event_types_detected": list(event_types.keys()),
                    "attacking_cells_identified": len(attacking_cells),
                    "conclusion": "2G downgrade attack detected!" if self.downgrade_events else "No attacks detected"
                },
                "attacking_cells": attacking_cells,
                "detailed_events": [
                    {
                        "timestamp": event.timestamp,
                        "datetime": event.datetime_str,
                        "event_type": event.event_type,
                        "source_cell_id": event.source_cell_id,
                        "source_pci": event.source_pci,
                        "source_tac": event.source_tac,
                        "target_technology": event.target_rat,
                        "target_frequency": event.target_arfcn,
                        "gsm_band": self.get_gsm_band(event.target_arfcn) if event.target_arfcn else None,
                        "downgrade_reason": event.downgrade_reason,
                        "raw_data": event.raw_data
                    }
                    for event in self.downgrade_events
                ]
            }
            
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
            
        # Print summary
        print("\n" + "="*60)
        print("2G DOWNGRADE ANALYSIS RESULTS")
        print("="*60)
        
        if self.downgrade_events:
            print(f"🚨 ATTACK DETECTED! {len(self.downgrade_events)} downgrade events found")
            
            # Show attacking cells
            if report["attacking_cells"]:
                print(f"\n🗼 ATTACKING CELL TOWERS:")
                for cell_key, cell_data in report["attacking_cells"].items():
                    print(f"   Cell ID: {cell_data['cell_id']}")
                    if cell_data['pci']:
                        print(f"   PCI: {cell_data['pci']}")
                    if cell_data['tac']:
                        print(f"   TAC: {cell_data['tac']}")
                    print(f"   Downgrade attempts: {cell_data['downgrade_attempts']}")
                    print(f"   Attack types: {', '.join(cell_data['event_types'])}")
                    print()
                    
            # Show event types
            event_counts = {}
            for event in self.downgrade_events:
                event_counts[event.event_type] = event_counts.get(event.event_type, 0) + 1
                
            print(f"📊 ATTACK BREAKDOWN:")
            for event_type, count in event_counts.items():
                print(f"   {event_type}: {count} events")
                
        else:
            print("✅ No 2G downgrade attacks detected")
            
        print(f"\n📄 Full report saved to: {output_file}")

def main():
    parser = argparse.ArgumentParser(description='Analyze QMDL file for 2G downgrade attacks')
    parser.add_argument('--qmdl', required=True, help='QMDL file to analyze')
    parser.add_argument('--output', '-o', default='downgrade_analysis.json', help='Output analysis file')
    
    args = parser.parse_args()
    
    qmdl_file = Path(args.qmdl)
    if not qmdl_file.exists():
        print(f"Error: QMDL file {qmdl_file} not found")
        sys.exit(1)
        
    analyzer = DowngradeAnalyzer()
    analyzer.analyze_qmdl_for_downgrades(qmdl_file)
    
    output_file = Path(args.output)
    analyzer.generate_downgrade_report(output_file)

if __name__ == "__main__":
    main()
