#!/bin/bash
set -e
set -x

DIR=`dirname $(readlink -f $0)`
cd $DIR/..

# Install dependencies.
apt-get update
apt-get install -y \
    rustc \
    cargo \
;

# Show rust version.
rustc --version

# Build the project.
cargo build

# Smoke test the result.
cargo run
