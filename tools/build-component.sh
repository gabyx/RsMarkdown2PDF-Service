#!/usr/bin/env bash
# Build a specific component. Started by specifiying a file which then
# locates the component and builds it.

function build {
    local path
    path=$(realpath "$1")
    echo "Build started from '$path'"

    shift 1

    if [ -f "$path" ]; then
        path=$(basename "$path")
    fi

    cd "$path" && just build "$@"
}

build "$@"
