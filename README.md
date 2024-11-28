<div align="center">

<img src="https://raw.githubusercontent.com/srlabs/phink/refs/heads/main/assets/phink.png" alt="phink" width="250"/>
</br>
</br>


![Docker](https://github.com/srlabs/phink/actions/workflows/docker.yml/badge.svg)
![Tests](https://github.com/srlabs/phink/actions/workflows/rust.yml/badge.svg)
[![Latest Release](https://img.shields.io/crates/v/phink.svg)](https://crates.io/crates/phink)
[![License](https://img.shields.io/github/license/srlabs/phink)](https://github.com/srlabs/phink/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/srlabs/phink/status.svg)](https://deps.rs/repo/github/srlabs/phink)
![Discord](https://img.shields.io/discord/1276519988349374587?label=Discord)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://srlabs.github.io/phink)

</div>
</br>

**Phink** is a blazing-fast⚡, property-based, coverage-guided fuzzer for ink! smart contracts. It enables developers to
embed inviolable properties into their smart contract testing workflows, equipping them with automatic tools to detect
vulnerabilities and ensure contract reliability before deployment. Please, read
the [documentation](https://srlabs.github.io/phink/) in order to properly
use Phink.

![Dashboard GIF](assets/dashboard.gif)

For documentation, visit our [**documentation site here**](https://srlabs.github.io/phink/). If you have any question,
feedback, features suggestion, join our [Discord](https://discord.gg/gAahQMGE).

## Install

**Important note:** Phink requires Cargo and can only be used by running it from source code. You can't install it via
`cargo install`. Since the fuzzing harness needs to be compiled with instrumentation and is included within the tool,
you must use `cargo run` with the source code to execute Phink.

### Building from source

#### Requirements

##### Install the requirements, configure AFL++ plugins and adapt the system configs

```bash
cargo install --force ziggy cargo-afl honggfuzz grcov cargo-contract --locked 
cargo afl config --build --plugins --verbose --force # don't use `--plugins` if you're on macOS
sudo cargo-afl afl system-config
```

##### Install Phink

```bash
git clone https://github.com/srlabs/phink && cd phink;
cargo run -- help
```

##### Install Phink via Docker

Alternatively, you can use Docker to set up and run Phink without needing to manually install dependencies. Detailed
instructions are available in [README.Docker.md](README.Docker.md). To build the Docker image:

```bash
docker build -t phink .
```

## Usage

### Via normal installation

```bash
cargo run -- instrument path/to/ink_contract
cargo run -- generate-seed path/to/ink_contract #optional but recommended
cargo run -- fuzz  
```  

### If installed via Docker

_Refer to [README.Docker.md](README.Docker.md) for more detailed instructions on using Phink with Docker._

## Example

#### Adding some invariants

Below are some invariants created for the [dns](https://github.com/kevin-valerio/phink/blob/main/sample/dns/lib.rs)
contract.

1. Add the `phink` feature to your `Cargo.toml`

```toml
[features]
phink = []
```

2. Create your invariants as below:

```rust
#[cfg(feature = "phink")]
#[ink(impl)]
impl DomainNameService {
    // This invariant ensures that `domains` doesn't contain the forbidden domain that nobody should regsiter 
    #[ink(message)]
    #[cfg(feature = "phink")]
    pub fn phink_assert_hash42_cant_be_registered(&self) {
        for i in 0..self.domains.len() {
            if let Some(domain) = self.domains.get(i) {
                // Invariant triggered! We caught an invalid domain in the storage...
                assert_ne!(domain.clone().as_mut(), FORBIDDEN_DOMAIN);
            }
        }
    }

    // This invariant ensures that nobody registed the forbidden number
    #[ink(message)]
    #[cfg(feature = "phink")]
    pub fn phink_assert_dangerous_number(&self) {
        let forbidden_number = 42;
        assert_ne!(self.dangerous_number, forbidden_number);
    }
}
```

#### Catching an invariant

```bash
cargo run -- execute output/phink/crashes/<timestamp>/<id:000x:seed>  
```

Below, the trace after executing the crash:

```
🚀 Now fuzzing `/tmp/ink_fuzzed_XqUCn/target/ink/transfer.json` (5H31F11yQUkqugbgC7ur4rT2WLKSkZKAZUfcmHkKoLkaRaZ4)!
🤯 An invariant got caught! Let's dive into it
🫵  This was caused by `phink_assert_cannot_transfer_1337`
🎉 Find below the trace that caused that invariant
🌱 Executing new seed

+---------+-------------------------------------------------------------------+
| Message | Details                                                           |
+---------+-------------------------------------------------------------------+
| pay_me  |  ⛽️ Gas required : Weight(ref_time: 591391866, proof_size: 28781) |
|         | 🔥 Gas consumed : Weight(ref_time: 582570121, proof_size: 12443)  |
|         | 💾 Storage deposit : StorageDeposit::Charge(0)                    |
|         | 💸 Message was payable, and 1809739 units were transferred        |
+---------+-------------------------------------------------------------------+
```

#### List of samples

You can find various sample ink! smart-contracts in the `sample/` directory. For detailed descriptions of these samples
and
instructions on how to instrument them for testing with Phink, please refer to the [sample's README](sample/README.md)
file.

## Features and upcoming ideas

- [x] Integration of a custom runtime, using a generic one by default
- [x] Invariants-based fuzzing
- [x] Detection of incorrect arithmetic, reentrancy, and panic handlers
- [x] Handling of ink! specific encoding and constructors
- [x] Automatic contract instantiation
- [x] Crafting multiple messages in a single transaction
- [x] Visualization of ink! contract coverage
- [x] Proper binary usage
- [x] Benchmarking the fuzzing campaign and generating statistics
- [x] Enabling multi-contract fuzzing and cross-contract interactions
- [x] Seed extraction out of unit and end-to-end tests
- [x] Development of a custom fuzzing dashboard (default options: Ziggy/AFL++/~~Honggfuzz~~ dashboard)
- [ ] Implementation of a snapshot-based fuzzing approach  (_research needed_)
- [ ] Migrate from pallet-contract to pallet-revive and targetting PolkaVM  (_research needed_)
