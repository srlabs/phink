#!/bin/bash
# Inspired from LibAFL script

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
# Goto the root of the project
cd "$SCRIPT_DIR" || exit
echo "[+] Use --no-clean if you don't want to cargo clean"
if [ "$1" != "--no-clean" ]; then
   # Usually, we want to clean, since clippy won't work otherwise.
   echo "[+] Cleaning up previous builds..."
   cargo clean -p phink
fi
echo

echo "[+] Clippying everything"
cargo +nightly fix --release --workspace --all-features --allow-dirty --allow-staged
cargo +nightly clippy --fix --examples --benches --all-features --allow-dirty --allow-staged
cargo +nightly clippy --all-targets -- -D warnings
echo "[+] Done fixing clippy"

echo "[+] Formatting all"

cargo +nightly fmt --all
