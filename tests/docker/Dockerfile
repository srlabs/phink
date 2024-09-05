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
RUN rustup default nightly \
    && rustup component add rust-src \
    && cargo install --force ziggy cargo-afl honggfuzz grcov cargo-contract \
    && rustup component add clippy

# Clone and build the project
WORKDIR /phink

RUN git clone https://github.com/srlabs/phink . \
    && cargo update \
    && cargo build --release

RUN cargo afl config --build --plugins --verbose --force

RUN curl https://raw.githubusercontent.com/AFLplusplus/AFLplusplus/stable/afl-system-config > afl-system-config.sh
RUN chmod +x afl-system-config.sh && bash afl-system-config.sh

RUN cp target/release/phink /usr/local/bin/phink

WORKDIR /phink/sample
RUN chmod 777 build.sh && bash build.sh

WORKDIR /phink
ENV RUST_BACKTRACE=1
CMD ["cargo", "test", "--no-fail-fast", "--", "--show-output", "--test-threads=1"]