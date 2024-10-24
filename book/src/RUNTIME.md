# Runtime integration

Phink provides developers with the flexibility to customize their fuzzing environment through a simple interface. By
editing `contract/custom/custom.rs` and `contract/custom/preferences.rs`, developers can tailor the runtime storage and
contract initialization
processes to suit their testing needs. Have a clone of Phink if you want to modify the source code.

## Custom runtime storage

Phink allows developers to tailor the runtime environment by customizing the storage configuration. Let's
create some realistic testing scenarios!

### Example

```rust,ignore
impl DevelopperPreferences for Preferences {
    fn runtime_storage() -> Storage {
        let storage = RuntimeGenesisConfig {
            balances: BalancesConfig {
                balances: (0..u8::MAX) // Allocates substantial balance to accounts
                    .map(|i| [i; 32].into())
                    .collect::<Vec<_>>()
                    .iter()
                    .cloned()
                    .map(|k| (k, 10000000000000000000 * 2))
                    .collect(),
            },
            ..Default::default()
        }
            .build_storage()
            .unwrap();
        storage
    }
}
```

### Customization Points

- **`runtime_storage`:** This function is your gateway to defining any mocks or `RuntimeGenesisConfig` settings needed
  for your testing environment. Whether it's allocating funds, initializing storage items, or setting up custom
  storage, you can adjust these configurations to mirror your deployment scenarios closely. This flexibility allows
  you to test how your ink! smart contract behave in various simulated network states.

## Contract Initialization

The `on_contract_initialize` function can be adapted to execute additional initialization logic, such as uploading
supplementary contracts or handling dependencies.

### Usage Example

```rust,ignore
fn on_contract_initialize() -> anyhow::Result<()> {
    Contracts::bare_upload_code(
        AccountId32::new([1; 32]),
        fs::read("adder.wasm")?,
        None,
        Determinism::Enforced,
    );
    Ok(())
}
```

### Customization points

- **`runtime_storage`:** Use this function as your gateway to defining any mocks or `RuntimeGenesisConfig` settings
  needed
  for your testing environment. Whether it's allocating funds, initializing storage items, or setting up custom
  storage, adjust these configurations to mirror your deployment scenarios closely. This flexibility lets
  you test how your ink! smart contract behaves in various simulated network states.

## Contract initialization

The `on_contract_initialize` function can be adapted to execute additional initialization logic, such as uploading
supplementary contracts or handling dependencies.

### Example

```rust,ignore
fn on_contract_initialize() -> anyhow::Result<()> {
    Contracts::bare_upload_code(
        AccountId32::new([1; 32]),
        fs::read("adder.wasm")?,
        None,
        Determinism::Enforced,
    );
    Ok(())
}
```

### Customization points

- **`on_contract_initialize`:** Use this function to automate contract uploads, configure dependencies, or perform any
  setup necessary before testing.

## Custom runtime parameters

Phink provides default runtime configurations, but developers can provide their own runtime parameters in
`contract/runtime.rs`. This can be particularly useful if you wish to connect your fuzzing environment to your own
Substrate runtime, so Phink can be adapted to work with your specific runtime.
**You can edit the runtime configure [here](https://github.com/srlabs/phink/blob/main/src/contract/runtime.rs).**

### Example: custom runtime configuration

For instance, customize the `pallet_timestamp` runtime parameters like this:

```rust,ignore
impl pallet_timestamp::Config for Runtime {
    type MinimumPeriod = CustomMinimumPeriod;
    ...
}
```