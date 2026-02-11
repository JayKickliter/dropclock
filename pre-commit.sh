#!/bin/sh

set -ex

cargo fmt -- --check
cargo clippy --all-targets -- -Dclippy::pedantic -Dclippy::cargo
cargo test
