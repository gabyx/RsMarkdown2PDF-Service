#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/general.sh"

cd "$ROOT_DIR" &&
    ci_container_mgr_run_mounted "$(pwd)" \
        docker.io/gabyxgabyx/rsmd2pdf-service:ci-lint-rust-2.0.1 \
        cargo clippy "$@" -- \
        -A clippy::needless_return
