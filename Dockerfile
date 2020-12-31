FROM rust:latest as cargo-build

WORKDIR /usr/src/rust-http-kafka-bridge

COPY ./Cargo.toml ./Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/rust-http-kafka-bridge*

COPY . .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=cargo-build /usr/src/rust-http-kafka-bridge/target/release/rust-http-kafka-bridge /usr/local/bin/rust-http-kafka-bridge

CMD ["rust-http-kafka-bridge"]
