# Impulse Tracker Compression (IT214 / IT215)

Sources: 
* https://github.com/nicolasgramlich/AndEngineMODPlayerExtension/blob/master/jni/loaders/itsex.c
* https://github.com/Konstanty/libmodplug/blob/master/src/load_it.cpp#L1183
* https://wiki.multimedia.cx/index.php/Impulse_Tracker#IT214_sample_compression
* https://github.com/schismtracker/schismtracker/blob/master/fmt/compression.c


Note:
Unless the compat version is 0215, assume all compressed samples to use IT214 compression.


# (In progress) Algorithm explanation

Compressed samples are split up into blocks

Each block is laid out as follows:
(Offsets are relative)

```
0x0000 => [u16] block size, aka length field
0x0002 => compressed data
0x0002
    + BLOCK_SIZE => next block
```

## Reading bits 
It doesn't matter how this algorithm is implemented, so long as the output satisfies the order:
```
bit reading order:

[0101_1100, 1011_0111, ...]

0101_1100, 1011_0111,   ....._.....
|       |  |       |    |         |
\    <--|  \    <--|    \      <--|
        |          |-------|      |-(3) Contiune until desired bits are met
        |                  |
        |-(1) START HERE   |
          (Right -> Left)  |
                           |-(2) When it reaches MSB (Most Significant Bit)
                              Move to next byte. Continue reading until it hits MSB.
 
If I want to read 12 bits, the result would be:

0111__0101_1100
   |  |_______|
   |          |-- From first Byte
   |--------|
            |---- From second Byte

You'd pad the Left Most bits like so:

0000_0111__0101_1100
```

## Decompressing 8/16-bit samples
**How does it all work?**

Fundamentally, samples are **compressed** through differentiation.

In essense, samples are stored as differences from each other.

Take this for example:
```
[1, 7, 10, 13, 8, 4, 2]
```
To differentiate, the next value is the difference between the current and the previous value.
```
1  - 0  = 1
7  - 1  = 6
10 - 7  = 3
13 - 10 = 3
...
```
```
[1, 6, 3, 3, -5, -4, -2]
```
Note: We assume the value before the first value to be 0.

The idea is that the values become less spread out through differentiation.

As these values are now much closer to eachother, we can use less bits to represent them. 

We can go further and encode each value with a varied bit width.


decompressing is the reverse.


## Algorithm
```
Set the bit width to 9 [17]

Read 9 bits from first block



# method: 1 { if bit width < 7 }


# method: 2 { if bit width < 9 }


# method: 3 { if bit width == 9 }

Comprare the highest bit:
    if 0, the last 8 bits are integrated and stored 
    if 1, the last 8 bits will be the new bit width




```