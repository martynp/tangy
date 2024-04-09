FROM rust:1.77.0-slim-bookworm as builder

COPY . /workspace/
RUN cd /workspace/ && cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /workspace/target/release/tangy /usr/bin
RUN mkdir /keys

CMd /usr/bin/tangy --dir /keys --address 0.0.0.0 --port 8000