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
- **Null Cipher**: Tests whether the cell suggests using a null cipher (EEA0) in the RRC layer.
- **NAS Null Cipher**: Tests whether the security mode command at the NAS layer suggests using a null cipher (EEA0). This would usually only happen after a UE has successfully authenticated with the MME but still it shouldn't happen at all, this could be indicative of an attack though using SS7 to get key material from the HLR of the UE for a succesful authentication. It could also indicate an IMSI catcher which is connected to the mobile network MME and HLR through cooperation between government and telco. Or it could be a false positive if the telco is intending to use null ciphers (if encryption is illegal or something.)
- **Incomplete SIB**: Tests whether the Sib1 message contains a complete SIB chain (sib3, sib5, etc.) A legitimate SIB1 should contain timing information for at least 2 additional sibs (sib3, 4, and 5 being the most common) but a fake base station will often not bother to send additional SIBs beyond 1 and 2. On its own this might just be a misconfigured base station (though we have only seen it in the wild under suspicious circumstances) but combined with other heuristics such as **ISMI Requested** detection it should be considered a strong indicator of malicious activity.  