#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
echo "$SCRIPT_DIR"
cd "$SCRIPT_DIR" || exit

echo "We're building every contract :-)"
for dir in */; do
    # Remove trailing slash from directory name
    dir=${dir%/}

    echo "Building $dir..."

    # Ensure we have a Cargo.toml file in the directory before proceeding
    if [ -f "$dir/Cargo.toml" ]; then
        if [ "$dir" = "multi-contract-caller" ]; then
            # Execute build-all.sh for multi-contract-caller
            (cd "$dir" && ./build-all.sh)
        else
            # Execute cargo contract build for other directories
            (cd "$dir" && cargo clean)
        fi
        echo "Finished building $dir"
    else
        echo "Skipping $dir: Cargo.toml not found."
    fi

    echo
done

echo "All builds completed."
