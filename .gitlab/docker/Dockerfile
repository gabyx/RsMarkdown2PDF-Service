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
COPY ./tools /container-setup
RUN apk add bash
RUN bash -c ". /container-setup/general.sh && ci_setup_nix"

# Pre-cache the whole Nix packages.
COPY flake.nix flake.lock rust-toolchain.toml /container-setup/
RUN cd /container-setup && nix --accept-flake-config \
        build --no-link ".#devShells.x86_64-linux.ci"

# Format image.
# ===============================================
FROM alpine:latest as ci-format
RUN apk add findutils coreutils git jq curl bash just parallel podman

FROM docker.io/instrumentisto/rust:alpine as ci-format-rust

# Lint image.
# ===============================================
FROM ci-format as ci-lint
FROM ci-format as ci-lint-docs

# Build image.
# ===============================================
FROM ci-docker-nix as ci-build
