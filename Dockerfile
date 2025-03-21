FROM rust:1.81.0 AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/src/app

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/app/target/release/loki_to_slack_notification /

# Command to run the application
CMD ["./loki_to_slack_notification"]
