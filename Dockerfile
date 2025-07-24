FROM alpine AS builder

WORKDIR /server

ADD . .
RUN apk add --no-cache rustup build-base && /usr/bin/rustup-init -y --default-toolchain nightly-unknown-linux-musl --profile minimal && cd server && ~/.cargo/bin/cargo build --release

FROM alpine
EXPOSE 25565

COPY --from=builder /server/server/target/release/oxide /app/oxide

CMD ["/app/oxide"]
