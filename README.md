<div align="center">

<img src="assets/phink.png" alt="phink" width="250"/>
</br>
</br>


![Build Status](https://github.com/srlabs/phink/actions/workflows/rust.yml/badge.svg)
[![License](https://img.shields.io/github/license/srlabs/phink)](https://github.com/srlabs/phink/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/srlabs/phink/status.svg)](https://deps.rs/repo/github/srlabs/phink)
![Discord](https://img.shields.io/discord/1276519988349374587?label=Discord)
</div>
</br>

**Phink** is a blazing-fast⚡, property-based, coverage-guided fuzzer for ink! smart contracts. It enables developers to
embed inviolable properties into their smart contract testing workflows, equipping them with automatic tools to detect
vulnerabilities and ensure contract reliability before deployment.
If you have any question, would like to share feedback, discuss features, and connect with other ink! developers, join
our [Discord community](https://discord.gg/gAahQMGE)

> ⚠️ This project is actively under development with new features and improvements being made regularly. Contributions
> and feedback are welcome!

## Install

### Building from source

If you prefer to install Phink manually, follow these steps:

```bash
git clone https://github.com/kevin-valerio/phink
cd phink/
cargo install --force ziggy cargo-afl honggfuzz grcov cargo-contract --locked 
cargo afl config --build --plugins --verbose --force # don't use `--plugins` if you're on macOS
sudo cargo-afl afl system-config
cargo build --release
./target/release/phink --help
```

### Using Docker

Alternatively, you can use Docker to set up and run Phink without needing to manually install dependencies. Detailed
instructions are available in [README.Docker.md](README.Docker.md).

To build the Docker image:

```bash
docker build -t phink .
```

## Usage

### Docker Usage

To use Phink via Docker, you can run:

```bash
docker run --rm phink
```

For instrumenting a specific contract:

```bash
docker run --rm phink instrument path/to/ink_contract
```

_Refer to [README.Docker.md](README.Docker.md) for more detailed instructions on using Phink with Docker._

### Manual Usage

```bash
phink instrument path/to/ink_contract
phink fuzz /tmp/ink_fuzzed_Bb9Zp # you can get this path by reading the output of the previous command
```  

## Example

#### Creating an invariant

Below are some invariants created for the [dns](https://github.com/kevin-valerio/phink/blob/main/sample/dns/lib.rs)
contract.

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
        let FORBIDDEN_NUMBER = 69;
        assert_ne!(self.dangerous_number, FORBIDDEN_NUMBER);
    }
}
```

#### Catching an invariant

```bash
phink execute output/phink/crashes/<timestamp>/<id:000x:seed> \
      /tmp/ink_fuzzed_<random_string>/
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
thread 'main' panicked at src/fuzzer/bug.rs:83:9:

Job is done! Please, don't matter the backtrace below/above 🫡
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
- [x] Enabling multi-contract fuzzing and cross-contract interactions
- [ ] Development of a custom fuzzing dashboard (default options: Ziggy/AFL++/Honggfuzz dashboard)
- [ ] Creation of default invariants common to every contract  (_research needed_)
- [ ] Provision of a specified on-chain state  (_research needed_)
- [ ] Implementation of a snapshot-based fuzzing approach  (_research needed_)
- [ ] Extraction of seeds and constants from the codebase (_research needed_)
- [ ] Creation of LLM-based invariants using [rust-llama](https://github.com/mdrokz/rust-llama.cpp) (_research needed_)
