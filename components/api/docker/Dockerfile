ARG COMPONENT="api"
ARG ROOT_DIR="/build"
ARG BUILD_TYPE="release"
ARG ENABLE_BACKTRACE="0"

## Base Stage ##############################################################
FROM rust:1.74 AS chef
ARG ROOT_DIR
RUN apt-get update && apt-get install -y jq python3 pipx
RUN pipx install yq
ENV PATH="/root/.local/bin:$PATH"

RUN cargo install cargo-chef --locked
WORKDIR "$ROOT_DIR"

## Planner Stage ##############################################################
FROM chef as planner
ARG COMPONENT
COPY . .
RUN tools/docker/set-workspace-for-component.sh "$COMPONENT"
RUN cargo chef prepare --recipe-path recipe.json

## Builder Stage ##############################################################
FROM chef AS builder
ARG COMPONENT
ARG ROOT_DIR
ARG BUILD_TYPE

COPY --from=planner /$ROOT_DIR/recipe.json recipe.json
COPY tools/docker ./tools/docker

# Build dependencies - this is the caching Docker layer!
RUN ./tools/docker/run-cargo-deps.sh "$BUILD_TYPE"
COPY . .

COPY --from=planner "$ROOT_DIR/Cargo.toml" ./
RUN ./tools/docker/run-cargo.sh "$BUILD_TYPE"


## Service Stage ##############################################################
FROM ubuntu:latest
ARG COMPONENT
ARG ROOT_DIR
ARG BUILD_TYPE
ARG ENABLE_BACKTRACE

ARG APP=/usr/src/app
EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    RUST_BACKTRACE=$ENABLE_BACKTRACE

RUN groupadd "$APP_USER" \
    && useradd -g "$APP_USER" "$APP_USER" \
    && mkdir -p "${APP}"

RUN apt update && \
    apt-get install -y libpq5 && \
    apt-get clean && \
    rm -rf /var/cache/apt/archives /var/lib/apt/lists/*

COPY --from=builder "$ROOT_DIR/target/$BUILD_TYPE/$COMPONENT" "${APP}/app"
RUN chown -R "$APP_USER:$APP_USER" "${APP}"

USER $APP_USER
WORKDIR ${APP}

CMD ["./app"]
