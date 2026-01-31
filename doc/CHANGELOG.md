# rayhunter-check Changelog

## [Unreleased]

### Added
- **JSON Output Format** (Issue #570)
  - Added `--format` flag with `text` (default) and `json` options
  - JSON output uses NDJSON (Newline Delimited JSON) format
  - Format matches rayhunter daemon's analysis output exactly
  - When processing directories, creates individual `.ndjson` files per capture
  - Includes complete metadata (analyzers, system info, versions)
  - Fully backward compatible - text format remains the default
  - Comprehensive test suite with 14 unit and integration tests
  - Documentation in `JSON_FORMAT.md` and `EXAMPLE_JSON_USAGE.md`

### Technical Details
- Added `serde` and `serde_json` dependencies for JSON serialization
- Implemented `Reporter` trait with `TextReporter` and `JsonReporter`
- Added `Clone` derive to `ReportMetadata`, `AnalyzerMetadata`, and `RuntimeMetadata`
- Created separate functions for JSON file output vs stdout output
- Zero performance impact on existing text mode operation

### Use Cases Enabled
- Automated analysis pipelines
- Building web UIs and wikis from capture analysis
- Statistics collection and aggregation
- Integration with data processing tools (jq, Python, JavaScript)
- Programmatic access to analysis results

### Testing
- 10 unit tests for JSON serialization and NDJSON format
- 4 integration tests for CLI functionality
- Verified output format matches daemon implementation
- All tests passing with zero warnings

## Previous Releases
(See main rayhunter CHANGELOG for earlier versions)
