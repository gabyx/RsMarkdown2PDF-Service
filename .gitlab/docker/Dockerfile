# This is a docker image containing docker and a Nix store.
# This enables to either run Docker images inside this one,
# or use `nix develop` to start a sandboxed environment to
# do other non-docker related stuff.

FROM docker:24-dind as ci-docker-dind

FROM docker:24 as ci-docker
RUN apk add git

# Docker-in-Docker with Nix.
# ===============================================
FROM ci-docker as ci-docker-nix
RUN [ "TARGETPLATFORM" = "linux/amd64" ] || echo "Platform not yet supported."

# Setup Nix.
# ===============================================
RUN mkdir -p /container-setup
COPY ./tools /container-setup
RUN . /container-setup/general.sh && ci_setup_nix

# Pre-cache the whole Nix packages.
COPY flake.nix flake.lock rust-toolchain.toml /container-setup/
RUN cd /container-setup && nix --accept-flake-config \
        build --no-link ".#devShells.x86_64-linux.ci"

# Format image.
# ===============================================
FROM alpine:latest as ci-format
RUN apk add git jq curl bash just findutils parallel podman


# Lint image.
# ===============================================
FROM ci-docker-nix as ci-lint
FROM ci-docker-nix as ci-lint-docs

# Build image.
# ===============================================
FROM ci-docker-nix as ci-build

# Test
# ===============================================
FROM alpine:latest as ci-test
RUN apk add sudo
ARG USER_NAME=ci
ARG USER_UID=1000
ARG USER_GID=1000
RUN adduser "$USER_NAME" -D -u "$USER_UID" -g "$USER_GID" -h "/home/$USER_NAME" && \
    mkdir -p /etc/sudoers.d && \
    echo "$USER_NAME ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/$USER_NAME" && \
    chmod 0440 "/etc/sudoers.d/$USER_NAME" && \
    chown -R "$USER_NAME:$USER_NAME" /home
RUN apk add podman
USER "$USER_NAME"
