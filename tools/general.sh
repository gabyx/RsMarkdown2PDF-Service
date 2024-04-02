#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
# shellcheck disable=SC2154,SC2086

function _print() {
    local color="$1"
    local flags="$2"
    local header="$3"
    shift 3

    local hasColor="0"
    if [ "${FORCE_COLOR:-}" != 1 ]; then
        [ -t 1 ] && hasColor="1"
    else
        hasColor="1"
    fi

    if [ "$hasColor" = "0" ] || [ "${LOG_COLORS:-}" = "false" ]; then
        local msg
        msg=$(printf '%b\n' "$@")
        msg="${msg//$'\n'/$'\n'   }"
        echo $flags -e "-- $header$msg"
    else
        local s=$'\033' e='[0m'
        local msg
        msg=$(printf "%b\n" "$@")
        msg="${msg//$'\n'/$'\n'   }"
        echo $flags -e "${s}${color}-- $header$msg${s}${e}"
    fi
}
function print_info() {
    _print "[0;94m" "" "" "$@"
}

function print_warning() {
    _print "[0;31m" "" "WARN: " "$@" >&2
}

function print_error() {
    _print "[0;31m" "" "ERROR: " "$@" >&2
}

function die() {
    print_error "$@"
    exit 1
}

function ci_is_running() {
    if [ "${CI:-}" = "true" ]; then
        return 0
    fi

    return 1
}

function ci_setup_githooks() {
    local installPrefix="${1:-$CI_BUILDS_DIR/githooks}"
    mkdir -p "$installPrefix"
    print_info "Install Githooks in '$installPrefix'."

    if [ -n "${NIX_PATH:-}" ] && [ ! -f /etc/os-release ]; then
        # Write some OS detection file which is not available in nixos images.
        local version
        version=$(grep -E -m 1 "nixos-" "$ROOT_DIR/.gitlab/pipeline.yaml" |
            sed -E "s/.*:nixos-(.*)/\1/")

        {
            echo ID=nixos
            echo VERSION_ID=\"$version\"
        } >/etc/os-release

    fi

    print_info "Install Githooks."
    curl -sL "https://raw.githubusercontent.com/gabyx/githooks/main/scripts/install.sh" |
        bash -s -- -- --use-manual --non-interactive --prefix "$installPrefix"

    git hooks config enable-containerized-hooks --global --set
    git hooks config container-manager-types --global --set "podman,docker"

    print_info "Pull all shared Githooks repositories."
    git hooks shared update

    export CI_GITHOOKS_INSTALL_PREFIX="$installPrefix"
}

function ci_setup_nix() {
    local install_prefix="${1:-/usr/sbin}"

    print_info "Install Nix."
    apk add curl bash xz shadow
    sh <(curl -L https://nixos.org/nix/install) --daemon --yes
    cp /root/.nix-profile/bin/* "$install_prefix/"

    print_info "Enable Features for Nix."
    mkdir -p ~/.config/nix
    {
        echo "experimental-features = nix-command flakes"
        echo "accept-flake-config = true"
    } >~/.config/nix/nix.conf
}

# Run the container manager which is defined.
function ci_container_mgr() {
    if command -v podman &>/dev/null; then
        podman "$@"
    else
        docker "$@"
    fi
}

# Define the container id `CI_JOB_CONTAINER_ID` where
# this job runs. Useful to mount same volumes as in
# this container with `ci_run_podman`.
function ci_container_mgr_setup() {
    export CONTAINER_HOST="unix://var/run/podman.sock"
    print_info "Container host: '$CONTAINER_HOST'"

    job_container_id=$(ci_container_mgr ps \
        --filter "label=com.gitlab.gitlab-runner.type=build" \
        --filter "label=com.gitlab.gitlab-runner.job.id=$CI_JOB_ID" \
        --filter "label=com.gitlab.gitlab-runner.project.id=$CI_PROJECT_ID" \
        --filter "label=com.gitlab.gitlab-runner.pipeline.id=$CI_PIPELINE_ID" \
        --format "{{ .ID }}") ||
        die "Could not find 'build' container for job id: '$CI_JOB_ID'."

    [ -n "$job_container_id" ] || die "Job id is empty."

    export CI_JOB_CONTAINER_ID="$job_container_id"
    print_info "Job container id: '$CI_JOB_CONTAINER_ID'"
}

function ci_container_mgr_login() {
    local user="$1"
    local token="$2"

    [ -n "$token" ] || die "Docker login token is empty"
    echo "$token" |
        ci_container_mgr login --password-stdin --username "$user" ||
        die "Could not log into docker."
}

# Run podman with volume mount from the
# current build container `CI_JOB_CONTAINER_ID`.
function ci_container_mgr_run() {
    [ -n "$CI_JOB_CONTAINER_ID" ] ||
        ci_define_job_container_id

    ci_container_mgr run --volumes-from "$CI_JOB_CONTAINER_ID" "$@"
}
