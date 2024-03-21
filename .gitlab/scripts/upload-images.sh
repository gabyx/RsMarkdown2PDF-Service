#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/general.sh"

cd "$ROOT_DIR"

function build_ci_image() {
    local image_type="$1"
    local repository="${2:-gabyxgabyx/rsmd2pdf-service}"
    local tag="$image_type-${3:-latest}"

    local image_name="$repository:$tag"

    print_info "Building image '$image_name'."

    docker build -f .gitlab/docker/Dockerfile \
        --target "$image_type" \
        -t "$image_name" \
        . || die "Could not build image."

    docker push "$image_name" || die "Could not upload image."
}

repository="${1:-gabyxgabyx/rsmd2pdf-service}"
tag="${2:-2.0.0}"

if [ "${CI:-}" = "true" ]; then
    ci_docker_login gabyxgabyx "$DOCKER_REPOSITORY_READ_TOKEN"
fi

build_ci_image "ci-test" "$repository" "$tag"

build_ci_image "ci-docker" "$repository" "$tag"
build_ci_image "ci-docker-dind" "$repository" "$tag"

build_ci_image "ci-format" "$repository" "$tag"
build_ci_image "ci-lint" "$repository" "$tag"
build_ci_image "ci-lint-docs" "$repository" "$tag"
build_ci_image "ci-build" "$repository" "$tag"
