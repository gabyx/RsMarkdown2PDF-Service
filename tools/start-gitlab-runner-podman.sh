#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091,SC2015
#
# Create a gitlab runner by first visiting:
# `CI/CD Settings page and creating a `linux` runner.
# The token has to be given to this function.
set -e
set -u

ROOT=$(git rev-parse --show-toplevel)
. "$ROOT/tools/general.sh"

force="false"
max_jobs=4
config_dir="$ROOT/.gitlab/local/config"
runner_name="gitlab-runner-md2pdf-podman"
cores=$(grep "^cpu\\scores" /proc/cpuinfo | uniq | cut -d ' ' -f 3)
image="registry.gitlab.com/qontainers/pipglr:latest@sha256:ca3093e580bd617d5f92b67a5b6ec9411e2d8fbff36d8edd1070fccba9d241d2"

function clean_up() {
    if [ -f "$config_dir/config.toml" ]; then
        true
        # rm -rf "$config_dir/config.toml"
    fi
}

trap clean_up EXIT
function modify_config() {
    local key="$1"
    local value="$2"
    local type="${3:-json}"

    podman run -v "$config_dir/config.toml:/config.toml" \
        "ghcr.io/tomwright/dasel" put -f /config.toml \
        -t "$type" \
        -s "$key" \
        -v "$value" ||
        die "Could not set gitlab runner config key '$key' to '$value'"
}

function register_runner() {
    print_info "Registering gitlab-runner ..."
    local token="$1"

    podman secret rm REGISTRATION_TOKEN &>/dev/null || true
    echo "$token" | podman secret create REGISTRATION_TOKEN - ||
        die "Could not set registration token secret."

    (cd "$config_dir" &&
        touch config.toml &&
        podman container runlabel register "$image") ||
        die "Could not register gitlab-runner."

    modify_config ".concurrent" "$max_jobs"
    modify_config ".runners.first().docker.pull_policy" '["always", "if-not-present"]'
    modify_config ".runners.first().docker.volumes.append()" "/home/runner/podman.sock:/var/run/docker.sock:rw" string

    podman secret rm config.toml &>/dev/null || true
    podman secret create config.toml "$config_dir/config.toml" ||
        die "Could not create config.toml secret."

    print_info "Config file:" \
        "$(sed 's/token\s*=.*/token = ***/g' "$config_dir/config.toml")"

    rm "$config_dir/config.toml"
}

function assert_volumes() {
    print_info "Asserting needed volumes ..."

    local volumes
    volumes=$(podman volume list --format="{{ .Name }}") ||
        die "Could not get volumes"

    if ! echo "$volumes" | grep -q "pipglr-storage"; then
        podman container runlabel setupstorage "$image"
    fi

    if ! echo "$volumes" | grep -q "pipglr-cache"; then
        podman container runlabel setupcache "$image"
    fi
}

function start_runner() {
    print_info "Start runner '$runner_name' ..."

    # Run the Gitlab runner. We cannot user `podman container runlabel run "$image"`
    # because we need to set some cpu constraints.
    podman run -dt --name "$runner_name" \
        --cpus "$cores" \
        --secret config.toml,uid=1001,gid=1001 \
        -v pipglr-storage:/home/podman/.local/share/containers \
        -v pipglr-cache:/cache \
        --systemd true --privileged \
        --device /dev/fuse "$image"

    podman exec -it --user root "$runner_name" \
        bash -c "mkdir -p /etc/containers;
                 cp /usr/share/containers/seccomp.json /etc/containers/seccomp.json"
}

function create() {
    local token="${1:?First argument must be the runner token.}"

    rm -rf "$config_dir" >/dev/null || true
    mkdir -p "$config_dir"

    register_runner "$token"
    assert_volumes

    start_runner
}

function stop() {
    if is_running; then
        print_info "Stop runner '$runner_name' ..."
        podman stop "$runner_name"

        # shellcheck disable=SC2046
        podman rm $(podman ps -a -q)
    fi
}

function is_running() {
    [ "$(podman inspect -f '{{.State.Running}}' "$runner_name" 2>/dev/null || true)" = 'true' ] || return 1
    return 0
}

if [ "${1:-}" = "--force" ]; then
    force="true"
    shift 1
fi

if [ "$force" = "true" ]; then
    stop
fi

if ! is_running; then
    create "$@"
else
    print_info "Gitlab runner '$runner_name' is already running. Restart it."
    podman restart "$runner_name" ||
        die "Could not restart gitlab runner '$runner_name'."
fi
