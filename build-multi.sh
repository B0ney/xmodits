rustup override set 1.60
cargo +nightly build --features="ascii_art" -p xmodits -Zmultitarget --target=x86_64-unknown-linux-gnu --target=x86_64-pc-windows-gnu --release