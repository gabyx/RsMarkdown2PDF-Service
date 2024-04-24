#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/general.sh"

cd "$ROOT_DIR"
ci_wrap_container_or_nix \
    docker.io/gabyxgabyx/rsmd2pdf-service:ci-format-2.0.1 \
    .#ci \
    cargo fmt -- --config-path /repo "$@"
