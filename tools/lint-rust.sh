#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/general.sh"

cd "$ROOT_DIR" &&
    nix develop .#ci --command \
        cargo clippy --no-deps -- \
        -A clippy::needless_return "$@"
