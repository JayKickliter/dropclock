#!/bin/sh

set -ex

export RUSTFLAGS="-Dwarnings"
export RUSTDOCFLAGS="-Dwarnings"

cargo fmt -- --check
cargo clippy --all-targets -- -Wclippy::pedantic -Wclippy::cargo
cargo test
cargo doc --no-deps
