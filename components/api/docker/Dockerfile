## Build Stage ##############################################################
FROM rust:1.73.0 as builder

# Trigger a build for just the dependencies
# to better cache cargo rebuilds.
WORKDIR /build
RUN USER=root cargo new --lib common
RUN USER=root cargo new --bin api

WORKDIR /build/common
COPY ./common/Cargo.toml ./Cargo.toml
RUN cargo build --release

WORKDIR /build/api
COPY ./api/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Now build the real thing
ADD ./api ./
ADD ./common /build/common
RUN rm ./target/release/deps/api*
RUN rm ./target/release/deps/common*
RUN cargo build --release

## Service Stage ##############################################################
FROM ubuntu:latest
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

COPY --from=builder /build/api/target/release/api "${APP}/api"
RUN chown -R "$APP_USER:$APP_USER" "${APP}"

USER $APP_USER
WORKDIR ${APP}

CMD ["./api"]
