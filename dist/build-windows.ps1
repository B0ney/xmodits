
$ARCH="x86_64"
$PLATFORM="windows"

$FEATURES="built","audio","iced_gif","manual"

# Compile application with features
cargo build -p xmodits-gui --release --features=$FEATURES

# Create artifact folder, remove residual files
mkdir -Force -p "target/release/artifact"
Remove-Item "target/release/artifact/*"

# Create archive folder, remove residual files
mkdir -Force -p "target/release/archive"
Remove-Item "target/release/archive/*"

# move and rename xmodits-gui.exe to xmodits in archive folder
Move-Item -Force "target/release/xmodits-gui.exe" "target/release/archive/xmodits.exe"

# Copy extras to archive folder
Copy-Item -Force "README.md" "target/release/archive/"
Copy-Item -Force "LICENSE" "target/release/archive/"

# Get version number
$VERSION = cargo run -p xmodits-gui --release -- --version | Out-String
$VERSION = $VERSION.Trim() # trim whitespace, includes \n

# Compress archive contents of archive folder to a zip file
$ARTIFACT_PATH = "target/release/artifact/xmodits-gui-v$VERSION-$PLATFORM-$ARCH.zip"
Compress-Archive -Force -Path "target/release/archive/*" -DestinationPath $ARTIFACT_PATH 

Write-Output "Packaged archive: $ARTIFACT_PATH"
