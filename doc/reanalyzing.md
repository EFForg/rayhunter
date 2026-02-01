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

Since 0.6.1, `rayhunter-check` is included in the release zipfile.

You can build `rayhunter-check` from source with the following command:
`cargo build --bin rayhunter-check`

## Usage
```sh
rayhunter-check [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>          Path to the PCAP or QMDL file. If given a directory will
                             recursively scan all pcap, qmdl files and subdirectories
  -o, --output <OUTPUT>      Output directory for generated files (.ndjson reports and
                             .pcapng files). If not specified, no files are written
  -P, --pcapify              Convert qmdl files to pcap (requires --output)
  -r, --report <REPORT>      Report format: 'log' outputs to stderr, 'ndjson' outputs
                             to stdout (or files if --output is set) [default: log]
                             [possible values: log, ndjson]
      --show-skipped         Show skipped messages
  -q, --quiet                Print only warnings/errors to stderr
  -d, --debug                Print debug info to stderr
  -h, --help                 Print help
  -V, --version              Print version
```

**Note:** All log output (info, warnings, errors) is written to stderr. This allows you to
redirect analysis output separately from logs when using `--report ndjson`.

### Report Formats

- **log** (default): Prints analysis results in human-readable format to stderr
- **ndjson**: Outputs newline-delimited JSON format
  - Without `--output`: Writes to stdout (can be piped or redirected)
  - With `--output`: Creates `.ndjson` files in the specified directory alongside input files

### File Generation

The `--output` flag controls where generated files are written:

- **NDJSON reports**: When using `--report ndjson --output <dir>`, a `.ndjson` file is created
  for each analyzed capture in the specified directory
- **PCAP conversion**: When using `--pcapify --output <dir>`, `.pcapng` files are created from
  QMDL files in the specified directory

Without `--output`, no files are created (NDJSON is written to stdout for piping/redirection).

### Examples
`rayhunter-check -p ~/Downloads/myfile.qmdl`

`rayhunter-check -p ~/Downloads/myfile.pcap`

`rayhunter-check -p ~/Downloads #Check all files in downloads`

`rayhunter-check -d -p ~/Downloads/myfile.qmdl #run in debug mode`

`rayhunter-check -p ~/Downloads -r ndjson > analysis.ndjson #Output NDJSON to stdout`

`rayhunter-check -p ~/Downloads -r ndjson -o ./reports #Generate NDJSON report files`

`rayhunter-check -p file.qmdl -P -o ./output #Convert QMDL to PCAP in output directory`
