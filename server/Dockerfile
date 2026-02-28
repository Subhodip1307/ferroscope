FROM rust:1.93.1-alpine3.23 AS builder
WORKDIR /code/
COPY . .
RUN cargo build --release

FROM rust:1.93.1-alpine3.23 AS runner
WORKDIR /code
COPY --from=builder /code/target/release/server /code/server
RUN chmod +X /code/server
CMD [ "./server" ]