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


# create directories
mkdir -p $ARCHIVE_DIR
rm -rf $ARCHIVE_DIR/*

mkdir -p $ARTIFACT_DIR
rm -rf $ARTIFACT_DIR/*


# Build universal binary and store it to the archive directory
export MACOSX_DEPLOYMENT_TARGET="11.0"
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
cargo build -p xmodits-gui --release --target=x86_64-apple-darwin --features=$FEATURES
cargo build -p xmodits-gui --release --target=aarch64-apple-darwin --features=$FEATURES

lipo "target/x86_64-apple-darwin/release/$TARGET_OLD" "target/aarch64-apple-darwin/release/$TARGET_OLD" -create -output "$BINARY"
echo "Created universal binary"

# copy extra files
cp  ./assets/manual.txt $ARCHIVE_DIR
cp  LICENSE $ARCHIVE_DIR

chmod +x $BINARY

ARCHIVE_NAME="xmodits-gui-v$($BINARY --version)-$PLATFORM-universal.zip"
ARCHIVE_PATH="$ARTIFACT_DIR/$ARCHIVE_NAME"

zip -j $ARCHIVE_PATH $ARCHIVE_DIR/*
