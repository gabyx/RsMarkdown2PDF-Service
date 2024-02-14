#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
#
# Run a specific component task:
#   run-component-task.sh api build
# or starting from a path inside the component
#   run-component-task.sh api/.env build
set -e
set -u

ROOT=$(git rev-parse --show-toplevel)
. "$ROOT/tools/general.sh"

function run {
    local path="$1"
    local task="$2"
    shift 2

    # If we are given a normal path.
    # otherwise just treat it as component name.
    if [ -e "$path" ]; then
        path=$(realpath "$path")
        if [ -f "$path" ]; then
            path=$(basename "$path")
        fi
    else
        path="$ROOT/components/$path"
    fi

    [ -d "$path" ] || {
        die "Path '$path' does not exist."
    }

    print_info "Start task '$task' inside directory '$path'."

    cd "$path" && just --justfile justfile "$task" "$@"
}

run "$@"
