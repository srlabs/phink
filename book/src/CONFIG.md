# Phink configuration guide

This guide provides an overview of the Phink configuration settings. You will learn how to configure the general
settings, specify some key paths and fuzzing options, and do some other essential tasks. Without further ado, let's jump
right into it!

## Configuration file overview

Here's how a configuration file looks like:

```toml
### Phink Configuration

# General Settings
cores = 4                # Set to 1 for single-core execution
max_messages_per_exec = 1 # Maximum number of message calls per input

# Paths
instrumented_contract_path.path = "toooooooooooz"  # Path to the instrumented contract, after `phink instrument my_contract` is invoked
report_path = "output/phink/contract_coverage" # Directory for coverage HTML files
fuzz_output = "output"                         # Directory for fuzzing output

# Deployment
deployer_address = "5C62Ck4UrFPiBtoCmeSrgF7x9yv9mn38446dhCpsi2mLHiFT" # Contract deployer address (Alice by default)
constructor_payload = "9BAE9D5E"                                     # Hexadecimal scale-encoded data for contract instantiation
storage_deposit_limit = "100000000000"                              # Storage deposit limit
instantiate_initial_value = "0"                                     # Value transferred during instantiation, if needed

# Fuzzing Options
fuzz_origin = false  # Attempt to call each message as a different user (affects performance)
verbose = true       # Print detailed debug messages
show_ui = true       # Display advanced UI
use_honggfuzz = false # Use Honggfuzz (set as false)
catch_trapped_contract = false # Not setting trapped contract as a bug, only detecting invariant-based bugs

# Gas Limits
[default_gas_limit]
ref_time = 100_000_000_000      # Reference time for gas
proof_size = 3_145_728          # Proof size (3 * 1024 * 1024 bytes)
```

## General settings

The General settings cover these 2 parameters:

- **cores**: Allocate the number of CPU cores for fuzzing. Setting this to `1` enables single-core execution.
- **max_messages_per_exec**: Define the maximum number of message calls allowed per fuzzing input. If you want to fuzz
  one function per one function, set this number to 1. Setting it to zero will fuzz zero message. Setting it, for
  example,
  to 4 will generate 4 different messages in one input, run all the invariants, and go to the next input.

## Paths

The Paths settings cover these 3 parameters:

- **instrumented_contract_path.path**: Specify the path to the instrumented contract, which should be set
  post-invocation of `phink instrument my_contract`. This path will contain the source code of the initial contract,
  with the additional instrumentation instructions. It will also contain the instrumented compiled contract.
- **report_path**: Designate the directory where HTML coverage reports will be generated if the user wishes to generate
  a coverage report.
- **fuzz_output**: Indicate the directory for storing all fuzzing output. This output is important as it will contain
  the log file, the corpus entries, the crashes, and way more.

## Deployment

The Deployment settings include these 4 parameters:

- **deployer_address**: Set the address of the smart contract deployer. The default is Alice's address.
- **constructor_payload**: Hexadecimal scale-encoded data necessary for contract instantiation. This is used when
  calling `bare_instantiate` extrinsic to instantiate the contract. You can use https://ui.use.ink/ to generate this
  payload. By default, Phink will deploy the contract using the constructor that has no arguments `new()`.
- **storage_deposit_limit**: Limit for storage deposits during contract deployment. It represents
  an optional cap on the amount of blockchain storage (measured in balance units) that can be used or reserved by the
  contract call.
- **instantiate_initial_value**: Initial value to be transferred upon contract instantiation if required. So if the
  contract requires a minimum amount of 3000 units during instantiation, set 3000 here.

## Fuzzing options

These 4 parameters are important when you configure fuzzing options:

- **fuzz_origin**: A Boolean option to try calling each message as a different user, which may impact performance. If
  set to `false`, the fuzzer will fuzz any message with the one input (Alice).
- **verbose**: Enables detailed debugging of messages when set to `true`. This will just output more logs.
- **show_ui**: Toggle for displaying the advanced user interface.
- **use_honggfuzz**: Determines whether to use Honggfuzz; remains `false` by
  default. (**let it false! is not handled currently**)
- **catch_trapped_contract**: Indicate whether the fuzzer should treat trapped contracts as bugs.
    - When set to `true`: The fuzzer will identify any contracts that become trapped (`ContractTrapped`) as bugs. This
      is
      useful for an examination of potential issues, as it covers all types of bugs, not just ones related to
      logic or state invariants.
    - When set to `false`: Focuses only on catching bugs related to invariant violations, ignoring trapped contract
      scenarios. This is preferable when you are only interested in logical correctness and not in trapping errors.

## Gas limit

The gas limit refers to the maximum amount of computational effort (or weight) that an execution is allowed to use when
performing a call to a contract. It controls how much balance a contract is allowed to use for expanding its state
storage during execution. The setting ensures that users won't unintentionally spend more than they wanted on storage
allocation. Besides, it offers protection against excessive storage costs by defining an upper limit on how much can be
spent on storage
within that call.

### Default gas limit configuration

The key Gas limit settings include:

- **ref_time**: Specify the reference time for gas allocation.
- **proof_size**: Define the proof size (e.g., `3145728` corresponds to 3 MB).