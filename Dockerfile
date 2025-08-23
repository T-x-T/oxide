FROM git.thetxt.io/thetxt/runner-base:latest AS builder

WORKDIR /server

ADD . .
RUN cd server && ~/.cargo/bin/cargo build --release

FROM alpine
EXPOSE 25565

COPY --from=builder /server/server/target/release/oxide /app/oxide
WORKDIR /app
CMD ["/app/oxide"]
