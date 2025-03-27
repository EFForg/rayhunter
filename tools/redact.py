import sys

import yara_x as yara
from scapy.utils import rdpcap
from scapy.error import Scapy_Exception
from pycrate_mobile.TS24008_IE import encode_bcd
from pycrate_mobile.TS24301_IE import IDTYPE_IMSI, IDTYPE_IMEISV

from extract import main as extract_identities


def identity_octets(idtype: IDTYPE_IMSI | IDTYPE_IMEISV, ident: str) -> str:
    """
    idtype: the EPS identity type:
     1 = IMSI
     3 = IMEISV
     6 = GUTI (unsupported)
    ident: a string of digits representing the IMSI or IMEISV

    Returns hex octets representing the mobile identity, as an int.
    """
    type_and_parity = str(idtype + 8 * (len(ident) % 2))
    return int(ident[0] + type_and_parity + encode_bcd(ident[1:]).hex(), 16)


def yara_rules(typ: str, ident: str) -> str:
    ident_octets = identity_octets(int(typ), ident)
    # NAS messages encoded in LTE RRC UL-DCCH-Messages as the
    # dedicatedInfoNAS field may not be aligned within the frame.
    shift1 = f"{ident_octets << 1:x}"
    shift2 = f"{ident_octets << 2:x}"
    shift3 = f"{ident_octets << 3:x}"
    # mark don't care nibbles, where the value could change depending on the
    # surrounding bits or the type_and_parity nibble.
    shift1 = "??" + shift1[2:-1] + "?"
    shift2 = "??" + shift2[2:-1] + "?"
    shift3 = "0" + shift3[0] + "??" + shift3[3:-1] + "?"
    return f"""
rule mobile_id_type{typ}
xx
    meta:
        description = "The NAS PDU may not be aligned within the entire frame."
    strings:
        $ident{typ} = xx {ident_octets:x} yy
        $shift1 = xx {shift1} yy
        $shift2 = xx {shift2} yy
        $shift3 = xx {shift3} yy
    condition:
        $ident{typ} or $shift1 or $shift2 or $shift3
yy""".replace("xx", "{").replace("yy", "}")


def scan(needle: str, haystack: str):
    rules = yara.compile(needle)
    return yara.Scanner(rules).scan_file(haystack)


def matches_from(results) -> list:
    matches = []
    for mr in results.matching_rules:
        for p in mr.patterns:
            for m in p.matches:
                print(f"{m.offset:x}:{m.length:x}:{p.identifier}")
                matches.append(m)
    return sorted(matches, key=lambda m: m.offset)


def overwrite_file(filename: list, matches: str, with_bytes: bytes):
    with open(filename, "r+b") as f:
        for m in matches:
            f.seek(m.offset)
            print(
                f"{filename}: overwriting 0x{m.length:x} bytes at 0x{m.offset:x} with {with_bytes}"
            )
            f.write(m.length * with_bytes)


def main(haystacks: list[str]):
    "Redact sensitive values from haystack files."

    _EXIT = 0

    idents = set()
    for h in haystacks:
        print(h)
        try:
            packets = rdpcap(h)
            for i in extract_identities(packets):
                idents.add(i)
        except Scapy_Exception as e:
            print(f"{h}: {e}")

    needle = "\n".join(yara_rules(*i) for i in idents)
    print(needle)
    print()

    for h in haystacks:
        print(h)
        before = matches_from(scan(needle, h))
        if before == []:
            print(f"{h}: no mobile identities found")
            continue

        overwrite_file(h, before, b"\xaa")

        after = scan(needle, h)
        if len(after.matching_rules) > 0:
            print(f"{h}: error: redaction unsuccessful")
            _EXIT = 2
        else:
            print(f"{h}: verified")

    exit(_EXIT)


if __name__ == "__main__":
    if len(sys.argv) < 2 or sys.argv[1] in ("-h", "--help"):
        print("usage: redact.py FILE [FILE...]")
        print("redact mobile identity values from FILE ...")
        exit(1)
    main(sys.argv[1:])
