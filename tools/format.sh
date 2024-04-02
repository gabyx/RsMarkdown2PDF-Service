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

function run_format_shared_hooks() {
    print_info "Run all formats scripts in shared hook repositories."

    local temp
    temp=$(mktemp)

    cat <<<"
    shared-mount-dest:
    workspace-mount-dest:
    " | sed -E 's/^\s+//g' >"$temp"

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

if ci_is_running; then
    ci_container_mgr_login gabyxgabyx "$DOCKER_REPOSITORY_READ_TOKEN"
    ci_setup_githooks "$GITHOOKS_INSTALL_PREFIX"
fi

run_format_general
run_format_shared_hooks

if ci_is_running; then
    ci_assert_no_diffs
fi
