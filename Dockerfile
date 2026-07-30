FROM debian:trixie-slim

WORKDIR /app

COPY assistant-core/target/release/ark /usr/local/bin/ark

CMD ["ark"]
