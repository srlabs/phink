#!/bin/bash

# Ensure we're starting in the correct directory
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
# Goto the root of the project
cd "$SCRIPT_DIR" || exit

docker build -t phink "$SCRIPT_DIR/.."
docker run --rm phink sh -c "cargo test -- --show-output"
echo "Phink's tests ran within Docker"