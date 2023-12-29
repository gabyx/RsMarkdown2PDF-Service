ARG COMPONENT="api"
ARG ROOT_DIR="/build"

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
COPY --from=planner /$ROOT_DIR/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
COPY --from=planner "$ROOT_DIR/Cargo.toml" ./
RUN cargo build --release -p "$COMPONENT"

## Service Stage ##############################################################
FROM ubuntu:latest
ARG COMPONENT
ARG ROOT_DIR

ARG APP=/usr/src/app
EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd "$APP_USER" \
    && useradd -g "$APP_USER" "$APP_USER" \
    && mkdir -p "${APP}"

RUN apt update && \
    apt-get install -y libpq5 && \
    apt-get clean && \
    rm -rf /var/cache/apt/archives /var/lib/apt/lists/*

COPY --from=builder "$ROOT_DIR/target/release/$COMPONENT" "${APP}/app"
RUN chown -R "$APP_USER:$APP_USER" "${APP}"

USER $APP_USER
WORKDIR ${APP}

CMD ["./app"]
