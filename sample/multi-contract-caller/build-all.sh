#!/usr/bin/env bash

set -eu

cargo contract build --manifest-path accumulator/Cargo.toml --features phink
cargo contract build --manifest-path adder/Cargo.toml --features phink
cargo contract build --manifest-path subber/Cargo.toml --features phink
cargo contract build --features phink