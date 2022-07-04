# Dumping samples from eXtended Modules (XM)



### Sources:
https://ftp.modland.com/pub/documents/format_documentation/FastTracker%202%20v2.04%20(.xm).html
https://github.com/milkytracker/MilkyTracker/blob/master/resources/reference/xm-form.txt


## XM Header Structure
Size before **0x003C** = **60 Bytes**
```
0x0000 => [char; 17]    "Extended Module: " (last character is a space)
0x0011 => [char; 20]    Module Name                                     [needed]
0x0025 => [char]        Always 0x1a     (Magic number)                  
0x0026 => [char; 20]    Tracker Name                                    [needed]
0x003A => [u16]         Version number, it should be  at least 0x0104   [needed]

0x003C => [u32]         Header Size (calculated from **this** offset)
0x0040 => [u16]         Song length
0x0041 => [u16]         Song restart position
0x0042 => [u16]         chnnum      number of channels
0x0046 => [u16]         patnum      number of patterns (max 256)        [needed]
0x0048 => [u16]         insnum      number of instruments (max 128)     [needed]
0x004A => [u16]         flags                                           [needed]
0x004C => [u16]         default tempo
0x004E => [u16]         default bpm
0x0050 => [u8; 256]     pattern order table

0x0150 => [PAT_HEADER; patnum]  => XM patten headers

0x0150
    + (PAT_HEADER * patnum) => [INS_HEADER; insnum] XM instrument headers [needed]

0x0150
    + (PAT_HEADER * patnum)
    + (INS_HEADER * insnum) => END OF XM FILE (useful for internal checking)
``` 

## XM Pattern Header data   [PAT_HEADER]
Offsets are relative to the start of this header.

Pattern data follows after header

Size: **9 bytes** +  **packed_pattern_data size**

```
0x0000 => [u32]     Pattern header length
0x0004 => [u8]      Packing type (Should be 0)
0x0005 => [u16]     number of rows in pattern   (1..256)
0x0007 => [u16]     Packed pattern data size 

(needs checking)
IF PACKED PATTERN DATA SIZE != 0:

    0x0009 => [u8; PAK_PAT_DAT_SIZE]    packed pattern data

    0x0009 
        + PAK_PAT_DAT_SIZE => NEW DATA STARTS FROM HERE

ELSE:

    0x0009 => NEW DATA STARTS FROM HERE  

```

## XM instrument headers
Offsets are relative to the start of this header.

[Notice](https://github.com/milkytracker/MilkyTracker/blob/master/resources/reference/xm-form.txt#L189=)

```
0x0000 => [u32]         Pattern header size**
0x0004 => [char; 22]    Instrument name
0x001a => [u8]          Instrument type
0x001b => [u16]         Number of samples in instrument (smpnum)

IF SMPNUM > 0:

    0x001b  => ADDITIONAL HEADER

    0x00F3 => SAMPLE HEADERS

    0x00F3 
        + SAMPLE HEADERS => XM SAMPLE DATA
```
**The header size tends to include the size of itself + total sample header length?

### XM ADDITIONAL HEADER
Size: 214 Bytes

Offsets are relative to XM instrument header

```
0x001D => [u32]         Sample header size              [needed]
0x0021 => [u8; 96]      sample number for all notes
0x0081 => [u8; 48]      points for vol envelope
0x00B1 => [u8; 48]      points for pan envelope
0x00E1 => [u8]          vol point num
0x00E2 => [u8]          pan point num
0x00E3 => [u8]          vol sus point
0x00E4 => [u8]          vol loop start point
0x00E5 => [u8]          vol loop end point
0x00E6 => [u8]          pan sus point
0x00E7 => [u8]          pan loop start point
0x00E8 => [u8]          pan loop end point
0x00E9 => [u8]          vol type
0x00EA => [u8]          pan type
0x00EB => [u8]          vib type
0x00EC => [u8]          vib sweep
0x00ED => [u8]          vib depths
0x00EE => [u8]          vib rate
0x00EF => [u16]         vol fadeout
0x00F1 => [u16]         reserved

0x00F3 => NEW DATA STARTS FROM HERE
```
## XM Sample header (**SAMPLE HEADERS**)

```
0x0000 => [u32]         sample length           [needed]
0x0004 => [u32]         sample loop start
0x0008 => [u32]         sample loop length
0x000C => [u8]          volume
0x000D => [u8]          finetune
0x000E => [u8]          type of sample flag **  [needed]
0x000F => [u8]          panning
0x0010 => [i8]          relative note number
0x0011 => [u8]          reserved
0x0012 => [char; 22]    sample name             [needed]

0x0040 => NEW DATA STARTS FROM HERE

```
** sample flag

**bits 0-1**:
* 00 = no loop
* 01 = forward loop
* 11 = ping-pong loop

**bit 4**:
* 0 = 8-bit sample data
* 1 = 16-bit sample data

## XM SAMPLE DATA

**Reference:** 

https://github.com/schismtracker/schismtracker/blob/master/fmt/xm.c

https://github.com/milkytracker/MilkyTracker/blob/master/resources/reference/xm-form.txt#L629=


Sample data is encoded in delta values, this is done to achive a better compression ratio when compressed with external programs.

At this stage, we have the following infomation about each sample:
* Name
* Length (assumed in bytes?)
* Bits per sample (8/16)
* Index (can be calculated)
* Sample is delta encoded

We **Don't** have the following infomation:
* Sampling frequency
* Is sample mono/stereo?

### Sampling frequency

Finding the sample frequencies is quite tricky.

We need to pass some infomation through an interpolation function to reconstruct the sampling frequency.

Before we do that we need to check the type of interpolation. 

This can be located by reading bit flag 0 in the header:

0 = Amiga frequency table (logarithmic)

1 = Linear frequency table
