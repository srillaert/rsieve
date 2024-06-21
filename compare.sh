#!/usr/bin/bash
cargo build --release
stop="100000000"
echo
echo rsieve
time target/release/rsieve $stop
echo
echo primesieve
time primesieve --quiet --threads=1 $stop
