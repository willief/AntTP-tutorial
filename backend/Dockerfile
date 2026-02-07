FROM rust:latest AS builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() { println!(\"dummy\"); }" > src/main.rs && \
    cargo build --release && \
    rm -rf target/release/deps/anttp*

# Copy actual source
COPY src ./src

# Build for real and verify
RUN cargo build --release && \
    ls -lh target/release/anttp-tutorial-backend && \
    file target/release/anttp-tutorial-backend

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/anttp-tutorial-backend /app/anttp-tutorial-backend

# Verify binary was copied
RUN ls -lh /app/anttp-tutorial-backend && \
    chmod +x /app/anttp-tutorial-backend

EXPOSE 8080

# Add error output
CMD ["/bin/sh", "-c", "/app/anttp-tutorial-backend 2>&1 || (echo 'Backend failed to start' && exit 1)"]
