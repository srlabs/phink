# Use the latest Debian as the base image
FROM debian:latest

WORKDIR /phink

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    llvm-18.1.8 \
    && apt-get clean

# Install Rust and Cargo (Nightly toolchain)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

# Install additional cargo tools
RUN cargo install --force ziggy cargo-afl honggfuzz grcov cargo-contract --locked

# Configure AFL
RUN cargo afl config --build --plugins --verbose --force


# Set the default command to run cargo with a configurable target
CMD ["cargo", "run", "--", "instrument", "sample/multi-contract-caller/"]
