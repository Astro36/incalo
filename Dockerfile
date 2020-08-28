FROM rust:alpine AS builder

RUN apk add --no-cache build-base openssl-dev

WORKDIR /usr/src/incalo
COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache tzdata
ENV TZ Asia/Seoul

WORKDIR /usr/local/bin/incalo
COPY --from=builder /usr/src/incalo/target/release/incalo-server .

CMD ["./incalo-server"]
