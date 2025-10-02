# Rust 1.81 slim
FROM rust:1.81-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    python3 \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy project
COPY Cargo.toml ./
COPY src ./src

# Build RSI binary
RUN cargo build --release

# Run RSI service
CMD ["./target/release/rsi_service"]
