<center>
<img src="https://raw.githubusercontent.com/srlabs/phink/refs/heads/main/assets/phink.png" alt="phink" width="250"/>

# Introduction

</center>

## Overview of Phink

Phink is a blazing-fast⚡, property-based, coverage-guided fuzzer for ink! smart contracts. It enables developers to
embed inviolable properties into their smart contract testing workflows, equipping them with automatic tools to detect
vulnerabilities and ensure contract reliability before deployment.

## Key Features

### Property-based Testing

Phink empowers developers to define properties directly within ink! smart contracts. By prefixing functions with
`phink`, such as `fn phink_assert_abc_always_true()`, developers create inviolable properties that
act as assertions. During testing, these properties are checked against every input. If a property’s assertion fails, it
triggers a panic, signaling that an invariant has been broken. This method ensures thorough validation of contract logic
and behavior.

### Coverage-guided Fuzzing

Leveraging coverage-guided fuzzing, Phink enhances its effectiveness by instrumenting smart contracts. Although
currently adding feedback on each line executed, Phink is designed to evolve, eventually monitoring coverage across new
edges and code branches. Feedback is transmitted to the `pallet_contract` via the `debug_message`, enabling dynamic
adaptation to uncovered paths and optimizing the fuzzing process. This results in broader exploration and identification
of potential vulnerabilities.

### Seamless Integration with ink! Smart Contracts

Phink offers a streamlined integration process for ink! smart contracts. Configuration is simple and intuitive,
requiring minimal setup in the `phink.toml` file. The workflow involves two straightforward steps: instrumenting the
contract and then initiating the fuzzing process. This ease of deployment allows developers to quickly incorporate Phink
into their smart contract testing pipeline, enhancing efficiency and reliability.

## Why Use Phink

### Detect Security Vulnerabilities

Phink addresses critical security concerns by automatically generating and testing a diverse range of inputs. This
process uncovers edge cases, logical flaws, and bugs that could lead to contract reversion. Through fuzz testing, Phink
explores different execution paths by generating extensive input variations. This
rigorous testing identifies bugs and potential vulnerabilities early in the development cycle, enabling developers to
address issues before deployment.
