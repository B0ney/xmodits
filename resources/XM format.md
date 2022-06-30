# Dumping samples from Impulse Tracker Modules


### Sources:
https://ftp.modland.com/pub/documents/format_documentation/FastTracker%202%20v2.04%20(.xm).html
https://github.com/milkytracker/MilkyTracker/blob/master/resources/reference/xm-form.txt


## IT Header Structure
Size: **60 Bytes**
```
0x0000 => [char; 17]    "Extended Module: " (last character is a space)
0x0011 => [char; 20]    Module Name, padded with zeros
0x0025 => [char]        Always 0x1a
0x0026 => [char; 20]    Tracker Name
0x003A => [u16]         Version number, it should be 0x0104 
                        (versions below have major differences)
0x003C => [u32]         Header Size
0x0040 => [u16]         Song length
0x0041 => [u16]         Song restart position
0x0042 => [u16]         number of channels
0x0046 => [u16]         number of patterns (max 256)
0x0048 => [u16]         number of instruments (max 128)
0x004A => [u16]         flags
0x004C => [u16]         default tempo
0x004E => [u16]         default bpm
0x0050 => [u8; 256]     pattern order table

0x003C =>               NEW DATA STARTS FROM HERE
```