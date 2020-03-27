FROM ekidd/rust-musl-builder:1.42.0 as cargo-build

COPY . .

RUN cargo install --path .

FROM alpine:latest



COPY --from=cargo-build /home/rust/.cargo/bin/lieferemma /usr/local/bin/lieferemma

ENV RUST_LOG=info

CMD ["lieferemma"]
