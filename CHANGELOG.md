# v0.12.0 (planned, subject to change)
## What's new
* Added option to filter files based on the following properties:
  * file size
  * file name
  * file extension
  * TODO: date-(created, modified)
* Added button to preview samples.
* Can import/export custom themes.
* Shows the number of errors while ripping.
* Added keyboard shortcuts:
  |Shortcut| Action|
  |-|-|
  |<kbd>delete</kbd>| Clears the selected entries|
  |<kbd>shift</kbd> + <kbd>delete</kbd>| Clears the entries|
  |<kbd>ctrl</kbd>/<kbd>⌘</kbd> + <kbd>S</kbd>| Save Configuration|

## Bug Fixes
* (xmodits-lib) Fixed downsampling bug when exporting samples to ``.8svx``

## Other Improvements
* XMODITS now has a built-in software renderer.
* Improved displaying for errors.
* Improved memory usage for Linux binaries (uses jemalloc).
* MacOS builds now include binaries for Apple Silicon.

# v0.11.0 (29 July 2023)

**Notice**: The command line version of XMODITS has moved to [here](https://github.com/B0ney/xmodits-cli)

## What's new
* You can now save samples to the following formats:
    *  Impulse Tracker Instrument - ``.its`` #33 
    * Scream Tracker 3 Instrument - ``.s3i`` #33 
* Added section to preview how samples will be named.
* Added button to invert selection.

## Bug Fixes
* Fixed clicking in right audio channel when ripping xm modules with stereo samples.

## Other Improvements
* XMODITS will no longer overwrite files with conflicting filenames. It will error instead.
* Prefixing samples with the module's filename now includes its file extension.

# v0.10.0 (22 May 2023)

## What's New:
* You can now extract samples to the following formats:
  * ``AIFF``
  * ``8SVX`` #27 
  * ``RAW``
* Extracted samples can be prefixed with the module's file name:
  e.g ``music.s3m`` -> ``music - 01 - kick.wav``
* Extracted samples can be named with its internal filename if it exists.
* Extracted samples will now have loop points embedded if the format supports it.
* Added support for stereo samples for ``Impulse Tracker`` and ``Extended Modules``.

## What's new (GUI Only):
* Added new themes because why not ([preview](https://github.com/B0ney/xmodits/tree/v0.10.0-rc1/screenshots)):
  * Dracula
  * Catppuccin
  * Nord
  * LMMS
  * OneShot
* Ripping from folders is faster as it now uses multi-threading.
* xmodits will now rip from entries that have been checked. If none or all have been checked, it will rip all of them.
* You can now manually export errors.
* (**Windows only**) launching the application from the terminal will show logs.
* It is now possible to cancel ripping.

## What's new (CLI Only):
* Included features from the GUI application.
* Added a ``no-exit-prompt`` flag which disables "Press Enter to Continue".

## Improvements and Bug Fixes:
* Samples extracted from ``MOD`` files no longer sound low pitched. #24
* Fixed a bug where ``MOD`` files that don't have 4 channels will produce misaligned samples.
* IFF MOD files will now produce  helpful errors.
* Support for partial extraction; xmodits will no longer terminate the ripping process if it fails to extract a sample.
* Improved memory usage when ripping from nested folders.
* Improved memory usage when handling errors.
* xmodits will suggest you use folders if there are a lot of files added.

**Full Changelog**: https://github.com/B0ney/xmodits/compare/0.9.8...v0.10.0

# v0.9.8 (23 December 2022)

## New
* New supported formats: **umx** and **mptm**
* New GUI and CLI applications.
* You can now customise how extracted samples are named.

## Improved
* Better format detection.
* Several optimisations to reduce memory.

# v0.9.5 (23 July 2022)

* Exported samples now have the correct WAV header.
* S3M: Exported 16-bit samples are now correct.
* MOD: Exported samples now have the correct sample frequency rather than the fixed **8363**Hz.
* MOD: Fixed exported samples not being properly aligned due to an incrorrectly calculated offset.
* IT: Ripping compressed samples should be faster.
* IT: Detect and provide useful error for ziRCON compressed modules. 
* XM: Fixed overflow panic. 

# 0.9.2 (19 July 2022)
Ripping from XM modules should be close to stable now

# 0.8.8 (15 July 2022)
First Release