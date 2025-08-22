# Re-analyzing recordings

Every once in a while, Rayhunter refines its heuristics to detect more kinds of
suspicious behavior, and to reduce noise from incorrect alerts.

This means that your old green recordings may actually contain data that is now
deemed suspicious, and also old red recordings may become green.

You can re-analyze any old recording inside of Rayhunter by clicking on "N
warnings" to expand details, then clicking the "re-analyze" button.

## Analyzing recordings on Desktop

If you have a PCAP or QMDL file but no rayhunter, you can analyze it on desktop
using the `rayhunter-check` CLI tool. That tool contains the same heuristics as
Rayhunter and will also work on traffic data captured with other tools, such as
QCSuper.

Since, 0.7.0, `rayhunter-check` is included in the release zipfile.

You can build `rayhunter-check` from source with the following command:
`cargo build --bin rayhunter-check` 

## Usage
```sh
rayhunter-check [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>   Path to the PCAP, or QMDL file. If given a directory will 
                        recursively scan all pcap, qmdl, and subdirectories 
  -P, --pcapify       Turn QMDL file into PCAP     
      --show-skipped  Show skipped messages
  -q, --quiet         Print only warnings
  -d, --debug         Print debug info 
  -h, --help          Print help
  -V, --version       Print version
```
### Examples 
`rayhunter-check -p ~/Downloads/myfile.qmdl`

`rayhunter-check -p ~/Downloads/myfile.pcap`

`rayhunter-check -p ~/Downloads #Check all files in downloads`

`rayhunter-check -d -p ~/Downloads/myfile.qmdl #run in debug mode`