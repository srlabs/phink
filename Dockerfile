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
    clang-19 \
    libclang-19-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set up Rust
RUN rustup toolchain install nightly \
    && rustup default nightly \
    && rustup component add rust-src \
    && rustup component add clippy \
    && rustup component add rust-src \
    && cargo install --force ziggy cargo-afl honggfuzz grcov cargo-contract

# Clone and build the project
WORKDIR /phink

RUN git clone https://github.com/srlabs/phink .
RUN cargo afl config --build --plugins --verbose --force
RUN cargo build --release

RUN curl https://raw.githubusercontent.com/AFLplusplus/AFLplusplus/stable/afl-system-config > afl-system-config.sh
RUN chmod +x afl-system-config.sh && bash afl-system-config.sh
RUN cp target/release/phink /usr/local/bin/phink

WORKDIR /phink
# If nothing is provided, we just start an instrumentation of `dummy`
CMD ["cargo", "run", "--", "instrument", "sample/dummy/"]