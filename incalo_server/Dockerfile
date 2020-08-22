FROM rust:alpine AS builder

RUN apk add --no-cache build-base

WORKDIR /usr/src/incalo
COPY . .

RUN cargo build --release

FROM scratch

WORKDIR /usr/local/bin/incalo
COPY --from=builder /usr/src/incalo/target/release/incalo-server .

CMD ["./incalo-server"]
