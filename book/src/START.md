## Getting started with Phink

### Installation

#### System requirements

To successfully install and run Phink, ensure your system meets the following requirements:

- **Operating System:**
    - **Linux:** **Recommended** for compatibility
    - **macOS:** *Not recommended* as it doesn’t support some AFL++ plugins
    - **Windows:** *Untested*

- **Rust:**
    - **Version:** Rust *nightly*
    - **Current Compatibility:** `cargo 1.83.0-nightly (ad074abe3 2024-10-04)`

#### Installation guide

You can install Phink by building it from the source or by using Docker. Choose the method that best suits your setup
and IT environment. Let's jump right into it!

##### Building from source

Follow these 5 easy steps:

1. **Clone the Repository**
   ```bash
   git clone https://github.com/srlabs/phink && cd phink/
   ```

   You can also run the following command:
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
   cargo run -- help  
   ```

##### Using Docker

1. **Build the Docker Image**
   To build the **Phink Docker image**, run the following command in your terminal:

   ```bash
   docker build -t phink .
   ```

For detailed Phink Docker installation instructions, refer
to [README.Docker.md](https://github.com/srlabs/phink/blob/main/README.Docker.md).

### Basic workflow

Follow these three high-level steps:

1. **Instrument the contract**
    - Use Phink to instrument your ink! smart contract for fuzzing

2. **Configure fuzzing parameters**
    - Edit the `phink.toml` file to set paths, deployment settings, and fuzzing options according to your project needs

3. **Run your fuzzing campaign**
    - Execute fuzzing with your configured settings to identify vulnerabilities early in the development cycle