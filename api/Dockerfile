FROM rust:1.73.0 as builder

RUN USER=root cargo new --bin api
WORKDIR ./api

# Trigger a build for just the dependencies
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/api*
# Now build the real thing
RUN cargo build --release


FROM ubuntu:latest
ARG APP=/usr/src/app
EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

RUN apt update && apt-get install -y libpq5 && apt-get clean && rm -rf /var/cache/apt/archives /var/lib/apt/lists/*

COPY --from=builder /api/target/release/api ${APP}/api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./api"]