# Heuristics

Rayhunter includes several analyzers to detect potential IMSI catcher activity. These can be enabled and disabled in your [configuration](./configuration.md) file.

## Available Analyzers

### IMSI Requested

This analyser tests whether the eNodeB sends an IMSI Identity Request NAS message.

Mobile network primarily requests IMSI number from mobile device during initial network attachment or when the network cannot identify the mobile device by its temporary identification (TMSI - *Temporary Mobile Subscriber Identity* or GUTI - *Globally Unique Temporary Identifier* in 4G/5G terminology).

IMSI request therefore usually happens when you first turn the device on especially after it has been off for a long time. Another possibility is, that you reboot your mobile device and your temporary ID expired. Sometimes temporary identification can expire if you have been in an area where there is absolutely no connection to your service provider or after you left your device on an airplane mode and then reconnect to the network (especially being disconnected for a long time). IMSI could also be requested when you connect to a new network (for instance for roaming), when you swap she SIM card or when your device moves to a new *Tracking Area* or *Location Area* and the network can not map the temporary identification to your device. IMSI number can also be requested after core network reboot.

It should also be noted that the network periodically reassigns your device new temporary identification to enhance security and avoid tracking, but in that cases usually does not request IMSI.

However, if you get this warning at a time when you have been steadily connected to towers and the device has been on for a while, this could be a sign of IMSI catcher use.

### Connection Release/Redirected Carrier 2G Downgrade

This analyser tests if a base station releases your device's connection and redirects your device to a 2G base station. This heuristics is useful, because many commercial IMSI catchers operate in a such way that they downgrade connection to 2G where they can intercept the communication (by performing man-in-the-middle attack).

This heuristic is the most useful in the United States or other countries where there are no more operating 2G base stations. See [Wikipedia page on past 2G networks](https://en.wikipedia.org/wiki/2G#Past_2G_networks) for information about your country. In countries where 2G is still in service (such as most of EU), this heuristics may trigger false positives. In that case you should consider disabling it. However this heuristics has been vastly improved to reduce false positive warnings and new tests in European networks show that false positives are vastly reduced.

### LTE SIB6/7 Downgrade

This analyser tests if LTE base station is broadcasting a SIB type 6 and 7 messages which include 2G/3G frequencies with higher priorities.

SIB (*System Information Block*) Type 6 and 7 are specific types of broadcast messages sent by the base station (eNodeB in 4G networks) to mobile devices. They contain essential radio-related configuration parameters to help mobile device perform cell reselection.

IMSI catchers exploit the fact that many SIB broadcast messages are not encrypted or authenticated. This allows them to pretend to be a legitimate cell by broadcasting fake system information in order to force mobile devices to downgrade from more secure 4G (LTE) to less secure 2G (GSM) network and then steal IMSI and/or perform man-in-the-middle attack. That is why this is also called a downgrade attack.

SIB6 is used for cell reselecion to CDMA2000 systems which are not supported by many modern mobile phones, and SIB7 Provides the mobile device with information to perform cell reselection to GSM/EDGE networks. Therefore SIB6 messages are quite rare, while malformed SIB7 messages are much more frequent in practice. 

### Null Cipher

This analyser tests whether the cell suggests using a null cipher (EEA0) in the RRC layer. That means that encryption between your mobile device and base station is turned off.

Normally this should never happen, because null cipher is used almost exclusively for testing and debugging in labs or in controlled environments. Sometimes null cipher is used if encryption negotiation fails or isn’t supported (however in most networks this should not be the case). Also, some regulations allow unencrypted communications in **specific** emergency cases.

The general rule is, that null cipher should never be used in commercial deployments, except in very controlled conditions (e.g., test labs) or in a very specific regulatory-approved use cases.

On the other hand, IMSI catchers often use null cipher to avoid setting up secure contexts (because they lack valid keys) and/or to trick mobile device into using unencrypted links (which makes eavesdropping easier).

### NAS Null Cipher

This analyser tests whether the security mode command at the NAS layer suggests using a null cipher (EEA0). This would usually only happen after a mobile device has successfully authenticated with the MME (*Mobility Management Entity* - core network component that handles signaling and control) but still it shouldn't happen at all. This could be indicative of an attack though using SS7 (*Signaling System 7* - a set of telecommunication protocols used to set up and manage calls and other services) to get key material from the HLR (*Home Location Register* - a database in mobile telecommunications networks that stores subscriber information) of the mobile phone for a successful authentication.

It could also indicate an IMSI catcher which is connected to the mobile network MME and HLR through cooperation between government and telecom provider. Or it could be a false positive if the telecom provider is intending to use null ciphers (if encryption is illegal in some country, or they have some misconfiguration of the network), however this should be very rare case.

### Incomplete SIB

This analyser tests whether the SIB1 message contains a complete SIB chain (SIB3, SIB5, etc.). A legitimate SIB1 message should contain timing information for at least 2 additional SIBs (SIB3, 4, and 5 being the most common) but a fake base station will often not bother to send additional SIBs beyond 1 and 2 (i. e. some IMSI catchers send just SIB1 and *one additional* SIB).

On its own this might just be a misconfigured base station (though we have only seen it in the wild under suspicious circumstances) but combined with other heuristics such as **IMSI Requested** detection it should be considered as a strong indicator of malicious activity.

### Test Analyzer

This analyzer is great for testing if your Rayhunter installation works. It will alert every time a new tower is seen (specifically every time a tower broadcasts a SIB1 message.) It is designed to be very noisey so we do not reccomend leaving it on but if this alerts it means your Rayhunter device is working! 