## Rayhunter tools

### `asn1grep.py`: a script for finding a datatype in ASN.1 files

`asn1grep` parses our ASN.1 spec files, then searches for a given datatype by recursively descending through the LTE-RRC types we care about. it then prints out each result as a "path" through the highly nested datatypes.

Setup:
1. `python -m venv .venv && . .venv/bin/activate`
2. `pip install -r requirements.txt`

Usage:
```
» python asn1grep.py IMSI                                  
searching for IMSI
PCCH-Message [message [message.c1 [c1 [c1.paging [paging [pagingRecordList[0] [ [ue-Identity [ue-Identity.imsi [IMSI]]]]]]]]]]
```
