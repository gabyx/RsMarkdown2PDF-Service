# This is a docker image containing docker and a Nix store.
# This enables to either run Docker images inside this one,
# or use `nix develop` to start a sandboxed environment to
# do other non-docker related stuff.

FROM alpine:latest as base-podman
RUN apk add findutils coreutils git jq curl bash just parallel podman

# Nix Image
# ===============================================
FROM base-podman as ci-nix
RUN [ "TARGETPLATFORM" = "linux/amd64" ] || echo "Platform not yet supported."
COPY ./tools /container-setup
# Install Nix and pre-cache the env.
RUN bash -c ". /container-setup/general.sh && ci_setup_nix"
COPY flake.nix flake.lock rust-toolchain.toml /container-setup/
RUN cd /container-setup && nix --accept-flake-config \
        build --no-link ".#devShells.x86_64-linux.ci"

# Format image.
# ===============================================
FROM ci-nix as ci-format

# Lint image.
# ===============================================
FROM ci-nix as ci-lint

# Build image.
# ===============================================
FROM ci-nix as ci-build
