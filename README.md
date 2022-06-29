# Tracker Dumper

**XMODITS**

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
| MOD | ✓| N/A| N/A| N/A|
| UMX | -| -| -| -|

### UMX
| IT | S3M | MOD | XM | 
| ---| --- | --- | ---| 
| 🚧 |🚧  | 🚧  | 🚧 |


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

## What's next?
My next plan is to implement more advanced features for the gui.

### Planned GUI features
* Live sample playback
* Super basic sample editor to trim & pitch samples
