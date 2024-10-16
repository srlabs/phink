# How Phink Works

Phink is built on top of AFL++, leveraging its capabilities to provide effective fuzz testing for ink! smart contracts.
Here's an overview of how it operates:

## AFL++ Integration

Phink utilizes AFL++ through two key components:

- **ziggy**: A multifuzzing crate that enables integration with multiple fuzzers.
- **afl.rs**: A crate that spawns AFL++ fuzzers, facilitating seamless mutation and coverage tracking.

### AFL++ Mechanics

AFL++ mutates the input bytes and evaluates whether these mutations increase code coverage. If a mutation results in new
execution paths, the modified seed is retained in the corpus. This iterative process enhances the likelihood of
discovering hidden vulnerabilities.

### Monitoring Execution

Users can monitor the execution logs using familiar AFL++ tools. For instance, by using `tail`, you can view real-time
fuzzer logs and activity:

```bash
tail -f output/phink/logs/afl.log
tail -f output/phink/logs/afl_1.log #if multi-threaded
```

Additionally, tools like `afl_showmap` allow developers to debug and visualize the coverage maps.

## Coverage-Guided Strategy

Currently, Phink employs a partially coverage-guided approach. While full coverage feedback from low-level
instrumentation is not available yet, plans are underway to integrate this capability via WASM VM or PolkaVM in future
versions.

## Execution and Validation

For each generated seed, Phink executes the associated input on a local Substrate node, which is emulated using mocks.
This setup ensures that invariants are verified : known selectors are checked to ensure contract robustness and that
invariants hold across different execution paths.

## Contract Instrumentation

Phink instruments contracts using the `syn` crate, allowing for precise modification and analysis of the smart contract
code. This instrumentation is crucial for identifying potential vulnerabilities and ensuring the integrity of the fuzz
testing process.