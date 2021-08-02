#!/bin/bash

set -e

echo "building the project in release mode"
mkdir -p /tmp/aoc-benches
cargo build --release --bins --target-dir=/tmp/aoc-benches

echo "sequentially running every binary"
time -p sh -c 'for f in /tmp/aoc-benches/release/day*[!.d]; do
	$f
done'
