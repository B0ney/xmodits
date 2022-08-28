# XMODITS python library

## development:
https://docs.python.org/3/library/venv.html

Create a python virtual environment in this directory:

```python -m venv dev```

activate virtual environment:

```source <venv>/bin/activate```

install [maturin (crates.io)](https://crates.io/crates/maturin) or from [pipi](https://pypi.org/project/maturin/)

run test library:

```maturin develop```


# API (In progress)

```python
from xmodits import dump

# dump samples to "samples/music_xm/" 
dump("music.xm", "samples/")

# dump samples to "samples/" folder 
dump("music.xm", "samples/", no_folder=True)
```

Samples are, by default named in this format:
```
00 - SAMPLE_NAME.wav

e.g.
    08 - trumpet.wav
    12 - vocal.wav

```
If a sample doesn't have a name, the naming format will be:
```
00.wav

e.g
    19.wav
```
# Required Arguments
|Argument| Meaning|
| --- | --- |
| ```Path``` | Path to a tracker module |
| Destination | Destination folder for dump* |

\* By default, xmodits will create a new folder


# Additional Arguments

|Argument| Meaning|
| --- | --- |
| ```no_folder``` | Do not create a new folder for ripped samples.<br> **e.g.** When set to ```True```, ```"drums.it"``` will NOT create ```"drums_it"``` in the destination folder.<br> **All samples will be placed in the destination folder instead.**  |
| ```no_padding``` | Do not pad sample number.<br > **e.g.** ```"01 - kick.wav"``` --> ```"1 - kick.wav"``` |
| ```number_only``` | Only name samples with a number.<br> **e.g.** ```"09 - HiHat.wav"``` --> ```"09.wav"``` |
| ```preserve_sample_number``` | Sample number will be identical  |



# Exceptions
They're pretty much self explanitory.

|Exception| Meaning|
| --- | --- |
|```SampleExtractionError```| xmodits could not rip a sample.|
| ```UnsupportedFormatError```  | |
| ```InvalidModuleError``` | |
| ```EmptyModuleError``` | The tracker module is valid but it has no samples! |

Generic