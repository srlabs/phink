# Phink Docker Image

This README provides instructions for building and running the Phink Docker image. This Dockerfile is designed to create
a streamlined environment for
building and using Phink.

## Dockerfile Overview

### Build Stage

This Dockerfile setups a build environment using the `rust:1.75-slim-bookworm` base image. This stage
includes:

1. **Installing Dependencies**: Essential build tools and libraries are installed to ensure the Rust environment has
   everything it needs to compile Phink and its dependencies.
    - `curl`, `git`, `build-essential`, and `wget` are installed.
    - LLVM 19 and Clang 19 are installed

2. **Setting Up Rust**:
    - This Dockerfile sets Rust to nightly version (`nightly`) to ensure compatibility with Phink's
      codebase.
    - Additional Rust components and tools such as `rust-src`, `cargo-afl`, `honggfuzz`, `grcov`, and `cargo-contract`
      are installed to support fuzzing and coverage instrumentation.

3. **Cloning and Building Phink**:
    - Phink is cloned from the `srlabs/phink`
    - The project is built in release mode.

4. **Setting the Entry Point**:
    - The entry point is set to execute the Phink binary directly

5. **Default Command**:
    - The default command provided is for instrumenting a sample contract located in `sample/multi-contract-caller/`.

### How to Use the Docker Image

1. **Building the Docker Image**:
   ```bash
   docker build -t phink .
   ```

2. **Running Phink**:
   To run Phink with the default command (running the default example):
   ```bash
   docker run --rm phink
   ```

   To specify a different contract for instrumentation:
   ```bash
   docker run --rm phink instrument <path_to_your_contract>
   ```

   To fuzz your instrumented ink! smart-contract:
   ```bash
   docker run --rm phink fuzz  
   ```

### Notes

- **No files copied into `/bin`**: The Dockerfile intentionally avoids copying files into `/bin` to keep the
  `phink.toml` configuration and `sample/` directory accessible for user interaction within the container.

