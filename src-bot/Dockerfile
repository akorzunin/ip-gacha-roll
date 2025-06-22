ARG RUST_VERSION=1.87.0
ARG APP_NAME=src-bot

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine3.22 AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git \
    openssl-dev openssl-libs-static libc6-compat

ENV OPENSSL_STATIC=yes \
    OPENSSL_LIB_DIR=/usr/lib \
    OPENSSL_INCLUDE_DIR=/usr/include

COPY shared /shared
RUN --mount=type=bind,source=src-bot/src,target=src \
    --mount=type=bind,source=src-bot/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=src-bot/Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build \
    --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.22 AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

EXPOSE 8000

CMD ["/bin/server"]
