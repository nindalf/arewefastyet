#!/bin/bash

# git clone https://github.com/nindalf/arewefastyet
# cd arewefastyet
# ./collect_samples.sh

sudo apt-get install -y build-essential nasm # nasm required for rav1e
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

CORES=$(cat /proc/cpuinfo | grep cores | head -n 1 | cut -d' ' -f3)
cd cmd
cargo build --release
./target/release/arewefastyet --times 5 --results ../data/results-$CORES.json