#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/general.sh"

cd "$ROOT_DIR"

trap clean_up EXIT

function clean_up() {
    if [ "${CI:-}" = "true" ]; then
        rm -rf "$GITHOOKS_INSTALL_PREFIX" || true
        git clean -dfx || die "Could not clean Git dir."
    fi
}

function ci_assert_no_diffs() {
    if ! git diff --quiet; then
        die "Commit produced diffs, probably because of format:" \
            "$(git diff main)" \
            "Run 'just format' to resolve."
    fi
}

function run_lint_shared_hooks() {
    print_info "Run all formats scripts in shared hook repositories."
    git hooks exec --containerized \
        ns:githooks-shell/scripts/check-shell-all.yaml -- --force --dir "."
}

function run_lint_general() {
    "tools/run-components-parallel.sh" "$parallel" "$regex" lint
}

parallel="$1"
regex="$2"

if [ "${CI:-}" = "true" ]; then
    ci_setup_githooks "$GITHOOKS_INSTALL_PREFIX"
    ci_assert_no_diffs
fi

run_lint_general
run_lint_shared_hooks
