<p align="center">
    <img alt="XMODITS Logo" src="icon.png">
</p>

# XMODITS - A fast & lightweight tool to rip samples from tracker music.
[![unit_testing](https://github.com/B0ney/xmodits/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/B0ney/xmodits/actions/workflows/rust.yml)

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

## Download
You can download builds for xmodits [here](https://github.com/B0ney/xmodits/releases).

If you wish to build from source, go to [building](#building).

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
