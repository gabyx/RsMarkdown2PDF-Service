#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/general.sh"

cd "$ROOT_DIR"
ci_wrap_container \
    docker.io/gabyxgabyx/rsmd2pdf-service:ci-lint-2.0.1 \
    nix develop .#ci --command \
    cargo clippy --no-deps -- -A clippy::needless_return "$@"
