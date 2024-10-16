## Writing Properties for ink! Contracts

### Adding Properties

#### Inside your `Cargo.toml`

First, you need to add the `phink` feature to your Cargo.toml, such as:

```toml
[features]
phink = []
```

#### Inside your `file.rs`

Then, you can use the following example to create invariants. You need to create another `impl` if your contract, and to
put
it under the feature of `phink`. Use `assert!` or `panic!` for your properties.

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

## Running Phink

### 1. Instrument the Contract

Run the following command to instrument your ink! smart contract, enabling it for fuzzing:

```sh
phink instrument my_contract/
```

This step modifies the contract to include necessary hooks for Phink's fuzzing process. It creates a fork of the
contract, so you don't have to make a copy before.

### 2. Run the fuzzer

After **instrumenting** your contract and **writing** properties and **configuring** your `phink.toml`, execute the
fuzzing process:

```sh
phink fuzz
```

This command runs your fuzzing tests based on the configuration set in your `phink.toml` file. A user interface should
appear.

## Analyzing Results

### Crashes

In case of crashes, you should see something like

### Coverage

**IMPORTANT: this feature is in alpha.**

#### Generating a coverage report

Generate coverage reports to analyze which parts of the contract were tested:

```sh
phink coverage my_contract/
```

Some HTML files should then be generated in the path you've configured inside your `phink.toml`.

#### Interpreting Coverage Reports

The coverage report provides a visual representation of tested code areas. Basically, the more green lines, the better. 