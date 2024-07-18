import asn1tools
import sys

ASN_FILES = [
    '../telcom-parser/specs/PC5-RRC-Definitions.asn',
    '../telcom-parser/specs/EUTRA-RRC-Definitions.asn',
]

TERMINATING_TYPE_NAMES = [
    'DL-CCCH-Message',
    'DL-DCCH-Message',
    'UL-CCCH-Message',
    'UL-DCCH-Message',
    'BCCH-BCH-Message',
    'BCCH-DL-SCH-Message',
    'PCCH-Message',
    'MCCH-Message',
    'SC-MCCH-Message-r13',
    'BCCH-BCH-Message-MBMS',
    'BCCH-DL-SCH-Message-BR',
    'BCCH-DL-SCH-Message-MBMS',
    'SBCCH-SL-BCH-Message',
    'SBCCH-SL-BCH-Message-V2X-r14',
]

def load_asn():
    return asn1tools.compile_files(ASN_FILES, cache_dir=".cache")

def get_terminating_types(rrc_asn):
    return [rrc_asn.types[name] for name in TERMINATING_TYPE_NAMES]

def search_type(haystack, needle):
    if haystack.type_name == needle or haystack.name == needle:
        return [needle]

    result = []
    if 'members' in haystack.__dict__:
        for name, member in haystack.name_to_member.items():
            for member_result in search_type(member, needle):
                result.append(f"{haystack.name} ({haystack.type_name}).{name}\n  {member_result}")
    elif 'root_members' in haystack.__dict__:
        for member in haystack.root_members:
            for member_result in search_type(member, needle):
                result.append(f"{haystack.name} ({haystack.type_name})\n  {member_result}")
    elif 'element_type' in haystack.__dict__:
        for element_result in search_type(haystack.element_type, needle):
            result.append(f"{haystack.name}[0] ({haystack.type_name})\n  {element_result}")
    elif 'inner' in haystack.__dict__:
        for inner_result in search_type(haystack.inner, needle):
            result.append(inner_result)

    return result


if __name__ == "__main__":
    type_name = sys.argv[1]
    print(f"searching for {type_name}")

    rrc_asn = load_asn()
    terminating_types = get_terminating_types(rrc_asn)
    needle = rrc_asn.types.get(type_name)
    if needle == None:
        raise ValueError(f"couldn't find type {type}")

    for haystack in terminating_types:
        for result in search_type(haystack.type, type_name):
            print(result + '\n')
