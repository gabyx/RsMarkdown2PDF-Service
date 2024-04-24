#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091,SC2119
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

    if ci_is_running; then
        TEMP_RUN_CONFIG=$(mktemp)

        cat <<<"
        shared-path-dest: $CI_GITHOOKS_INSTALL_PREFIX/.githooks/shared
        workspace-path-dest: $ROOT_DIR
        auto-mount-workspace: false
        auto-mount-shared: false
        args: [ '--volumes-from' , '$CI_JOB_CONTAINER_ID' ]
    " | sed -E 's/^\s+//g' >"$TEMP_RUN_CONFIG"

        echo "Setting containerized run config for Githooks."
        cat "$TEMP_RUN_CONFIG"

        # Set the mount arguments to influence
        # Githooks containerized execution.
        export GITHOOKS_CONTAINER_RUN_CONFIG_FILE="$TEMP_RUN_CONFIG"
    fi

    git hooks exec --containerized \
        ns:githooks-shell/scripts/check-shell-all.yaml -- --force --dir "."

    if ci_is_running; then
        rm -rf "$TEMP_RUN_CONFIG"
    fi
}

function run_lint_general() {
    "tools/run-components-parallel.sh" "$parallel" "$regex" lint
}

parallel="$1"
regex="$2"

if ci_is_running; then
    ci_container_mgr_login gabyxgabyx "$DOCKER_REPOSITORY_READ_TOKEN"
    ci_setup_githooks
fi

run_lint_general
run_lint_shared_hooks

if ci_is_running; then
    ci_assert_no_diffs
fi
