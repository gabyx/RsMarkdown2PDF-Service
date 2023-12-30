#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
#
# Create a gitlab runner by first visiting:
# `CI/CD Settings page and creating a `linux` runner.
# The token has to be given to this function.
set -e
set -u

ROOT=$(git rev-parse --show-toplevel)
. "$ROOT/tools/general.sh"

force="false"
config_dir="$ROOT/.gitlab/local/config"
runner_name="gitlab-runner"
cores=$(grep "^cpu\\scores" /proc/cpuinfo | uniq | cut -d ' ' -f 3)

function create() {
    local token="${1:?First argument must be the runner token.}"

    rm -rf "$config_dir" >/dev/null || true
    mkdir -p "$config_dir"

    docker run -d \
        --cpus "$cores" \
        --name "$runner_name" \
        --restart always \
        -v /var/run/docker.sock:/var/run/docker.sock \
        -v "$config_dir":/etc/gitlab-runner \
        gitlab/gitlab-runner:latest || die "Could not create gitlab-runner"

    docker exec -it "$runner_name" gitlab-runner register \
        --non-interactive \
        --url https://gitlab.com --token "$token" \
        --executor docker \
        --description "md2pdf-ci-$USER" \
        --docker-image "docker:24" \
        --docker-privileged \
        --docker-volumes "/certs/client" || die "Could not start gitlab runner"
}

function stop() {
    if is_running; then
        docker stop "$runner_name"
        # shellcheck disable=SC2046
        docker rm $(docker ps -a -q)
    fi
}

function is_running() {
    [ "$(docker inspect -f '{{.State.Running}}' "$runner_name" 2>/dev/null || true)" = 'true' ] || return 1
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
    printInfo "Gitlab runner '$runner_name' is already running. Restart it."
    docker restart "$runner_name" || die "Could not restart gitlab runner"
fi
