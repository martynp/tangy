FROM rust:1.85.1-slim-bookworm AS builder

COPY . /workspace/
RUN cd /workspace/ && cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /workspace/target/release/tangy /usr/bin
RUN mkdir /keys

CMd /usr/bin/tangy --dir /keys --address 0.0.0.0 --port 8000
