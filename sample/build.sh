#!/bin/bash

echo "We're building every contract :-) see ya! "
for dir in */; do
    # Remove trailing slash from directory name
    dir=${dir%/}

    # Change to the directory
    cd "$dir"

    echo "Building $dir..."

    # Check if the current directory is multi-contract-caller
    if [ "$dir" = "multi-contract-caller" ]; then
        # Execute build-all.sh for multi-contract-caller
        ./build-all.sh
    else
        # Execute cargo contract build for other directories
        cargo contract build --features phink
    fi

    # Change back to the parent directory
    cd ..

    echo "Finished building $dir"
    echo
done

echo "All builds completed."

