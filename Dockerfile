FROM rust:1.81.0 AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN cargo build --release

FROM ubuntu:25.04
RUN apt-get update && apt install -y libssl3
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/loki_to_slack_notification /loki_to_slack_notification
RUN chmod +x /loki_to_slack_notification
CMD ["/loki_to_slack_notification"]
