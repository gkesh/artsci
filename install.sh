#!/bin/sh

# Creating the config folder
mkdir -p -- "~/.config/artsci/ascii"

# Copying the default ars files to ascii folder
cp ./artifacts/ascii/* ~/.config/artsci/ascii/

# Builing the release version for installation
cargo build --release

# copying the generated config to bin
cp ./target/release/artsci /usr/bin/