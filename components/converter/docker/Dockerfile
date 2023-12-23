ARG COMPONENT="converter"
ARG ROOT_DIR="/build"

## Base Stage ##############################################################
FROM rust:1.74 AS chef
ARG ROOT_DIR
RUN cargo install cargo-chef --locked
WORKDIR "$ROOT_DIR"

## Planner Stage ##############################################################
FROM chef as planner
COPY . .
RUN rm -rf rust-toolchain.toml
RUN cargo chef prepare --recipe-path recipe.json

## Planner Stage ##############################################################
FROM chef AS builder
ARG COMPONENT
ARG ROOT_DIR
COPY --from=planner /$ROOT_DIR/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN rm -rf rust-toolchain.toml
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
