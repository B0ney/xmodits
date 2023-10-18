#!/bin/bash

PLATFORM="linux"

TARGET_OLD="xmodits-gui"
TARGET="xmodits"

FEATURES="build_info","wgpu"

PROFILE="release"
RELEASE_DIR="target/$PROFILE"

BINARY_OLD="$RELEASE_DIR/$TARGET_OLD"


ARCHIVE_DIR="$RELEASE_DIR/archive"
ARTIFACT_DIR="$RELEASE_DIR/artifact"


cargo build -p xmodits-gui --profile $PROFILE --features=$FEATURES

# create directories
mkdir -p $ARCHIVE_DIR
mkdir -p $ARTIFACT_DIR

# copy and rename xmodits-gui to xmodits in archive folder
cp $BINARY_OLD $ARCHIVE_DIR/$TARGET

# copy extra files
cp  README.md $ARCHIVE_DIR
cp  LICENSE $ARCHIVE_DIR

chmod +x $ARCHIVE_DIR/$TARGET

ARCHIVE_NAME="$TARGET-v$($BINARY_OLD --version)-$PLATFORM-$ARCH.zip"
ARCHIVE_PATH="$ARTIFACT_DIR/$ARCHIVE_NAME"

zip -j $ARCHIVE_PATH $ARCHIVE_DIR/*
