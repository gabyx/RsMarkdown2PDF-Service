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

function ci_setup_githooks() {
    mkdir -p "$GITHOOKS_INSTALL_PREFIX"

    printInfo "Install Githooks."
    curl -sL "https://raw.githubusercontent.com/gabyx/githooks/main/scripts/install.sh" |
        bash -s -- -- --use-manual --non-interactive --prefix "$GITHOOKS_INSTALL_PREFIX"

    git hooks config enable-containerized-hooks --global --set

    printInfo "Pull all shared Githooks repositories."
    git hooks shared update
}

function ci_assert_no_diffs() {
    if ! git diff --quiet; then
        die "Commit produced diffs, probably because of format:" \
            "$(git diff main)" \
            "Run 'just format' to resolve."
    fi
}

function run_format_shared_hooks() {
    printInfo "Run all formats scripts in shared hook repositories."
    git hooks exec --containerized \
        ns:githooks-shell/scripts/format-shell-all.yaml -- --force --dir "."

    git hooks exec --containerized \
        ns:githooks-configs/scripts/format-configs-all.yaml -- --force --dir "."

    git hooks exec --containerized \
        ns:githooks-docs/scripts/format-docs-all.yaml -- --force --dir "."

    git hooks exec --containerized \
        ns:githooks-python/scripts/format-python-all.yaml -- --force --dir "."
}

function run_format_general() {
    "tools/run-components-parallel.sh" "$parallel" "$regex" format
}

parallel="$1"
regex="$2"

if [ "${CI:-}" = "true" ]; then
    ci_setup_githooks
    ci_assert_no_diffs
fi

run_format_general
run_format_shared_hooks
