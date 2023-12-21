#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/general.sh"

parallel="${1:-false}"
regex="$2"
task="$3"
shift 3

printInfo "Run task '$task' in parallel over all components with:" "$@"

cd "$ROOT_DIR"

readarray -t comps < <(just list-components "$regex")

if [ "$parallel" = "true" ]; then
    parallel just component {} "$task" "$@" ::: "${comps[@]}"
else
    for comp in "${comps[@]}"; do
        just component "$comp" "$task" "$@"
    done
fi
