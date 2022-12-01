# XMODITS python library

## development:
https://docs.python.org/3/library/venv.html

Create a python virtual environment in this directory:

```python -m venv dev```

activate virtual environment:

```source ./dev/bin/activate```

install [maturin (crates.io)](https://crates.io/crates/maturin) or from [pipi](https://pypi.org/project/maturin/)

run test library:

```maturin develop```


# API (In progress)

### Dump samples to a folder

```python
import xmodits

# dump samples to "samples/" 
tracker = "mods/music.xm"
folder  = "samples/"

xmodits.dump(tracker, folder)
```
This produces the following output in folder **"samples"**:

```
01 - hihat.wav
02 - kick.wav
03 - snare.wav
04 - toms.wav
...
15 - vocal.wav
```

### Dump samples without names

```python
import xmodits

tracker = "mods/music.xm"
folder  = "samples/"

xmodits.dump(
    tracker,
    folder,

    index_only=True 
)
```
This produces the following output in folder **"samples"**:

```
01.wav
02.wav
03.wav
04.wav
...
15 - vocal.wav
```
### Dump samples without padding the index:

```python
import xmodits

tracker = "mods/music.xm"
folder  = "samples/"

xmodits.dump(
    tracker,
    folder,

    index_padding=0         # or 1, both have the same effect
)
```
Output:
```
1 - hihat.wav
2 - kick.wav
3 - snare.wav
4 - toms.wav
...
15 - vocal.wav
```

Samples stored in tracker modules can have an arbitary index. If you prefer to use this index, include the parameter:

```python
index_raw=True
```

If you're dumping from multiple modulse to the same folder, you're guaranteed to have collisions. 

You should include the parameter:

```Python
with_folder=True
```

It will produce a new folder within the destination folder

# Required Arguments
|Argument| Meaning|
| --- | --- |
| ```Path``` | Path to a tracker module |
| ```Destination``` | Destination folder for dumped samples |

# Additional Arguments

|Argument| Meaning|
| --- | --- |
| ```with_folder``` | Create a new folder for ripped samples.<br> **e.g.** When set to ```True```, ```"drums.it"``` will create ```"drums_it"``` in the destination folder and place those samples there. |
| ```index_padding``` | Set padding.<br > **e.g.** ```"01 - kick.wav"``` --> ```"1 - kick.wav"``` |
| ```index_only``` | Only name samples with a number.<br> **e.g.** ```"09 - HiHat.wav"``` --> ```"09.wav"``` |
| ```index_raw``` | Sample number will be identical  |



# Exceptions
They're pretty much self explanitory.

|Exception| Meaning|
| --- | --- |
|```SampleExtractionError```| xmodits could not rip a sample.|
| ```UnsupportedFormatError```  | |
| ```InvalidModuleError``` | |
| ```EmptyModuleError``` | The tracker module is valid but it has no samples! |