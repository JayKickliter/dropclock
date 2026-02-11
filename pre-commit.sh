#!/bin/sh

set -ex

cargo fmt -- --check
cargo clippy --all-targets -- -Dclippy::all
cargo test
