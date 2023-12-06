#/usr/bin/env bash
# Build a specific component.

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
