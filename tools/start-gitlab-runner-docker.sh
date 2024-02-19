#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
#
# Create a gitlab runner (docker executor)
# by first visiting `CI/CD Settings` page and
# creating a `linux` runner which gives you a `<token>` needed
# for this script.
#
# This creates a docker container which runs the Gitlab runner
# which will execute jobs over the `docker` executor.
# The running container is not that safe in the sense that the Docker socket
# is mounted into the container (privilege escalation can be done:
# .https://github.com/stealthcopter/deepce).
# The `gitlab-runner` does not forward the socket to the job containers
# because that would be to risky. Nevertheless,
# docker-in-docker for a job works as shown below.
#
# Usage:
# ```shell
# start-gitlab-runner-docker.sh [--force] [<token>]
# ```
# Read token from stdin.
# ```shell
# start-gitlab-runner-docker.sh [--force] -
#
# Usage in Pipeline:
#
# A job which uses `docker` to run/build images.
# the `service`-container `docker:24-dind`.
#
# ```yaml
# docker-run-build:
#   image: docker:24
#   #
#   # When you use the dind service, you must instruct Docker to talk with
#   # the daemon started inside of the service 'docker:*-dind'.
#   # The daemon is available with a network connection instead of the default
#   # /var/run/docker.sock socket.
#   # Docker does this automatically by setting the DOCKER_HOST in
#   # https://github.com/docker-library/docker/blob/master/docker-entrypoint.sh#L30
#   # The 'docker' hostname is the alias of the service container as described
#   # at https://docs.gitlab.com/ee/ci/services/#accessing-the-services.
#   # which is `docker` and then DOCKER_HOST=tcp://docker:2376
#   services:
#     - docker:24-dind
#
#   script:
#     - docker info
#     - docker run alpine:latest cat /etc/os-release
#     - docker build -f Dockerfile .
# ```

set -e
set -u

ROOT=$(git rev-parse --show-toplevel)
. "$ROOT/tools/general.sh"

force="false"
max_jobs=4
config_dir="$ROOT/.gitlab/local/config"
runner_name="gitlab-runner-md2pdf"
cores=$(grep "^cpu\\scores" /proc/cpuinfo | uniq | cut -d ' ' -f 3)

function create() {
    local token="${1:-}"

    if [ "$token" = "-" ] || [ -z "$token" ]; then
        read -rs -p "Enter Gitlab Runner Token: " token ||
            die "Could not read token from TTY."
    fi

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

    # Set concurrency.
    docker exec -it "$runner_name" \
        sed -i "s/concurrent =.*/concurrent = $max_jobs/" \
        "/etc/gitlab-runner/config.toml" || die "Could not set concurrency."

    docker exec -it "$runner_name" gitlab-runner start || die "Could not start runner."
}

function stop() {
    if is_running; then
        print_info "Stop runner '$runner_name' ..."
        podman stop "$runner_name"

    fi

    if is_exited; then
        # shellcheck disable=SC2046
        podman rm $(podman ps -a -q)
    fi
}

function is_running() {
    [ "$(podman inspect -f '{{.State.Status}}' "$runner_name" 2>/dev/null || true)" = 'running' ] || return 1
    return 0
}

function is_exited() {
    [ "$(podman inspect -f '{{.State.Status}}' "$runner_name" 2>/dev/null || true)" = 'exited' ] || return 1
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
    docker restart "$runner_name" || die "Could not restart gitlab runner"
fi
