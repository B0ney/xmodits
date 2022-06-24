# Dumping samples from ProTracker modules


The **.MOD** format should be the easiest to dump samples as it's the simplest format.

* Samples are stored as **mono 8-bit PCM**.
* Every sample has a maximum size of **128KB**
* Each sample is stored sequentially

## Sources
* https://wiki.multimedia.cx/index.php/Protracker_Module

Note:
Data is read as BE, (This doesn't apply to sample data as it's only 8-bits remember?)

Default number of samples allocated is **31**, but it will be **15** if:

**4 bytes** at offset **0x0438** contains non-ASCII values.

## Header Infomation
```
0x0000 => [u8; 20] module name
```



to find the number of patterns, 
we select 128 bytes before offset 0x0438,
find the highest value, add 1.

Use this (0x0438 + value * 1024) to skip pattern data
which will lead us to the sample data

sample data is placed sequentially.

