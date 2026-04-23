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
  -p, --path <PATH>   Path to the PCAP, or QMDL file. If given a directory will
                        recursively scan all pcap, qmdl, and subdirectories
  -P, --pcapify       Turn QMDL file into PCAP in --output (requires --output)
      --show-skipped  Show skipped messages (also includes them in NDJSON output)
      --format <FORMAT>  Output format: [possible values: text, json].
                        JSON is NDJSON (one record per line) and is always
                        written to stdout.
  -o, --output <DIR>  Optional directory for output files. With --format json,
                        NDJSON is also written to <output>/<input>.ndjson
                        alongside stdout. Required for --pcapify.
  -q, --quiet         Print only warnings
  -d, --debug         Print debug info
  -h, --help          Print help
  -V, --version       Print version
```

**Stdout vs. stderr:** with `--format json`, NDJSON records go to **stdout**;
all log output (including the analyzer listing, per-file headers, warnings,
and any errors) goes to **stderr**. This makes it safe to pipe stdout into
`jq` or any other NDJSON consumer without log lines mixing in.

**Skipped messages:** rows that exist only because a packet was skipped
during analysis are omitted from NDJSON by default. Pass `--show-skipped`
to include them in stdout (and in any `--output` file copy), the same way
`--show-skipped` enables the per-reason summary in text mode.

**Output file names:** with `--format json` or `--pcapify`, files in
`--output` are named `<input-file-name>.<extension>`. For example,
`capture.qmdl` produces `capture.qmdl.ndjson` (and, with `--pcapify`,
`capture.qmdl.pcapng`); `capture.pcap` produces `capture.pcap.ndjson`.
This preserves dotted names like `2026-01-02_10.05.00_capture.qmdl` and
avoids collisions when a directory contains both a `.qmdl` and a `.pcap`
with the same stem.

`rayhunter-check` will refuse to overwrite an existing file in
`--output`. If two inputs from different directories share the same file
name, the second one's file copy is skipped with an error on stderr;
NDJSON for that input still goes to stdout. Point `--output` at an empty
directory (or remove the conflicting file) and re-run if you need a
separate file copy of every input.

When `--path` is a directory, `rayhunter-check` processes inputs
sequentially. With `--format json`, each input begins with its own
metadata record on stdout followed by its analysis rows; consumers can
key off the `analyzers` field to know when a new input starts.

### Examples 
`rayhunter-check -p ~/Downloads/myfile.qmdl`

`rayhunter-check -p ~/Downloads/myfile.pcap`

`rayhunter-check -p ~/Downloads #Check all files in downloads`

`rayhunter-check -d -p ~/Downloads/myfile.qmdl #run in debug mode`

`rayhunter-check -p ~/Downloads/myfile.qmdl --format json #stream NDJSON to stdout`

`rayhunter-check -p ~/Downloads/myfile.qmdl --format json | jq . #pipe into jq`

`rayhunter-check -p ~/Downloads/myfile.qmdl --format json -o ./reports #also write a copy to ./reports/myfile.qmdl.ndjson`
