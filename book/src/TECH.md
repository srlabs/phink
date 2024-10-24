# How Phink works

Phink is built on top of AFL++, leveraging its capabilities to provide effective fuzz testing for ink! smart contracts.
Here's an overview of how the fuzzer operates.

## AFL++ integration

Phink utilizes AFL++ through two key components:

- **ziggy**: A multifuzzing crate that enables integration with multiple fuzzers.
- **afl.rs**: A crate that spawns AFL++ fuzzers, facilitating seamless mutation and coverage tracking.

### AFL++ mechanics

AFL++ mutates the input bytes and evaluates whether these mutations increase code coverage. If a mutation results in new
execution paths, the modified seed is retained in the corpus. This iterative process enhances the likelihood of
discovering hidden vulnerabilities.

### Monitoring execution

Users can monitor the execution logs using familiar AFL++ tools. For instance, by using `tail`, you can view real-time
fuzzer logs and activity:

```bash
tail -f output/phink/logs/afl.log
tail -f output/phink/logs/afl_1.log #if multi-threaded
```

Additionally, tools like `afl_showmap` allow developers to debug and visualize the coverage maps.

## Coverage-guided strategy

Currently, Phink employs a partially coverage-guided approach. While full coverage feedback from low-level
instrumentation is not available yet, plans are underway to integrate this capability
via [WASMI](https://github.com/wasmi-labs/wasmi) or [PolkaVM](https://github.com/koute/polkavm) in future
releases.

## Execution and validation

For each generated seed, Phink executes the associated input on a mock-emulated 'node'.
This setup ensures that invariants are verified: known selectors are checked to ensure that
invariants hold across different message calls.

## Contract instrumentation

Phink instruments contracts using the `syn` crate, allowing for precise modification and analysis of the smart contract
code. This instrumentation is pivotal for identifying potential vulnerabilities and safeguarding the integrity of the
fuzz testing process.