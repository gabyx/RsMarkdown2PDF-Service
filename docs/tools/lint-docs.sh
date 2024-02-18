#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/general.sh"

cd "$ROOT_DIR"

print_info "Running vale over all markdown documents."
docker run --rm \
    -v "$(pwd):/workspace" \
    -w /workspace jdkato/vale:latest \
    --config docs/.vale/vale.ini \
    docs README.md
