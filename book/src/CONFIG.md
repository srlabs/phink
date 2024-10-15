# Phink Configuration Guide

This guide provides an overview of the Phink configuration settings.

## Configuration File Overview

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

# Gas Limits
[default_gas_limit]
ref_time = 100_000_000_000      # Reference time for gas
proof_size = 3_145_728          # Proof size (3 * 1024 * 1024 bytes)
```

## General Settings

- **cores**: Allocate the number of CPU cores for fuzzing. Setting this to `1` enables single-core execution.
- **max_messages_per_exec**: Define the maximum number of message calls allowed per fuzzing input. If you want to fuzz
  one function per one function, set this number to 1. Setting it to zero will fuzz zero message. Setting it for example
  to 4 will generate 4 different messages in one input, run all the invariants, and go to the next input.

## Paths

- **instrumented_contract_path.path**: Specify the path to the instrumented contract, which should be set
  post-invocation of `phink instrument my_contract`. This path will contains the source code of the initial contract,
  with the additional instrumentation instructions. It also will contians the instrumented compiled contract.
- **report_path**: Designate the directory where HTML coverage reports will be generated, if the user wishes to generate
  a coverage report.
- **fuzz_output**: Indicate the directory for storing all fuzzing output. This output is important as it will contains
  the log file, the corpus entries, the crashes, and way more.

## Deployment

- **deployer_address**: Set the address of the smart contract deployer. The default is Alice's address.
- **constructor_payload**: Hexadecimal scale-encoded data necessary for contract instantiation. This is used when
  calling `bare_instantiate` extrinsic to instantiate the contract. You can use https://ui.use.ink/ to generate this
  payload. By default, Phink will deploy the contract using the constructor that have no arguments `new()`.
- **storage_deposit_limit**: Limit for storage deposits during contract deployment. It represents
  an optional cap on the amount of blockchain storage (measured in balance units) that can be used or reserved by the
  contract call.
- **instantiate_initial_value**: Initial value to be transferred upon contract instantiation, if required. So if the
  contract requires during instantiation a minimum amount of 3000 units, set 3000 here.

## Fuzzing Options

- **fuzz_origin**: A Boolean option to try calling each message as a different user, which may impact performance. If
  set to `false`, the fuzzer will fuzz any message with the one input (Alice).
- **verbose**: Enables detailed debugging messages when set to `true`. This will just output more logs.
- **show_ui**: Toggle for displaying the advanced user interface.
- **use_honggfuzz**: Determines whether to use Honggfuzz; remains `false` by
  default. (**let it false! not handled currently** )

## Gas Limits

### Default Gas Limit Configuration

The gas limit refers to the maximum amount of computational effort (or weight) that an execution is allowed to use when
performing a call to a contract. It controls how much balance a contract is allowed to use for expanding its state
storage during execution, ensures that users won't unintentionally spend more than they wanted on storage allocation,
and offers protection against excessive storage costs by defining an upper limit on how much can be spent on storage
within that call.

- **ref_time**: Specifies the reference time for gas allocation.
- **proof_size**: Defines the proof size (e.g., `3145728` corresponds to 3 MB).