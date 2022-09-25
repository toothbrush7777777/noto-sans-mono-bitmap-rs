#!/usr/bin/env bash

# Invokes the codegen project in "./codegen" to generate the crate in
# "./src". Afterwards it applies Rustfmt to it, Clippy, and builds everything.

set -e
set -x

# nice "hack" which make the script work, even if not executed from "./"
DIR=$(dirname "$(realpath "$0")")
cd "$DIR" || exit

echo "This script generates the crate 'noto-sans-mono-bitmap', verifies the build, and applies Rustfmt and clippy afterwards."

cd "codegen" || exit
# Needs rustc 1.58 or above
RUSTFLAGS="-C target-cpu=native" cargo +stable run --release --bin codegen
cd ..

cargo fmt
cargo +stable clippy --features all  --all-targets
cargo +stable doc --features all
cargo +stable build --features all --all-targets
