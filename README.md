<p align="center">
    <img alt="XMODITS Logo" src="icon.png">
</p>

# XMODITS - A fast & lightweight tool to rip samples from tracker music.
[![unit_testing](https://github.com/B0ney/xmodits/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/B0ney/xmodits/actions/workflows/rust.yml)


## Supported Formats (upcoming)
| Extension | Format |
| --- | --- |
| IT | Impulse Tracker |
| XM | Extended Module | 
| S3M | Scream Tracker 3 |
| MOD | Amiga Pro Tracker |
| MPTM | ModPlug Tracker module |
| UMX | Unreal Music Package (Containing above) |

## Screenshots (GUI)
![xmodits gui](./extras/screenshots/Screenshot_1.png)
![xmodits gui](./extras/screenshots/Screenshot_2.png)

<!-- ## Download
You can download builds for xmodits [here](https://github.com/B0ney/xmodits/releases).

If you wish to build from source, go to [building](#building). -->

## Other projects:
* xmodits cli application
* xmodits python library

## How to Use

### Windows: 
You simply drag and drop a module(s) onto the binary.

If successful, a folder with a similar name to the module should appear.

NOTE: xmodits will place the ripped samples wherever the program is located.

### Linux:

```
xmodits <module path> [destination folder]

e.g:
    xmodits test.s3m ~/Music/Samples/
```
### The destination folder is optional, you can just do this:
```
xmodits test.s3m
```

xmodits will place the ripped samples wherever the program is located.


### Rip from multiple trackers:
```
xmodits mod1.mod mod2.it mod3.s3m [destination folder]
```
### Rip from multiple trackers via "*" :
```
xmodits ./*.mod [destination folder]
```
<!-- ### Advanced Arguments
If you want to customize how ripped samples are named, the following arguments can be used:

|short| long| Description|
|--|--|--|
|-i |--index-only| Ripped samples will only be named with an index|
|-r |--index-raw| Preserve internal sample indexing|
|-p |--index-padding| Pad sample index with preceding 0s|
|-n |--no-folder| Do not create a new folder for ripped samples |
|-u |--upper| Name samples in UPPER CASE |
|-l |--lower| Name samples in lower case |
|-c| --with-comment |Include embedded text from tracker (if it exists) 🚧|
|-k |--parallel| Rip samples in parallel (Requires compiling with **``features="advanced"``**) | -->

<!-- ## Note
The purpose of this tool (the core) is to dump samples that's it.

You'll notice some dumped samples may not sound identical to what's heard in a tracker module. 

This is because the tracker authour has applied effects such as pitch increase, vibrato.

Replicating these effects is not a top priority.  -->

## Resources
The resources that made this project possible can be found [here](./resources/).

---
## Building
Requirements:
* Rust compiler: https://www.rust-lang.org/tools/install
* Minimum rust version 1.65


## Licenses (Upcoming)
The xmodits project has multiple programs. Each with their own licenses.

|Program| License|
|--|--|
|[XMODITS-GUI](app/gui/) (App) | GPLv3|
|[XMODITS-CLI](app/cli/) (App) | LGPLv3 |
|[XMODITS-PY](pylib/) (basically xmodits-cli)| LGPLv3 |
|XMODITS-LIB | MPLv2 |
