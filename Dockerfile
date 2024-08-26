# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.75-slim-bookworm AS builder

# Install dependencies and LLVM 19
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    git \
    wget \
    gnupg \
    software-properties-common \
    ca-certificates \
    && wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - \
    && echo "deb http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-19 main" >> /etc/apt/sources.list.d/llvm.list \
    && apt-get update && apt-get install -y --no-install-recommends \
    llvm-19 \
    llvm-19-dev \
    clang-19 \
    libclang-19-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set up Rust
RUN rustup default nightly-2024-08-13 \
    && rustup component add rust-src \
    && cargo install --force ziggy cargo-afl honggfuzz grcov cargo-contract

# Clone and build the project
WORKDIR /phink
RUN git clone https://github.com/kevin-valerio/phink . \
    && cargo update \
    && cargo afl config --build --plugins --verbose --force \
    && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary and necessary files from the builder stage
COPY --from=builder /phink/target/release/phink /usr/local/bin/phink
COPY --from=builder /phink/phink.toml /phink/phink.toml
COPY --from=builder /phink/sample /phink/sample

WORKDIR /phink

# Set the entrypoint
ENTRYPOINT ["phink"]

# Default command: instrument a contract
CMD ["instrument", "sample/multi-contract-caller/"]