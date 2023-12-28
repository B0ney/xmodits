#!/bin/bash

ARCH="x86_64"
PLATFORM="linux"

TARGET="xmodits-gui"
FEATURES="jemallocator","built","audio","iced_gif"

PROFILE="release"
RELEASE_DIR="target/$PROFILE"
BINARY="$RELEASE_DIR/$TARGET"

ARCHIVE_DIR="$RELEASE_DIR/archive"
ARTIFACT_DIR="$RELEASE_DIR/artifact"

install_deps() {
    sudo apt-get install -y libasound2-dev pkg-config
    # sudo apt-get install -y libxkbcommon-dev
}

build() {
    cargo build -p xmodits-gui --profile $PROFILE --features=$FEATURES
}

copy_binary() {
    install -Dm755 $BINARY $ARCHIVE_DIR
}

copy_extras() {
    install -D README.md LICENSE -t $ARCHIVE_DIR
}

create_dirs() {
    mkdir -p $ARCHIVE_DIR
    rm -rf $ARCHIVE_DIR/*

    mkdir -p $ARTIFACT_DIR
    rm -rf $ARTIFACT_DIR/*
}

package() {
    install_deps
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

package