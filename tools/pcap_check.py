#!/usr/bin/python3
import nasparse
from scapy.utils import RawPcapNgReader
import sys

TYPE_LTE_NAS = 0x12
UDP_LEN = 28

def process_pcap(pcap_path):
    print('Opening {}...'.format(pcap_path))

    count = 0
    for pkt_data, pkt_metadata in RawPcapNgReader(pcap_path):
        count += 1
        gsmtap_len = pkt_data[UDP_LEN+1]  * 4 # gsmtap header length is stored in the 2nd byte of GSMTAP as a number of 32 bit words
        header_end = gsmtap_len + UDP_LEN #length of UDP/IP header plus GSMTAP header
        
        gsmtap_hdr = pkt_data[UDP_LEN:header_end]
        
        if gsmtap_hdr[2] != TYPE_LTE_NAS:
            continue

        # uplink status is the 7th bit of the 5th byte of the GSMTAP header. 
        # Uplink (Mobile originated) = 0 Downlink (mobile terminated) = 1
        uplink = (gsmtap_hdr[4] & 0b01000000) >> 6
        buffer = pkt_data[header_end:]
        msg = nasparse.parse_nas_message(buffer, uplink)
        triggered, message = nasparse.heur_ue_imsi_sent(msg)
        if triggered:
            print(f"Frame {count} triggered heuristic: {message}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("usage: pcap_check.py [path/to/pcap/file]")
        exit(1)
        
    pcap_path = sys.argv[1]
    process_pcap(pcap_path)