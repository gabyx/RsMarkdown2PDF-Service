ARG COMPONENT="api"
ARG SRC_DIR="/build/components"

## Build Stage ##############################################################
FROM rust:1.73.0 as builder
ARG COMPONENT
ARG SRC_DIR

# Trigger a build for just the dependencies
# to better cache cargo rebuilds.
WORKDIR "$SRC_DIR"
RUN cargo new --lib common && \
    cargo new --bin $COMPONENT

WORKDIR "$SRC_DIR/common"
COPY ./common/Cargo.toml ./Cargo.toml
RUN cargo build --release

WORKDIR "$SRC_DIR/$COMPONENT"
COPY ./$COMPONENT/Cargo.toml ./Cargo.toml
RUN cargo build --release

# Now build the real thing
WORKDIR "$SRC_DIR/$COMPONENT"
RUN rm -rf "$SRC_DIR/$COMPONENT/src" \
    rm -rf "$SRC_DIR/common/src"
ADD ./$COMPONENT "$SRC_DIR/$COMPONENT"
ADD ./common "$SRC_DIR/common"

RUN rm ./target/release/deps/$COMPONENT* || true
RUN rm ./target/release/deps/common* || true
RUN cargo build --release

## Service Stage ##############################################################
FROM ubuntu:latest
ARG COMPONENT
ARG SRC_DIR

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

COPY --from=builder "$SRC_DIR/$COMPONENT/target/release/$COMPONENT" "${APP}/app"
RUN chown -R "$APP_USER:$APP_USER" "${APP}"

USER $APP_USER
WORKDIR ${APP}

CMD ["./app"]
