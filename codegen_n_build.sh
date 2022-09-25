#!/usr/bin/env bash

# Invokes the codegen project in "./codegen" to generate the crate in
# "./src". Afterwards it applies Rustfmt to it, Clippy, and builds everything.

set -e
set -x

# nice "hack" which make the script work, even if not executed from "./"
DIR=$(dirname "$(realpath "$0")")
cd "$DIR" || exit

echo "This script generates the crate 'noto-sans-mono-bitmap', verifies the build, and applies Rustfmt and clippy afterwards."

# delete all generated raster files from previous run; irrelevant anyway
find src/res_rasterized_characters -type f -name "*.txt" -exec rm {} +
cd "codegen" || exit
# Needs rustc 1.58 or above
cargo fmt
cargo test
RUSTFLAGS="-C target-cpu=native" cargo run --release --bin codegen
cd ..

cargo fmt
cargo clippy --features all  --all-targets
cargo doc --features all
cargo build --features all --all-targets
