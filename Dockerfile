FROM rust:1.78 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release --locked

RUN ls /usr/src/app/target/release

# FROM debian:buster-slim
# RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
# COPY -–from=builder /usr/src/app/target/release/light-http-server /usr/local/bin/

CMD ["/usr/src/app/target/release/light-http-server"]
