#!/bin/bash
# Inspired from LibAFL scripts

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
# Goto the root of the project
cd "$SCRIPT_DIR" || exit
echo "Use --no-clean if you don't want to 'cargo clean' Phink"
if [ "$1" != "--no-clean" ]; then
   # Usually, we want to clean, since clippy won't work otherwise.
   echo "[+] Cleaning up previous builds..."
   cargo clean -p phink
fi
echo

echo "[+] Fixing build"
cargo +nightly fix --release --workspace --all-features --allow-dirty --allow-staged

echo "[+] Done fixing build"
echo

echo 'Fixing clippy (might need a "git commit" and a rerun, if "cargo fix" changed the source)'

cargo +nightly clippy --fix --tests --examples --benches --all-features --allow-dirty --allow-staged

echo "[+] Done fixing clippy"
echo

echo "[+] Formatting all"

cargo +nightly fmt --all