#!/bin/sh

export RUST_BACKTRACE=1
export RUST_LOG=info
INPUT_BZ2="/path/to/jawiki-latest-pages.hogehoge.bz2"
OUTPUT="/path/to/prefix"
cargo run $INPUT_BZ2 $OUTPUT
