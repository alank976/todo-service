FROM rust as builder

# install pre-requisites for cross-compile to musl
RUN apt-get update
RUN apt-get -y install musl-tools  && rm -rf /var/lib/apt/lists/*
COPY install-openssl.sh .
RUN chmod +x install-openssl.sh && ./install-openssl.sh
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_STATIC=true
ENV OPENSSL_DIR=/openssl-musl
RUN rustup target add x86_64-unknown-linux-musl
# TODO: move above steps as a github action for optimizing CI time

# build once for docker cache
# TODO: use actions/cache to restore from cache when Cargo.lock is unchanged
# See also: https://github.com/actions/cache 
RUN USER=root cargo new --bin todo-service
WORKDIR /todo-service
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --target x86_64-unknown-linux-musl

# build again with code change (if any)
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# runnable
FROM alpine
RUN USER=root adduser -D -u 10001 dummy
COPY --from=builder /usr/local/cargo/bin/todo-service /usr/local/bin/todo-service
USER dummy
CMD ["todo-service"]
