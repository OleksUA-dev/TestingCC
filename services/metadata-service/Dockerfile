# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml ./
COPY crates ./crates
COPY services/metadata-service ./services/metadata-service

# Build the application
RUN cargo build --release --package metadata-service

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/metadata-service /app/metadata-service

# Copy migrations
COPY services/metadata-service/migrations /app/migrations

# Expose port
EXPOSE 8001

# Run the application
CMD ["/app/metadata-service"]
