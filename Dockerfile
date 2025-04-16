# Prepare build dependencies
FROM rust:1.82 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Build the project
COPY . .
RUN cargo build --release

# Prepare runtime dependencies
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/gcloud-axum-test /usr/local/bin/gcloud-axum-test

# Run the application
EXPOSE 8080
CMD ["/usr/local/bin/gcloud-axum-test"]
