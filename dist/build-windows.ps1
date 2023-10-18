
$ARCH="x86_64"
$PLATFORM="windows"

$FEATURES="build_info"

$ARCHIVE_DIR="$RELEASE_DIR/archive"
$ARTIFACT_DIR="$RELEASE_DIR/artifact"


# Compile application with features
cargo build -p xmodits-gui --release --features=$FEATURES

# Create artifact folder, remove residual files
mkdir -Force -p "target/release/artifact"
rm "target/release/artifact/*"

# Create archive folder, remove residual files
mkdir -Force -p "target/release/archive"
rm "target/release/archive/*"

# move and rename xmodits-gui.exe to xmodits in archive folder
mv -Force "target/release/xmodits-gui.exe" "target/release/archive/xmodits.exe"

# Copy extras to archive folder
cp -Force "README.md" "target/release/archive/"
cp -Force "LICENSE" "target/release/archive/"

# Get version number
$VERSION = cargo run -p xmodits-gui --release -- --version | Out-String
$VERSION = $VERSION.Trim() # trim whitespace, includes \n

# Compress archive contents of archive folder to a zip file
$ARTIFACT_PATH = "target/release/artifact/xmodits-gui-v$VERSION-$PLATFORM-$ARCH.zip"
Compress-Archive -Force -Path "target/release/archive/*" -DestinationPath $ARTIFACT_PATH 

echo "Packaged archive: $ARTIFACT_PATH"
