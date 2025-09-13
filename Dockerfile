FROM git.thetxt.io/thetxt/runner-base:latest AS builder

WORKDIR /

ADD . .
RUN ~/.cargo/bin/cargo build --bin server --release

FROM alpine
EXPOSE 25565

COPY --from=builder /server/target/release/server /app/server
WORKDIR /app
CMD ["/app/server"]
