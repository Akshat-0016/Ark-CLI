FROM rust:1.97 AS builder

WORKDIR /app

COPY assistant-core ./assistant-core

WORKDIR /app/assistant-core

RUN cargo build --release

FROM debian:trixie-slim

COPY --from=builder /app/assistant-core/target/release/ark /usr/local/bin/ark

CMD ["ark"]
