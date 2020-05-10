FROM ekidd/rust-musl-builder:1.42.0 as backend-builder
WORKDIR /home/rust/src
COPY --chown=rust:rust . .
RUN cargo build

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=backend-builder /home/rust/src/target/x86_64-unknown-linux-musl/debug/artell ./
ENTRYPOINT ["./artell"]
