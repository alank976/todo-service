FROM clux/muslrust:stable

# RUN USER=root cargo new --bin todo-service
# WORKDIR /todo-service
# COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release --target x86_64-unknown-linux-musl

# # build again with code change (if any)
# COPY src ./src
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

# runnable
FROM alpine
RUN USER=root adduser -D -u 10001 dummy
COPY --from=builder /usr/local/cargo/bin/todo-service /usr/local/bin/todo-service
USER dummy
CMD ["todo-service"]
