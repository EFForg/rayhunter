#!/usr/bin/python3
import pycrate_mobile
from pycrate_mobile import NASLTE
import pycrate_core
import binascii
import sys
import pprint
from enum import Enum

import pycrate_mobile.TS24301_EMM

EPS_IMSI_ATTACH = 2

def parse_nas_message(buffer, uplink=None):
    if isinstance(buffer, str): #handle string argument or raw bytes
        bin = binascii.unhexlify(buffer)
    else:
        bin = buffer
    if uplink:
        parsed = NASLTE.parse_NASLTE_MO(bin)
    elif uplink == None: #We don't know if its an up or downlink
        parsed = NASLTE.parse_NASLTE_MO(bin)
        if parsed[0] == None:
            parsed = NASLTE.parse_NASLTE_MT(bin)
    else:
        parsed = NASLTE.parse_NASLTE_MT(bin)
    
    if parsed[0] is None: # Not a NAS Packet 
        raise TypeError("Not a nas packet")
    return parsed[0]

def heur_ue_imsi_sent(msg):
    output = "device transmitted IMSI to base station!"

    if type(msg) not in [pycrate_mobile.TS24301_EMM.EMMAttachRequest, pycrate_mobile.TS24301_EMM.EMMSecProtNASMessage]: 
        return (False, None)

    if isinstance(msg, pycrate_mobile.TS24301_EMM.EMMSecProtNASMessage):
        try:
            msg = msg['EMMAttachRequest']
        except pycrate_core.elt.EltErr:
            return (False, None)

    if msg['EPSAttachType']['V'].to_int() == EPS_IMSI_ATTACH: #EPSAttachType Value is 'Combined EPS/IMSI Attach (2)'
        return (True, output)
    return (False, None)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("usage: nasparse.py [hex encoded nas message]")
        exit(1)
        
    buffer = sys.argv[1]
    msg = parse_nas_message(buffer)
    pprint.pprint(msg)
    triggered, message = heur_ue_imsi_sent(msg)
    if triggered: 
        print(message)
        exit(1)