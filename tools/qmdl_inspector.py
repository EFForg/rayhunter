#!/usr/bin/env python3
"""
QMDL Log Code Inspector

This tool analyzes QMDL files to identify all log codes present
and provides detailed information about message structure.
"""

import sys
import struct
from collections import Counter, defaultdict
from pathlib import Path

class QMDLInspector:
    def __init__(self, file_path):
        self.file_path = Path(file_path)
        self.log_codes = Counter()
        self.message_lengths = []
        self.total_messages = 0
        self.raw_messages = []
        
    def parse_message(self, data):
        """Parse a potential diagnostic message"""
        if len(data) < 4:
            return None
            
        try:
            # Try different interpretations of the message header
            interpretations = []
            
            # Standard diag header interpretation
            if len(data) >= 12:
                length = struct.unpack('<H', data[0:2])[0]
                log_code = struct.unpack('<H', data[2:4])[0]
                timestamp_low = struct.unpack('<L', data[4:8])[0]
                timestamp_high = struct.unpack('<L', data[8:12])[0]
                
                interpretations.append({
                    'type': 'standard_diag',
                    'length': length,
                    'log_code': log_code,
                    'timestamp': (timestamp_high << 32) | timestamp_low,
                    'valid': length <= len(data) and log_code < 0x10000
                })
            
            # Alternative interpretation - log code at start
            if len(data) >= 8:
                log_code = struct.unpack('<H', data[0:2])[0]
                length = struct.unpack('<H', data[2:4])[0]
                timestamp = struct.unpack('<L', data[4:8])[0]
                
                interpretations.append({
                    'type': 'alt_diag',
                    'length': length,
                    'log_code': log_code,
                    'timestamp': timestamp,
                    'valid': length <= len(data) and log_code < 0x10000
                })
            
            # Simple log code interpretation
            if len(data) >= 4:
                log_code = struct.unpack('<H', data[0:2])[0]
                seq_or_len = struct.unpack('<H', data[2:4])[0]
                
                interpretations.append({
                    'type': 'simple',
                    'log_code': log_code,
                    'seq_or_len': seq_or_len,
                    'valid': log_code < 0x10000
                })
            
            # Return the most likely valid interpretation
            valid_interp = [i for i in interpretations if i.get('valid', False)]
            if valid_interp:
                return valid_interp[0]
            elif interpretations:
                return interpretations[0]
                
        except struct.error:
            pass
            
        return None
    
    def inspect_file(self):
        """Inspect the QMDL file structure"""
        print(f"Inspecting QMDL file: {self.file_path}")
        
        if not self.file_path.exists():
            raise FileNotFoundError(f"File {self.file_path} does not exist")
            
        file_size = self.file_path.stat().st_size
        print(f"File size: {file_size} bytes")
        
        with open(self.file_path, 'rb') as f:
            data = f.read()
            
        print(f"First 32 bytes (hex): {data[:32].hex()}")
        print(f"Last 32 bytes (hex): {data[-32:].hex()}")
        
        # Look for frame patterns
        frame_delimiters = []
        for i, byte in enumerate(data):
            if byte == 0x7E:
                frame_delimiters.append(i)
                
        print(f"Found {len(frame_delimiters)} potential frame delimiters (0x7E)")
        
        if len(frame_delimiters) >= 2:
            print(f"First few frame positions: {frame_delimiters[:10]}")
            
        # Parse frames
        offset = 0
        frame_count = 0
        
        while offset < len(data) and frame_count < 50:  # Limit for analysis
            # Look for next frame delimiter (0x7E)
            frame_start = -1
            for i in range(offset, len(data)):
                if data[i] == 0x7E:
                    frame_start = i
                    break
                    
            if frame_start == -1:
                break
                
            # Look for frame end
            frame_end = -1
            for i in range(frame_start + 1, min(frame_start + 2000, len(data))):
                if data[i] == 0x7E:
                    frame_end = i
                    break
                    
            if frame_end == -1:
                frame_data = data[frame_start+1:]
                offset = len(data)
            else:
                frame_data = data[frame_start+1:frame_end]
                offset = frame_end + 1
                
            if len(frame_data) < 4:
                continue
                
            frame_count += 1
            self.total_messages += 1
            self.message_lengths.append(len(frame_data))
            
            # Store raw message for detailed analysis
            if len(self.raw_messages) < 10:
                self.raw_messages.append({
                    'frame_num': frame_count,
                    'length': len(frame_data),
                    'data': frame_data[:min(64, len(frame_data))],  # First 64 bytes
                    'hex': frame_data[:min(64, len(frame_data))].hex()
                })
            
            # Try to parse the message
            parsed = self.parse_message(frame_data)
            if parsed and 'log_code' in parsed:
                self.log_codes[parsed['log_code']] += 1
                
        print(f"Processed {self.total_messages} messages")
        
    def generate_report(self):
        """Generate detailed inspection report"""
        print(f"\n=== QMDL INSPECTION REPORT ===")
        print(f"Total messages: {self.total_messages}")
        print(f"Unique log codes found: {len(self.log_codes)}")
        
        if self.message_lengths:
            print(f"Message length stats:")
            print(f"  Min: {min(self.message_lengths)} bytes")
            print(f"  Max: {max(self.message_lengths)} bytes")
            print(f"  Avg: {sum(self.message_lengths)/len(self.message_lengths):.1f} bytes")
        
        print(f"\n=== TOP LOG CODES ===")
        for log_code, count in self.log_codes.most_common(20):
            print(f"0x{log_code:04x}: {count} messages")
            
        print(f"\n=== RAW MESSAGE SAMPLES ===")
        for msg in self.raw_messages:
            print(f"Frame {msg['frame_num']}: Length={msg['length']}")
            print(f"  Hex: {msg['hex']}")
            
            # Try to interpret as different message types
            data = msg['data']
            if len(data) >= 4:
                try:
                    # Standard interpretation
                    val1 = struct.unpack('<H', data[0:2])[0]
                    val2 = struct.unpack('<H', data[2:4])[0]
                    print(f"  As uint16s: {val1:04x} {val2:04x}")
                    
                    if len(data) >= 8:
                        val3 = struct.unpack('<L', data[4:8])[0]
                        print(f"  Next uint32: {val3:08x}")
                except:
                    pass
            print()
            
        # Check against known cellular log codes
        known_cellular_codes = {
            # LTE codes
            0xb0c0: "LOG_LTE_RRC_OTA_MSG_LOG_C",
            0xb0e0: "LOG_LTE_ML1_SERVING_CELL_MEAS_AND_EVAL",
            0xb0e1: "LOG_LTE_ML1_NEIGHBOR_MEASUREMENTS", 
            0xb0e2: "LOG_LTE_NAS_ESM_OTA_IN_MSG_LOG_C",
            0xb0e3: "LOG_LTE_NAS_ESM_OTA_OUT_MSG_LOG_C",
            0xb0e4: "LOG_LTE_ML1_SERVING_CELL_INFO",
            0xb0ec: "LOG_LTE_NAS_EMM_OTA_IN_MSG_LOG_C",
            0xb0ed: "LOG_LTE_NAS_EMM_OTA_OUT_MSG_LOG_C",
            # GSM codes
            0x512f: "LOG_GSM_RR_SIGNALING_MESSAGE_C",
            0x5134: "LOG_GSM_L1_BURST_METRICS",
            0x513a: "LOG_GSM_L1_CELL_ID",
            # WCDMA codes
            0x412f: "WCDMA_SIGNALLING_MESSAGE",
            0x4127: "LOG_WCDMA_CELL_ID",
            # General
            0x713a: "LOG_UMTS_NAS_OTA_MESSAGE_LOG_PACKET_C",
        }
        
        print(f"=== KNOWN CELLULAR LOG CODES FOUND ===")
        found_cellular = False
        for code, name in known_cellular_codes.items():
            if code in self.log_codes:
                print(f"0x{code:04x} ({name}): {self.log_codes[code]} messages")
                found_cellular = True
                
        if not found_cellular:
            print("No known cellular log codes found in this file.")
            print("This might indicate:")
            print("1. The device was not capturing cellular logs")
            print("2. Different log code format is used")
            print("3. Messages need different parsing approach")
            
    def save_analysis(self):
        """Save detailed analysis to file"""
        analysis_file = self.file_path.parent / f"{self.file_path.stem}_analysis.txt"
        
        with open(analysis_file, 'w') as f:
            f.write(f"QMDL Analysis Report\n")
            f.write(f"File: {self.file_path}\n")
            f.write(f"Generated: {datetime.now().isoformat()}\n\n")
            
            f.write(f"Total messages: {self.total_messages}\n")
            f.write(f"Unique log codes: {len(self.log_codes)}\n\n")
            
            f.write("All log codes found:\n")
            for log_code, count in sorted(self.log_codes.items()):
                f.write(f"0x{log_code:04x}: {count}\n")
                
        print(f"\nDetailed analysis saved to: {analysis_file}")

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 qmdl_inspector.py <qmdl_file>")
        sys.exit(1)
        
    qmdl_file = sys.argv[1]
    inspector = QMDLInspector(qmdl_file)
    
    try:
        inspector.inspect_file()
        inspector.generate_report()
        inspector.save_analysis()
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == "__main__":
    from datetime import datetime
    main()
