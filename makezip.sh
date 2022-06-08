#!/usr/bin/env bash
ZIPFILE=metrix-cli.zip

echo "Building macos release"
cargo build --release
echo "Done"

echo "Building windows release"
cargo build --target x86_64-pc-windows-gnu --release
echo "Done"

echo "Delete old zip file if exists"
rm -f -- "$ZIPFILE"
echo "Done"

echo "Creating distribution zip"
zip "$ZIPFILE" target/x86_64-pc-windows-gnu/release/metrix-cli.exe target/release/metrix-cli docs/MAC-running.txt docs/WIN-running.txt
echo "Done"
