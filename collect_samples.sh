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
sudo apt-get install -y libssl-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

CORES=$(cat /proc/cpuinfo | grep cores | head -n 1 | cut -d' ' -f3)
cd cmd
cargo build --release
./target/release/arewefastyet --times 5 --results ../data/results-$CORES.json