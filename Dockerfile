FROM rust as builder
COPY . .
RUN cargo install --path .

# TODO try alpine before going GCP
FROM debian:buster-slim 
# RUN apt-get update && apt-get install -y extra-runtime-dependencies
COPY --from=builder /usr/local/cargo/bin/todo-service /usr/local/bin/todo-service
CMD ["todo-service"]
