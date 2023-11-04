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

APP_TEMPLATE="dist/macos/xmodits.app"
APP_DIR="$RELEASE_DIR/xmodits.app"
APP_PLIST="$APP_DIR/Contents/Info.plist"
APP_BINARY_DIR="$RELEASE_DIR/xmodits.app/Contents/Macos"

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

VERSION="$($BINARY --version)"
BUILD="v$VERSION-$(git rev-parse --short=8 HEAD)"

# remove app binary folder, copy template
rm -rf $APP_DIR/* &> /dev/null
cp -r $APP_TEMPLATE $RELEASE_DIR

# update version & build from template
sed -i '' "s/{{ VERSION }}/$VERSION/g" "$APP_PLIST"
sed -i '' "s/{{ BUILD }}/$BUILD/g" "$APP_PLIST"

chmod +x $BINARY
mkdir -p $APP_BINARY_DIR
mv $BINARY $APP_BINARY_DIR

# copy .app folder
cp -r $APP_DIR $ARCHIVE_DIR
# copy extra files
cp  ./assets/manual.txt $ARCHIVE_DIR
cp  LICENSE $ARCHIVE_DIR

ls $ARCHIVE_DIR


ARCHIVE_NAME="xmodits-gui-v$VERSION-$PLATFORM-universal.zip"
ARCHIVE_PATH="$ARTIFACT_DIR/$ARCHIVE_NAME"

zip -r $ARCHIVE_PATH $ARCHIVE_DIR/*
