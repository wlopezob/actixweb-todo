FROM rust:alpine3.19 AS builder
RUN apk add musl-dev jq --no-cache
WORKDIR /src
COPY . .
RUN cargo build --release

FROM alpine:3.19
ARG APPNAME
EXPOSE 8080
WORKDIR /app
COPY --from=builder /src/target/release/todo .
ENTRYPOINT [ "./todo" ]