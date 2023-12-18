## Build Stage ##############################################################
FROM rust:1.73.0 as builder

# Trigger a build for just the dependencies
# to better cache cargo rebuilds.
WORKDIR /build
RUN USER=root cargo new --lib common
RUN USER=root cargo new --bin converter

WORKDIR /build/common
COPY ./common/Cargo.toml ./Cargo.toml
RUN cargo build --release

WORKDIR /build/converter
COPY ./api/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Now build the real thing
ADD ./converter ./
ADD ./common /build/common
RUN rm ./target/release/deps/converter*
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

COPY --from=builder /build/api/target/release/converter "${APP}/converter"
RUN chown -R "$APP_USER:$APP_USER" "${APP}"

USER $APP_USER
WORKDIR ${APP}

CMD ["./converter"]
