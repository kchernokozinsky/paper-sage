FROM rust:slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy main.rs and copy actual source code
RUN rm src/main.rs
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 app

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/paper-sage /usr/local/bin/

# Create results directory
RUN mkdir -p /app/results && chown -R app:app /app

# Switch to app user
USER app

# Expose port (if needed for web interface in the future)
EXPOSE 8080

# Default command
CMD ["paper-sage", "--help"] 