# Build stage
FROM rust:1.74-slim AS builder
WORKDIR /app

COPY . .
RUN cargo build --release

# Server stage
FROM rust:1.74-slim AS server
WORKDIR /app

COPY --from=builder /app/target/release/quotes-http-server .
RUN chmod +x quotes-http-server

EXPOSE $PORT
CMD ["/app/quotes-http-server"]
