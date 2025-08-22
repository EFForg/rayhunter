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

You need Rust installed and the rayhunter repository checked out:

```sh
cargo run -p rayhunter-check -- --path ~/Downloads/myfile.qmdl
cargo run -p rayhunter-check -- --path ~/Downloads/myfile.pcap
```

Since, 0.7.0, `rayhunter-check` is included in the release zipfile.
