FROM clux/muslrust as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/super_continent_core /application/super_continent_core

EXPOSE 55555

WORKDIR /application

CMD ["./super_continent_core"]