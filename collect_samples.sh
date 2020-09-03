#!/bin/bash

# git clone https://github.com/nindalf/arewefastyet
# cd arewefastyet
# ./collect_samples.sh

sudo apt-get install -y build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

cd cmd
cargo build --release
./target/release/arewefastyet --times 5 --results ../results.json