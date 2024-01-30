#!/bin/bash

PLATFORM="macos"

TARGET_OLD="xmodits-gui"
TARGET="xmodits"

FEATURES="built","wgpu","audio","iced_gif","manual"

PROFILE="release"
RELEASE_DIR="target/$PROFILE"

ARCHIVE_DIR="$RELEASE_DIR/archive"
ARTIFACT_DIR="$RELEASE_DIR/artifact"
BINARY="$ARCHIVE_DIR/$TARGET"

case "$1" in
    "silicon") BINARY_TYPE="apple-silicon";;
    *) BINARY_TYPE="intel"
esac

# create directories
mkdir -p $ARCHIVE_DIR
rm -rf $ARCHIVE_DIR/*

mkdir -p $ARTIFACT_DIR
rm -rf $ARTIFACT_DIR/*

export MACOSX_DEPLOYMENT_TARGET="11.0"

cargo build -p xmodits-gui --release --features=$FEATURES

mv "target/release/$TARGET_OLD" "$BINARY"
echo "Created MacOS binary"

# copy extra files
cp  ./assets/manual.txt $ARCHIVE_DIR
cp  LICENSE $ARCHIVE_DIR

chmod +x $BINARY

ARCHIVE_NAME="xmodits-gui-v$($BINARY --version)-$PLATFORM-$BINARY_TYPE.zip"
ARCHIVE_PATH="$ARTIFACT_DIR/$ARCHIVE_NAME"

zip -j $ARCHIVE_PATH $ARCHIVE_DIR/*
