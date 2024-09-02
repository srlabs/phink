#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
# Goto the root of the project
cd "$SCRIPT_DIR" || exit

cargo +nightly clippy --all --all-features --allow-dirty --no-deps --tests --examples --benches -- -Z macro-backtrace \
   -D clippy::all \
   -D clippy::pedantic \
   -W clippy::similar_names \
   -A clippy::type_repetition_in_bounds \
   -A clippy::missing-errors-doc \
   -A clippy::cast-possible-truncation \
   -A clippy::used-underscore-binding \
   -A clippy::ptr-as-ptr \
   -A clippy::missing-panics-doc \
   -A clippy::missing-docs-in-private-items \
   -A clippy::unseparated-literal-suffix \
   -A clippy::module-name-repetitions \
   -A clippy::unreadable-literal

echo "Clippy executed"