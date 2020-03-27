FROM rust:1.42.0 as cargo-build

COPY migrations migrations
COPY proto proto
COPY src src
COPY build.rs build.rs
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN cargo build --release
RUN cargo install --path . --verbose

FROM debian:10.3-slim

COPY --from=cargo-build /home/rust/.cargo/bin/lieferemma /usr/local/bin/lieferemma

ENV RUST_LOG=info

CMD ["lieferemma"]
