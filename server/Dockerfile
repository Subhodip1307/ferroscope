FROM rust:1.93.1-alpine3.23 AS builder
WORKDIR /code/

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN cargo build --release

FROM alpine:3.23 AS runner
WORKDIR /code
COPY --from=builder /code/target/release/ferroscope-server /code/ferroscope-server
COPY --from=builder /code/target/release/fero /usr/local/bin/fero
RUN addgroup -S scope && adduser -S scope -G scope \
 && chmod +X /code/ferroscope-server /usr/local/bin/fero
CMD [ "./ferroscope-server" ]