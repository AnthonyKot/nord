# Use the official Rust image as a builder
FROM rust:1.70 as builder
WORKDIR /usr/src/nord
COPY . .
RUN cargo install --path .

# Use the Debian image for the runtime environment
FROM debian:bullseye
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/nord/target/release/nord .
COPY --from=builder /usr/src/nord/Default.toml .

EXPOSE 8000
CMD ["./nord"]