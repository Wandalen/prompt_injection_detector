# Dockerfile for Prompt Injection Detector
# Multi-stage build for optimized production image

# Build stage
FROM rust:1.70-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml ./
COPY module ./module

# Build release binary
RUN cargo build --release --features full -p injection_server

# Runtime stage with NVIDIA CUDA support (optional)
FROM nvidia/cuda:11.8.0-runtime-ubuntu22.04

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/injection_server /app/injection_server

# Create directory for model cache
RUN mkdir -p /app/models

# Expose HTTP port
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info
ENV MODEL_CACHE_DIR=/app/models

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Run server
CMD ["/app/injection_server"]
