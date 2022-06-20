https://isip.piconepress.com/projects/speech/software/tutorials/production/fundamentals/v1.0/section_02/s02_01_p05.html

```
0x0000 -> "RIFF"                    [u32]
0x0004 -> Size of File*             [u32]
0x0008 -> "WAVE"                    [u32]
0x000C -> "fmt "                    [u32]
0x0010 -> WAV sec chunk size**      [u32]
0x0014 -> WAV type***               [u16] 
0x0016 -> mono/stereo flag****      [u16]
0x0018 -> sample frequency          [u32]
0x001C -> bytes/sec                 [u32]
0x0020 -> block align               [u16]
0x0022 -> bits per sample           [u16]
0x0024 -> "data"                    [u32]
0x0028 -> size of data chunk*****   [u32]
0x002C -> audio data
```
*The file size - 8 bytes

** The size of the WAV type format (2 bytes) + mono/stereo flag (2 bytes) + sample rate (4 bytes) + bytes/sec (4 bytes) + block alignment (2 bytes) + bits/sample (2 bytes). This is usually 16. 

*** Type of WAV format. This is a PCM header, or a value of 0x01. 

**** mono (0x01) or stereo (0x02) 

***** length of byte array * (bits_per_sample / 8)
