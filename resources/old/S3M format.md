# S3M format



ScreamTracker stores sample data at the end of the file right after the pattern data.
## References
https://moddingwiki.shikadi.net/wiki/S3M_Format

## Header Format

```
0x0000 => [char; 28]    title
0x001C => [u8]          "0x1A"
0x001D => [u8]          "0x10" for S3M
0x001E => [u16]         "0x0000"
0x0020 => [u16]         ORDER_COUNT         (needed)
0x0022 => [u16]         INSTRUMENTS         (needed)
0x0024 => [u16]         PATTERN PTR COUNT   (needed)
0x0026 => [u16]         FLAGS
0x0028 => [u16]         TRACKER VERSION
0x002A => [u16]         Sample type 1=Signed, 2=Unsigned
0x002C => [char; 4]     "SCRM"
0x0030 => [u8]          global volume
0x0031 => [u8]          initial speed
0x0032 => [u8]          initial tempo
0x0033 => [u8]          master volume
0x0034 => [u8]          ultra click removal
0x0035 => [u8]          default pan
0x0036 => [u8; 8]       reserved
0x003E => [u16]         Parapointer to additional data
0x0040 => [u8; 32]              Channel Settings
0x0060 => [u8; ORDER_COUNT]     Order to play patterns

0x0060 
    + ORDER_COUNT  => [u16; INSTRUMENTS] (needed) list of pointers to each instrument data

0x0060 
    + ORDER_COUNT 
    + (INSTRUMENTS * 2) => [u16; PATTERN_PTR_COUNT] list of pointers to pattern data

0x0060 
    + ORDER_COUNT 
    + (INSTRUMENTS * 2)
    + (PATTERN_PTR_COUNT * 2) => NEW DATA STARTS FROM HERE

```

**NOTE:**

Each parapointer is an offset from the start of the file in units of 16 bytes.

This pointer **MUST** be converted to a byte-level offset.

You can do so by shifting left 4. 

## Instrument Header
Size: **13 Bytes**
```
0x0000 => [u8]          type (0=empty, 1=PCM instrument n>1=Adlib instrument)
0x0001 => [char; 12]    Original instrument filename
```

## Sample Header (PCM Only)
Comes after instrument header.

Size: **67** bytes. CHECK

The header stores a 24 bit pointer to the file
Data is LE.

smp ptr = Sample Parapointer

```
0x0000 => [u8]          Upper 8-bits of smp ptr                         (needed)
0x0001 => [u16]         Lower 16-bits (LE) of smp ptr                   (needed)
0x0003 => [u32]         length of sample in bytes (lower 16 bits (LE))  (needed) 
0x0007 => [u32]         loop start 
0x000B => [u32]         loop end
0x000F => [u8]          volume
0x0010 => [u8]          reserved, always 0x00  
0x0011 => [u8]          pack, 0=unpacked, 1=DP30ADPCM [Deprecated]      (needed)
0x0012 => [u8]          flags**                                         (needed)
0x0013 => [u32]         c2sp sample rate for C-4 note                   (needed)
0x0017 => [u8; 12]      always 0, don't worry about it
0x0023 => [char; 28]    sample title                                    (needed)
0x003F => [u32]         "SCRS"                                          (needed)

0x0043 => NEW DATA STARTS FROM HERE


```

** flags NEEDS CHECKING
```
0000_000x => loop on
0000_00x0 => stereo
0000_x000 => 16 bit LE sample



```

**NOTE:**

For stereo samples, sample data is length bytes for left channel then length bytes for right channel.

Remember that stereo wav files have data encoded as follows:

L => Left channel sample
R => Right channel sample

LRLRLRLRLRLRLRLRL