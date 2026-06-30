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
  -P, --pcapify       Convert each .qmdl to PCAPNG beside the file (or under
                        --output if you set it)
      --show-skipped  Show skipped messages (also includes them in JSON output)
      --format <FORMAT>  Output format: [possible values: text, json].
                        'json' writes a JSON array of per-file reports to
                        stdout.
  -o, --output <DIR>  Optional directory for output files. With --format json,
                        each input's report is also written to
                        <output>/<input>.json alongside stdout. With
                        --pcapify, PCAPNG files go here instead of next to
                        each .qmdl.
  -q, --quiet         Print only warnings
  -d, --debug         Print debug info
  -h, --help          Print help
  -V, --version       Print version
```

**JSON shape:** with `--format json`, stdout is a single JSON array. Each
element is one analyzed input file and has the form:

```json
[
  {
    "path": "myfile.qmdl",
    "metadata": { "analyzers": [ ... ], "rayhunter": { ... }, "report_version": 2 },
    "rows": [ { "packet_timestamp": "...", "skipped_message_reason": null, "events": [ ... ] }, ... ]
  }
]
```

`metadata` mirrors the header the on-device daemon writes (the analyzers
that ran and the report format version), and `rows` holds the analysis
rows for that file. stdout is always an array, even for a single input, so
consumers get the same shape regardless of how many files matched
(`jq '.[0]'` for the lone object).

**Stdout vs. stderr:** the JSON array goes to **stdout**; all log output
(including the analyzer listing, per-file headers, warnings, and any
errors) goes to **stderr**. This makes it safe to pipe stdout into `jq` or
any other JSON consumer without log lines mixing in.

**Skipped messages:** rows that exist only because a packet was skipped
during analysis are omitted from the JSON by default. Pass `--show-skipped`
to include them in stdout (and in any `--output` file copy), the same way
`--show-skipped` enables the per-reason summary in text mode.

**Output file names:** outputs are named `<input-file-name>.<extension>`.
With `--format json` and `--output`, each input's report is written as a
single JSON object to that directory (e.g. `capture.qmdl` →
`capture.qmdl.json`). With `--pcapify` and no `--output`, the `.pcapng` is
written **next to** the QMDL (e.g. `~/Downloads/foo.qmdl` →
`~/Downloads/foo.qmdl.pcapng`). With `--pcapify` and `--output`, PCAPNG
files use the same naming under the given directory. `capture.pcap` with
`--format json` and `--output` produces `capture.pcap.json`. This preserves
dotted names like `2026-01-02_10.05.00_capture.qmdl` and avoids collisions
when a directory contains both a `.qmdl` and a `.pcap` with the same stem.

`rayhunter-check` will refuse to overwrite an existing JSON or PCAPNG file
it would create. If you use `--output` and two inputs from different
directories share the same file name, the second one's file copy is
skipped with an error on stderr; that input's report still appears in the
stdout array. Point `--output` at an empty directory (or remove the
conflicting file) and re-run if you need a separate JSON copy of every
input.

When `--path` is a directory, `rayhunter-check` processes inputs
sequentially and emits one array element per input, in the order they were
analyzed. Each element carries its own `path` so consumers can tell the
reports apart.

### Examples 
`rayhunter-check -p ~/Downloads/myfile.qmdl`

`rayhunter-check -p ~/Downloads/myfile.pcap`

`rayhunter-check -p ~/Downloads #Check all files in downloads`

`rayhunter-check -d -p ~/Downloads/myfile.qmdl #run in debug mode`

`rayhunter-check -p ~/Downloads/myfile.qmdl --format json #emit JSON to stdout`

`rayhunter-check -p ~/Downloads/myfile.qmdl --format json | jq . #pipe into jq`

`rayhunter-check -p ~/Downloads/myfile.qmdl --format json -o ./reports #also write a copy to ./reports/myfile.qmdl.json`

`rayhunter-check -p ~/Downloads/myfile.qmdl --pcapify #writes ~/Downloads/myfile.qmdl.pcapng`
