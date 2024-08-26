# Build stage
FROM rust:1.75-slim-bookworm AS builder

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    git \
    wget \
    gnupg \
    software-properties-common \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install LLVM 19
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - \
    && echo "deb http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-19 main" >> /etc/apt/sources.list.d/llvm.list \
    && apt-get update && apt-get install -y --no-install-recommends \
    llvm-19 \
    llvm-19-dev \
    clang-19 \
    libclang-19-dev \
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

# Set the entrypoint
# We don't COPY into /bin because we want 1. the `phink.toml` and 2. `sample/` for the user to play with it
ENTRYPOINT ["./target/debug/phink"]

# Default command : we just instrument a contract
CMD ["instrument", "sample/multi-contract-caller/"]