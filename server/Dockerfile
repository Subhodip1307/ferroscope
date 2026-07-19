FROM rust:1.93.1-alpine3.23 AS builder
WORKDIR /code/
COPY . .
RUN cargo build --release

FROM alpine:3.21 AS runner
WORKDIR /code
COPY --from=builder /code/target/release/server /code/server
COPY --from=builder /code/target/release/fero /usr/local/bin/fero
RUN addgroup -S scope && adduser -S scope -G scope \
 && chmod +X /code/server /usr/local/bin/fero
CMD [ "./server" ]