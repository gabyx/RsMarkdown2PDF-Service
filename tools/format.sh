#!/usr/bin/env bash
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)

cd "$ROOT_DIR"

readarray -t components < <(find ./components -mindepth 1 -maxdepth 1 -type d)
for dir in "${components[@]}"; do
    (cd "$dir" && just format)
done

readarray -t starlark_files < <(find . -name "Tiltfile")
black "${starlark_files[@]}"
