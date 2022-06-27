# Tracker Dumper

**XMODITS**

A tool to dump samples from popular tracker formats.

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
```

```

## Note
The purpose of this tool (the core) is to dump samples that's it.

You'll notice some dumped samples may not sound identical to what's heard in a tracker module. 

This is because the tracker authour has increased the pitch to their liking.

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
