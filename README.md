# **XMODITS**

<p align="center">
    <img alt="XMODITS Logo" src="icon.png">
</p>


<!-- ![logo](icon.png "xmodits-logo") -->

A **fast** & **lightweight** tool to dump samples from popular tracker formats with **ease**.

## Supported Formats:

|Key| Meaning|
|---|---|
|**✓** | Fully supported with little or no bugs.|
| **/** | Sorta works|
| **-** | Format is a container|
| **X** | unsupported |
| **n/a** | Format doesn't support it|
| ⏳ | In progress|
| 🚧 | Part of roadmap |

|Format| 8-bit| 16-bit| compression|Stereo|
| --- | --- | --- | --- | --- |
|IT| ✓|✓|✓|⏳|
| XM  | 🚧| 🚧| 🚧| 🚧|
| S3M | ✓| ✓| N/A| ⏳|
| MOD | /| N/A| N/A| N/A|
| UMX | -| -| -| -|

### UMX
| IT | S3M | MOD | XM | 
| ---| --- | --- | ---| 
| 🚧 |🚧  | 🚧  | 🚧 |

## Download
You can download builds for xmodits here.

If you wish to build from source, go to [Building](#building).

## How to Use
To dump samples from a module:
```
xmodits <module path>

e.g:
    xmodits test.s3m
```
To dump samples to a folder:
```
xmodits <module path> [dest folder]

e.g:
    xmodits test.s3m ~/B0ney/Downloads/Samples/
```

## Note
The purpose of this tool (the core) is to dump samples that's it.

You'll notice some dumped samples may not sound identical to what's heard in a tracker module. 

This is because the tracker authour has applied effects such as pitch increase, vibrato.

Replicating these effects is not a top priority. 

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

## Building (Advanced)
If you want to make the binary size smaller, you can do so here. (nightly version required)

If you have nightly installed, update it to minimize the chance of a build failure:
```
rustup update
```
If you don't have nightly installed:
```
rustup toolchain install nightly
```

To build on windows (MSVC):
```
cargo +nightly build -p xmodits -Z build-std=std,panic_abort --target=x86_64-pc-windows-msvc --release
```
To build on windows (GNU):
```
cargo +nightly build -p xmodits -Z build-std=std,panic_abort --target=x86_64-pc-windows-gnu --release
```


To build on Linux:
```
cargo +nightly build -p xmodits -Z build-std=std,panic_abort --target=x86_64-unknown-linux-gnu --release
```

---