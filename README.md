<div align="center">

<img alt="XMODITS Logo" src="icon.png"> 
<!-- I could do with an improved logo tbh -->

# XMODITS - A fast & lightweight tool to extract samples from tracker music.
<!-- [![unit_testing](https://github.com/B0ney/xmodits/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/B0ney/xmodits/actions/workflows/rust.yml) -->
</div>

## Supported Formats
| Extension | Format | 
| --- | --- |
| IT | Impulse Tracker* |
| XM | Extended Module | 
| S3M | Scream Tracker 3 |
| MOD | Amiga Pro Tracker |
| MPTM | ModPlug Tracker module* |
| UMX | Unreal Music Package (Containing above) |

\* Ripping from OpenMPT trackers is not pefect [TODO]

## Features
<!-- * Sample previewing  (0.12.0)-->
* View information about a tracker
* Multi-threaded ripping*
* 
<!-- * Resuming -->
<!-- * History -->

\* xmodits will only use threads if it's ripping from a directory.

## Screenshots (GUI)
![xmodits gui](./screenshots/home.png)
![xmodits gui](./screenshots/selection.png)
![xmodits gui](./screenshots/ripping.png)

Click [here](./screenshots/README.md) for different themes

<!-- ![xmodits gui](./extras/screenshots/Screenshot_2.png) -->

## CLI Screenshot
--Soon--

## Download
You can download builds for xmodits [here](https://github.com/B0ney/xmodits/releases).

If you wish to build from source, go to [building](#building).

<!-- ## Other projects:
* xmodits cli application
* xmodits python library -->

## How to Use (GUI)
**Windows Only**

If you want to simply extract samples, you can just drag and drop a module(s) onto the binary. Xmodits will (by default) place the samples in a self contained folder in your ```~/Downloads``` folder.

**All Platforms**

### Sample Naming
<!-- Configures how ripped samples are named -->

|||
|--|--|
| Index Only | Samples will only be named with an index |
| Preserve Index | Sample index will match how it's represented internally. |
| Upper Case | Samples will be named in upper case.|
| Lower Case | Samples will be named in lower case.|
| Index Padding | Pad index with preceding zeros.<br>Set to 1 to remove padding.|


### Ripping Configuration

|||
|--|--|
| No Folder | Xmodits will not put samples in a self contained folder.<br>This can overwrite data so use with caution.|
| Strict Loading||
| Export Format| |
| Folder Scan Depth | Limit how far a folder can be traversed. |

### Saving Configuration
Any changes made to the configuration must be saved manually.<br>The configuration file can be located at:

windows:
```%appdata%\xmodits\```

Linux:
```~/.config/xmodits/```



## How to Use (CLI version)
Note: On Windows, the CLI binary has been renamed to "xmodits-cli.exe"

```
xmodits <module path> [destination folder]

e.g:
    xmodits ./test.s3m ~/Music/Samples/

e.g: 
    xmodits ./test.s3m

e.g:
    xmodits ./mod1.mod ./mod2.it ./mod3.s3m [destination folder]

e.g: (linux only)
    xmodits ./*.mod ~/Downloads
```
If the destination is not provided, xmodits will place the ripped samples in a self contained folder in the current working directory.

### Additional Arguments
If you want to customize how ripped samples are named, the following arguments can be used:

|short| long| Description|
|--|--|--|
|-i |--index-only| Ripped samples will only be named with an index.|
|-r |--index-raw| Preserve internal sample indexing.|
|-p |--index-padding| Pad sample index with preceding zeros. 0-1 will disable padding.|
|-n |--no-folder| Do not create a new folder for ripped samples.<br>This can overwrite data, BE CAREFUL!|
|-u |--upper| Name samples in upper case. |
|-l |--lower| Name samples in lower case. |
||--info| Print information about a tracker module. |


<!-- ## Note
The purpose of this tool (the core) is to dump samples that's it.

You'll notice some dumped samples may not sound identical to what's heard in a tracker module. 

This is because the tracker authour has applied effects such as pitch increase, vibrato.

Replicating these effects is not a top priority.  -->

<!-- ## Resources
The resources that made this project possible can be found [here](./resources/). -->

## Building
Requirements:
* Rust compiler: https://www.rust-lang.org/tools/install
* Minimum rust version: 1.65

### Building the GUI
```
cargo build -p xmodits-gui --release
```
### Building the CLI
```
cargo build -p xmodits --release
```

## Licenses
The xmodits project has multiple programs. Each with their own licenses.

|Program| License|Description|
|--|--|--|
|[XMODITS-GUI](https://github.com/B0ney/xmodits) (App) | GPLv3| xmodits gui app|
|[XMODITS-CLI](https://github.com/B0ney/xmodits) (App) | LGPLv3 | xmodits cli program|
|[XMODITS-PY](https://github.com/B0ney/xmodits-lib)| LGPLv3 | xmodits Python library <br> (Mainly used for [DawVert](https://github.com/SatyrDiamond/DawVert))|
|[XMODITS-LIB](https://github.com/B0ney/xmodits-lib) | MPLv2 | xmodits core library|

## Special Thanks
- The GUI was made with [Iced](https://github.com/iced-rs/iced)
- [@0x192](https://github.com/0x192) (and contributers) for their [Universal Android Debloat tool](https://github.com/0x192/universal-android-debloater/). I've learned a lot of gui stuff from that project.
- [SatyrDiamond](https://github.com/SatyrDiamond)'s [DawVert](https://github.com/SatyrDiamond/DawVert), A program to convert different daw project files to [TODO]