# Build stage
FROM --platform=$BUILDPLATFORM node:20-slim as builder

# Install Rust
RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config libssl-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /build
# Copy package files for caching
COPY package*.json ./
COPY frontend/package*.json ./frontend/
RUN npm install

# Copy rest of the source
COPY . .

# Build everything using our npm script
RUN npm run build

# Runtime stage
FROM --platform=$TARGETPLATFORM debian:bookworm-slim
WORKDIR /app

# Install only necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy built artifacts
COPY --from=builder /build/dist/static ./static
COPY --from=builder /build/target/release/conductor .

# Runtime configuration
ENV RUST_LOG=info
EXPOSE 5001

CMD ["./conductor"]