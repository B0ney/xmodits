# Dumping samples from Impulse Tracker Modules

We're only interested in dumping the samples, so it should be decent.

IT Samples can be stored as:
```
* 8-bit     raw PCM         (mono/stereo)
* 8-bit     compressed      (mono/stereo)
* 16-bit    raw PCM         (mono/stereo)
* 16-bit    compressed      (mono/stereo)
```
Refer to "IT compression.md" for more infomation.

### Sources:

[ITTECH.TXT ARCHIVE 1](https://web.archive.org/web/20220610182703/https://github.com/schismtracker/schismtracker/wiki/ITTECH.TXT)

[ITTECH.TXT ARCHIVE 2](https://ia600506.us.archive.org/view_archive.php?archive=/4/items/msdos_it214c_shareware/it214c.zip&file=ITTECH.TXT)


### Terminology:
|term| meaning|
|---|---|
|**iN**|**Signed** **N**-bit integer ( e.g **i8** )|
|**uN**|**Unsigned** **N**-bit integer ( e.g **u8** )|
| [**iN**; **S**]| **Signed** **N**-bit **array with size S**  (e.g **[i8; 64]**)|
| [**UN**; **S**]| **Unsigned** **N**-bit **array with size S**  (e.g **[u8; 64]**)|


### IT file structure:
* Header (**192 bytes**)
* Pattern orders with bytes indicated by **ordnum**
* Instrument parameters with bytes **4x** size of **insnum**
* Sample parameters with bytes **4x** size of **smpnum**
* Pattern parameters with bytes **4x** value of **patnum**
* **Undocumented data**
* Sample metadata, each **80 bytes**. Amount specified by **smpnum**

Note: I don't think it's worth documenting out every part of the file, why not just look where the first **"IMPS"** is located?

## IT Header Structure
Size: **192 Bytes** 

Offsets are in **bytes** and they are relative to the **"IMPM"** header.

This should be the first thing you see when you load an **.it**

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

0x0028 -> cwtv          [u16]       created with tracker version
0x002A -> cmwt          [u16]       [need this] compatible with tracker version, i.e "214/215"

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

0x00c0 -> END OF HEADER DATA, NEW DATA STARTS AFTER HERE. 

```





## IT Sample Byte Structure
Size: **80 Bytes**

They do not store samples but rather metadata about them. 

They'll point to the raw samples. 

The sample pointer needs to be read as Little endian


Offsets are in **bytes** and they are relative to the **"IMPS"** header

Reference: https://github.com/schismtracker/schismtracker/blob/master/include/it_defs.h#L102=

```
0x0000 -> "IMPS"        [u32]       [We need this for verification]
0x0004 -> filename      [i8; 12]    [We need this]

0x0010 -> zero          [u8]
0x0011 -> gvl           [u8]        
0x0012 -> flags         [u8]        [We need this] **

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

0x0050 -> END OF INSTRUMENT STRUCT, NEW STRUCT STARTS AFTER HERE.
```

### **Flags (0x0012):


**16-bit samples** have a flag of   **0bxxxx_xx1x**

**8-bit samples** have a flag of    **0bxxxx_xx0x**






## IT Instrument Byte structure
Size: **554** bytes

Not needed for dumping samples. 

again, offsets are relative.
```
0x0000 => "IMPI"            [u32]
0x0004 => zero              [u8]
0x0010 => nna               [u8]
0x0011 => dct               [u8]
0x0012 => dca               [u8]
0x0013 => fadeout           [u16]
0x0014 => pps               [char]
0x0016 => ppc               [u8]
0x0017 => gbv               [u8]
0x0018 => dfp               [u8]
0x0019 => rv                [u8]
0x001A => rp                [u8]
0x001B => trkvers           [u16]
0x001C => nos               [u8]
0x001E => reserved1         [u8]
0x001F => name              [u8; 26]
0x0020 => ifc               [u8]
0x003A => ifr               [u8]
0x003B => mch               [u8]
0x003C => mbr               [u8]
0x003D => keyboard          [u8; 240]
0x003E => vol envelope      ( Struct DATA ){ 82 bytes }
0x0040 => pan envelope      ( Struct DATA ){ 82 bytes }
0x0130 => Pitch envelope    ( Struct DATA ){ 82 bytes }
0x0226 => DUMMY DATA        [u8]  FOR COMPATABILITY

0x22a => END OF INSTRUMENT STRUCT, NEW DATA STARTS AFTER HERE
```
