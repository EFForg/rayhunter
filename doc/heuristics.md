# Heuristics

Rayhunter includes several analyzers to detect potential IMSI catcher activity. These can be enabled and disabled in your [config.toml](https://github.com/EFForg/rayhunter/blob/main/dist/config.toml.example) file.

## Available Analyzers

- **IMSI Requested**: Tests whether the ME sends an IMSI Identity Request NAS message
- **Connection Release/Redirected Carrier 2G Downgrade**: Tests if a cell
  releases our connection and redirects us to a 2G cell. This heuristic only
  makes sense in the US, European users may want to disable it.
- **LTE SIB6/7 Downgrade**: Tests for LTE cells broadcasting a SIB type 6 and 7
  which include 2G/3G frequencies with higher priorities
- **Null Cipher** (disabled by default): Tests whether the cell suggests using a null cipher (EEA0).
  This is currently disabled by default due to a parsing bug triggering false
  positives.
