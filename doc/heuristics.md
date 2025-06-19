# Heuristics

Rayhunter includes several analyzers to detect potential IMSI catcher activity. These can be enabled and disabled in your [config.toml](./configuration.md) file.

## Available Analyzers

- **IMSI Requested**: Tests whether the eNodeB sends an IMSI Identity Request NAS message. This 
  can sometimes happen under normal circumstances when the network doesn't already have a TMSI 
  (Temporary Mobile Subscriber ID or GUTI in 5G terminology) for your device. This most often 
  happens when you first turn the device on, especially after it has been off for a long time or 
  if you are in an area where ther is absolutely no connection to your service provider. This can 
  also happen if you leave your device on while on an airplane and it suddenly connects to a new
  tower after being disconnected for a long time. 
  However, if you get this warning at a time when you have been steadily connected to towers and the device has been on for a while it can be treated as suspcious. 
- **Connection Release/Redirected Carrier 2G Downgrade**: Tests if a cell
  releases our connection and redirects us to a 2G cell. This heuristic only
  makes sense in the US or other countries where there are no more operating 2G base stations.
  Users in contries where 2G is still in service (such as most of EU) may want to disable it.
  See https://en.wikipedia.org/wiki/2G#Past_2G_networks for information about your country. 
- **LTE SIB6/7 Downgrade**: Tests for LTE cells broadcasting a SIB type 6 and 7
  which include 2G/3G frequencies with higher priorities
- **Null Cipher**: Tests whether the cell suggests using a null cipher (EEA0).
