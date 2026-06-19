# Adding a heuristic

This guide explains a small, complete path for adding a new Rayhunter
heuristic, from creating the analyzer to making it configurable and documenting
it for users.

## Before you start

Read these files first:

- `lib/src/analysis/analyzer.rs`: the `Analyzer` trait, `AnalyzerConfig`, and
  the `Harness` that wires analyzers together.
- `lib/src/analysis/test_analyzer.rs`: a compact example of a simple analyzer.
- `doc/heuristics.md`: the user-facing descriptions for built-in heuristics.
- `dist/config.toml.in`: the config template that exposes analyzer toggles.

If you are proposing a substantial new heuristic, start a discussion or issue
first so maintainers can confirm the idea fits the project.

## 1. Create the analyzer module

Add a new file under `lib/src/analysis/`, for example
`lib/src/analysis/my_new_heuristic.rs`.

Implement the `Analyzer` trait:

- `get_name()`: short user-facing name shown in reports and metadata.
- `get_description()`: explain what the heuristic looks for, what warnings it
  may emit, and common false-positive cases.
- `analyze_information_element()`: inspect each parsed
  `InformationElement` and return an `Event` only when the heuristic should
  surface something to the user.
- `get_version()`: increment this when you make a substantial change to the
  heuristic's behavior.

Keep state minimal. Analyzers may run for many hours and process large numbers
of messages in parallel.

## 2. Export the module

Add the new module to `lib/src/analysis/mod.rs`:

```rust
pub mod my_new_heuristic;
```

If your analyzer needs to be constructed from `analyzer.rs`, also import it
there alongside the existing analyzers.

## 3. Register it in the harness

Rayhunter only runs analyzers that are added in `Harness::new_with_config()` in
`lib/src/analysis/analyzer.rs`.

There are usually three edits in that file:

1. Add a boolean field to `AnalyzerConfig`.
2. Set its default value in `impl Default for AnalyzerConfig`.
3. Add a conditional block in `Harness::new_with_config()` that constructs your
   analyzer and passes it to `harness.add_analyzer(...)`.

Follow the existing analyzers as examples.

## 4. Expose a config toggle

Add the new option to `dist/config.toml.in` under `[analyzers]` so users can
enable or disable it from configuration and the web UI.

Try to keep the config key descriptive and consistent with the Rust field name
in `AnalyzerConfig`.

## 5. Document it for users

Add a section to `doc/heuristics.md` that covers:

- what the heuristic detects
- why it matters
- what severity or message patterns users should expect
- known or likely false positives

Write this section for operators, not for contributors. The implementation
details belong in code and contributor docs, while `heuristics.md` should stay
focused on what a user needs to understand when the heuristic fires.

## 6. Add tests

If possible, add focused tests alongside the analyzer or in the existing test
suite that prove:

- the heuristic triggers on the intended input
- it does not trigger on common non-matching input
- stateful behavior works as expected

Small, direct tests are better than broad fixtures that are hard to reason
about.

## 7. Run local checks

Before opening a PR, run at least:

```sh
cargo fmt
cargo clippy
cargo test
```

If your heuristic depends on new captures or parser behavior, do a manual check
too and summarize what you verified in the PR description.

## 8. Keep reports compatible

Each analyzer contributes metadata through `Harness::get_metadata()`, including
its name, description, and version. Be deliberate about version bumps when the
heuristic changes in a way that affects interpretation of old results.

If you change report structure rather than just analyzer behavior, review the
report versioning and normalization logic in `lib/src/analysis/analyzer.rs`
before merging.
