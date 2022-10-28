<p align="center">
    <img alt="XMODITS Logo" src="icon.png">
</p>

# XMODITS - A fast & lightweight tool to rip samples from tracker music.
[![unit_testing](https://github.com/B0ney/xmodits/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/B0ney/xmodits/actions/workflows/rust.yml)


## Download
You can download builds for xmodits [here](https://github.com/B0ney/xmodits/releases).

If you wish to build from source, go to [building](#building).

## Supported Formats:

|Format| 8-bit| 16-bit| compression|Stereo|
| --- | --- | --- | --- | --- |
|IT| ✓|✓|✓|⏳|
| XM  | ✓| ✓| N/A| N/A|
| S3M | ✓| ✓| N/A| ⏳|
| MOD | ✓| N/A| N/A| N/A|

|Key| Meaning|
|---|---|
|**✓** | Fully supported with little or no bugs|
| **~** | Sorta works |
| **n/a** | Format doesn't support it|
| ⏳ | In progress|
| 🚧 | Part of roadmap |

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

## Note
The purpose of this tool (the core) is to dump samples that's it.

You'll notice some dumped samples may not sound identical to what's heard in a tracker module. 

This is because the tracker authour has applied effects such as pitch increase, vibrato.

Replicating these effects is not a top priority. 

## Resources
The resources that made this project possible can be found [here](./resources/).
## Goals
* Fully Support Listed formats.
* Easy to use.
* Simple codebase with very few dependencies. 
* Well documented (self documented code preferred)
* Hackable: Contributors can implement obscure tracker formats. 

---
## Building
Requirements: 
* Rust: https://www.rust-lang.org/learn/get-started

The easiest way to compile xmodits is through this command.

The binary size should be acceptable (<1MB)

```
cargo build -p xmodits --release
```

windows

```

cargo build --release -p xmodits --bin xmodits

```

## Licenses (Upcoming)
The xmodits project has multiple programs. Each with their own licenses.

|Program| License|
|--|--|
|[XMODITS-GUI](app/gui/) (App) | AGPLv3|
|[XMODITS-CLI](app/cli/) (App) | LGPLv3? |
|[XMODITS-PY](pylib/) (basically xmodits-cli)| LGPLv3? |
|XMODITS-LIB | MPL2? (Remove/replace *potential* use of gpl2 code in amig_mod.rs & tables.rs ) |
