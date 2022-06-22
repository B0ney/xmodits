# Dumping samples from Impulse Tracker Modules

We're only interested in dumping the samples, so it should be decent.

IT Samples are raw signed PCM (PLEASE VERIFY), the sample rate.

Some Are 16-bit.
note that some samples can be compressed.


### Terminology:
* **i** = Signed, **u** = Unsigned
* [ **u8** ]  -> Unsigned 8-bit integer
* [ **i8** ]  -> Signed 8-bit integer
* **[u8; 26]**  -> Unsigned 8-bit **array** with size of 26


### IT file structure:
* Header (**192 bytes**)
* Pattern Orders with bytes indicated by ordnum
* Instrument Parameters with bytes 4x size of insnum
* Sample Parameters with bytes 4x size of smpnum
* Pattern Parameters with bytes 4x value of patnum
* **Undocumented data**
* Sample metadata, each **80 bytes** number specified by **smpnum**

Note: I don't think it's worth documenting out every part of the file, why not just look where the first "IMPS" is located

## IT Header byte structure
Offsets are in **bytes** and they are relative to the **"IMPM"** header.

This should be the first thing you see when you load an **.it**

Total Size: **192 Bytes** 

Reference: https://github.com/schismtracker/schismtracker/blob/master/include/it_defs.h#L7=

```
0x0000 -> "IMPM"        [u32]       [need this for verification purposes]
0x0004 -> songname      [i8; 26]    [might be needed]
0x001E -> hilight_minor [u8]
0x001F -> hilight_major [u8]
0x0020 -> ordnum        [u16]
0x0022 -> insnum        [u16] 

0x0024 -> smpnum        [u16]       [need this] (number of samples)

0x0026 -> patnum        [u16]
0x0028 -> cwtv          [u16]
0x002A -> cmwt          [u16]
0x002C -> flags         [u16]
0x002E -> special       [u16]
0x0030 -> globalvol     [u8]
0x0031 -> mv            [u8]
0x0032 -> speed         [u8]
0x0033 -> tempo         [u8]
0x0034 -> sep           [u8]
0x0035 -> pwd           [u8]
0x0036 -> msglength     [u16]
0x0038 -> msgoffset     [u32]
0x003C -> reserved      [u32]
0x0040 -> chnpan        [u8; 64]
0x0080 -> chnvol        [u8; 64]

0x00c0 -> END OF HEADER DATA, NEW DATA STARTS HERE. 

```

## IT Sample byte structure
They do not store samples but rather metadata about them. 

They'll also point to the raw samples. 

The sample pointer needs to be converted to bigendian


Offsets are in **bytes** and they are relative to the **"IMPS"** header

Total Size: **80 Bytes**

Reference: https://github.com/schismtracker/schismtracker/blob/master/include/it_defs.h#L102=

```
0x0000 -> "IMPS"        [u32]       [We need this for verification]
0x0004 -> filename      [i8; 12]    [We need this]

0x0010 -> zero          [u8]
0x0011 -> gvl           [u8]        
0x0012 -> flags         [u8]
0x0013 -> vol           [u8]

0x0014 -> name          [i8; 26]    [We need this]

0x002E -> cvt           [u8]
0x002F -> dfp           [u8]

0x0030 -> length        [u32]       [We need this]

0x0034 -> loopbegin     [u32]
0x0038 -> loopend       [u32]
0x003C -> C5Speed       [u32]
0x0040 -> susloopbegin  [u32]
0x0044 -> susloopend    [u32]

0x0048 -> samplepointer [u32]       [We need this]

0x004C -> vis           [u8]
0x004D -> vid           [u8]
0x004E -> vir           [u8]
0x004F -> vit           [u8]

0x0050 -> END OF INSTRUMENT STRUCT, NEW STRUCT STARTS HERE.
```

observations:

**16-bit samples** have a flag of   **0bxxxx_xx11**

**8-bit samples** have a flag of    **0bxxxx_xx01**

**Under investigation**: compressed samples have a flag of **0101_0001**

confirmed it that's not the case