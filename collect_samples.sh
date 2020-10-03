#!/bin/bash

# git clone https://github.com/nindalf/arewefastyet
# cd arewefastyet
# ./collect_samples.sh

sudo apt-get update
sudo apt-get install -y build-essential 
# required for rav1e
sudo apt-get install -y nasm
# required for alacritty
sudo apt-get install -y cmake pkg-config libfreetype6-dev libfontconfig1-dev libxcb-xfixes0-dev python3
#required for tokio
sudo apt-get install -y libssl-dev pkg-config

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

cd cmd
echo "Building arewefastyet"
cargo build --release

# Log at level info to file output.log.
# Collect 5 samples for each pair of CompilerMode and ProfileMode.
# Run in the background.
env RUST_LOG=info ./target/debug/arewefastyet --times 5 --results-dir ../data/ 2> output.log &

tail -f output.log