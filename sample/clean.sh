#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
echo "$SCRIPT_DIR"
cd "$SCRIPT_DIR" || exit

echo "We're cleaning every contract :-)"
for dir in */; do
    # Remove trailing slash from directory name
    dir=${dir%/}

    echo "Building $dir..."

    # Ensure we have a Cargo.toml file in the directory before proceeding
    if [ -f "$dir/Cargo.toml" ]; then
        (cd "$dir" && cargo clean)
        echo "Finished cleaning $dir"
    else
        echo "Skipping $dir: Cargo.toml not found."
    fi

    echo
done

echo "Cleaned everything"
