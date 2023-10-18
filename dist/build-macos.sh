#!/bin/bash

PLATFORM="macos"

TARGET_OLD="xmodits-gui"
TARGET="xmodits"

FEATURES="build_info","wgpu"

PROFILE="release"
RELEASE_DIR="target/$PROFILE"

BINARY_OLD="$RELEASE_DIR/$TARGET_OLD"

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
cp  README.md $ARCHIVE_DIR
cp  LICENSE $ARCHIVE_DIR

chmod +x $BINARY

ARCHIVE_NAME="$TARGET-v$($BINARY --version)-$PLATFORM.zip"
ARCHIVE_PATH="$ARTIFACT_DIR/$ARCHIVE_NAME"

zip -j $ARCHIVE_PATH $ARCHIVE_DIR/*
