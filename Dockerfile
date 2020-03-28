FROM clux/muslrust:1.42.0-stable as cargo-build

COPY . .

RUN cargo install --path .

FROM alpine:latest



COPY --from=cargo-build /root/.cargo/bin/lieferemma /usr/local/bin/lieferemma

ENV RUST_LOG=info

CMD ["lieferemma"]
