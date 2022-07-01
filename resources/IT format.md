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

## IT Header Structure
Size: **192 Bytes**

Offsets are in **bytes** and they are relative to the **"IMPM"** header.

This should be the first thing you see when you load an **.it**

Reference: https://github.com/schismtracker/schismtracker/blob/master/include/it_defs.h#L7=

```
0x0000 -> "IMPM"        [u32]       [need this for verification purposes]
0x0004 -> songname      [char; 26]  [might be needed]

0x001E -> hilight_minor [u8]
0x001F -> hilight_major [u8]
0x0020 -> ordnum        [u16]       [need this] (number of orders)
0x0022 -> insnum        [u16]       [need this] (number of instruments)
0x0024 -> smpnum        [u16]       [need this] (number of samples)

0x0026 -> patnum        [u16]
0x0028 -> cwtv          [u16]       created with tracker version
0x002A -> cmwt          [u16]       [need this] compatible with tracker version. (Assume 0214 if 0215 not present)

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
0x00c0 -> orders        [u8; ORDNUM]

0x00c0
    + ORDNUM -> [u32; INSNUM] list of instrument offsets

0x00c0
    + ORDNUM
    + (INSNUM * 4) -> [u32; SMPNUM] list of sample header offsets [needed]

0x00c0  
    + ORDNUM
    + (INSNUM * 4)
    + (SMPNUM * 4) -> [u32; PATNUM] list of pattern offsets

0x00c0 
    + ORDNUM
    + (INSNUM * 4)
    + (SMPNUM * 4)
    + (PATNUM * 4) -> END OF HEADER DATA, NEW DATA STARTS FROM HERE. 

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
