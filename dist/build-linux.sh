#!/bin/bash

ARCH="x86_64"
PLATFORM="linux"

TARGET="xmodits-gui"
FEATURES="jemalloc","build_info"

PROFILE="release"
RELEASE_DIR="target/$PROFILE"
BINARY="$RELEASE_DIR/$TARGET"

ARCHIVE_DIR="$RELEASE_DIR/archive"
ARTIFACT_DIR="$RELEASE_DIR/artifact"


build() {
    cargo build -p xmodits-gui --profile $PROFILE --features=$FEATURES
}

copy_binary() {
    install -Dm755 $BINARY $ARCHIVE_DIR
}

copy_extras() {
    install -D README.md LICENSE -t $ARCHIVE_DIR
}

artifact_path() {
    echo $ARTIFACT_DIR
}

create_dirs() {
    mkdir -p $ARCHIVE_DIR
    rm -rf $ARCHIVE_DIR/*

    mkdir -p $ARTIFACT_DIR
    rm -rf $ARTIFACT_DIR/*
}

package() {
    build

    create_dirs

    copy_binary
    copy_extras

    ARCHIVE_NAME="$TARGET-v$($BINARY --version)-$PLATFORM-$ARCH.tar.gz"
    ARCHIVE_PATH="$ARTIFACT_DIR/$ARCHIVE_NAME"

    # Compress archive directory
    tar czvf $ARCHIVE_PATH -C $ARCHIVE_DIR .

    echo "Packaged archive: $ARCHIVE_PATH"
}

case "$1" in
  "package") package;;
  "artifact_path") artifact_path;;
  *)
    echo "available commands: package, artifact_path"
    ;;
esac