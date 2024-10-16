## Getting Started with Phink

### Installation

#### System Requirements

To successfully install and run Phink, ensure your system meets the following requirements:

- **Operating System:**
    - **Linux:** **Recommended** for compatibility.
    - **macOS:** *Not recommended* as it doesn’t support some AFL++ plugins.
    - **Windows:** *Untested.*

- **Rust:**
    - **Version:** Rust *nightly*
    - **Current Compatibility:** `cargo 1.83.0-nightly (ad074abe3 2024-10-04)`

#### Installation Guide

You can install Phink by building it from the source or by using Docker. Choose the method that best suits your setup.

##### Building from Source

1. **Clone the Repository**
   ```bash
   git clone https://github.com/srlabs/phink && cd phink/
   ```

   You can also use:
   ```bash 
   cargo +nightly install --git https://github.com/srlabs/phink
   ```

2. **Install Dependencies**
   ```bash
   cargo install --force ziggy cargo-afl honggfuzz grcov cargo-contract --locked
   ```

3. **Configure AFL++**
   ```bash
   cargo afl config --build --plugins --verbose --force
   sudo cargo-afl afl system-config
   ```

4. **Build Phink**
   ```bash
   cargo build --release
   ```

5. **Run Phink**
   ```bash
   ./target/release/phink --help
   phink --help # if installed via `cargo install`
   ```

##### Using Docker

1. **Build the Docker Image**
   ```bash
   docker build -t phink .
   ```

For detailed Docker instructions, refer
to [README.Docker.md](https://github.com/srlabs/phink/blob/main/README.Docker.md).

#### Basic Workflow

1. **Instrument the Contract**
    - Use Phink to instrument your ink! smart contract for fuzzing.

2. **Configure Fuzzing Parameters**
    - Edit the `phink.toml` file to set paths, deployment settings, and fuzzing options according to your project needs.

3. **Run Fuzzing**
    - Execute fuzzing with your configured settings to identify vulnerabilities early in the development cycle.