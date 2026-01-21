# Single stage - run from build location
FROM rust:latest

WORKDIR /app
COPY . .

# Build
RUN cargo build --release

# Expose port
EXPOSE 8000

# Run from the target directory (where it was built)
CMD ["./target/release/autonomi-rust-backend"]
