import sys

import pycrate_core.elt
from pycrate_mobile import TS24301_EMM
from scapy.layers.inet import UDP
from scapy.packet import Raw
from scapy.utils import rdpcap

from nasparse import parse_nas_message


EPS_IMSI_ATTACH = 2


def heur_ue_id_sent(msg):
    if type(msg) not in [
        TS24301_EMM.EMMAttachRequest,
        TS24301_EMM.EMMSecProtNASMessage,
        TS24301_EMM.EMMSecurityModeComplete,
    ]:
        return (False, None)

    # TODO: handle SecurityModeComplete in Security Protected messages
    if isinstance(msg, TS24301_EMM.EMMSecProtNASMessage):
        try:
            msg = msg["EMMAttachRequest"]
        except pycrate_core.elt.EltErr:
            return (False, None)

    if (
        isinstance(msg, TS24301_EMM.EMMAttachRequest)
        and msg["EPSAttachType"]["V"].to_int() == EPS_IMSI_ATTACH
    ):  # EPSAttachType Value is 'Combined EPS/IMSI Attach (2)'
        try:
            t, i = msg["EPSID"]["EPSID"].decode()
            if i == str(0xA):
                return (False, "This was previously redacted.")
            else:
                return (True, (t, i))
        except pycrate_core.elt.EltErr:
            return (False, None)
    elif isinstance(msg, TS24301_EMM.EMMSecurityModeComplete):
        try:
            t, i = msg["IMEISV"]["ID"].decode()
            if i == str(0xA):
                return (False, "This was previously redacted.")
            else:
                return (True, (t, i))
        except pycrate_core.elt.EltErr:
            return (False, None)
    return (False, None)


def main(packets):
    idents = []
    for gsmtap in packets:
        try:
            # gsmtap header is always 16 bytes
            # if this isn't true, the second byte is the length in 32-bit words
            msg = parse_nas_message(gsmtap[UDP][Raw].load[16:].hex())
            triggered, message = heur_ue_id_sent(msg)
            if triggered:
                print(message, gsmtap.load.hex())
                idents.extend([message])
        except TypeError:
            pass
    return idents


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("usage: extract.py PCAP")
        exit(1)

    packets = rdpcap(sys.argv[1])
    print(main(packets))
