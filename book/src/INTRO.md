<center>
<img src="https://raw.githubusercontent.com/srlabs/phink/refs/heads/main/assets/phink.png" alt="phink" width="250"/>

# Introduction

</center>

## Overview of Phink

**Phink** is a blazing-fast⚡, property-based, coverage-guided fuzzer for ink! smart contracts. It enables developers to
embed inviolable properties into their smart contract testing workflows, equipping them with automatic tools to detect
vulnerabilities and ensure contract reliability before deployment.

### Dashboard Overview

<img src="https://raw.githubusercontent.com/srlabs/phink/refs/heads/main/assets/dashboard.png" alt="dashboard" />

### Key Features

#### Property-based Testing

Phink requires developers to define properties directly within ink! smart contracts. By prefixing functions with
`phink`, such as `fn phink_assert_abc_always_true()`, developers create properties that
act as assertions. During testing, these properties are checked against every input (a set of ink! messages). If a
property’s assertion fails, it
triggers a panic, signaling that an invariant has been broken. This method ensures thorough validation of contract logic
and behavior.

#### Coverage-guided Fuzzing

In order to become coverage-guided, Phink needs to instrument the ink! smart contract. Although
currently adding feedback on each line executed, Phink is designed to evolve, eventually monitoring coverage across new
edges and code branches. Feedback is transmitted to the `pallet_contract` via the `debug_message`.

### Why Use Phink

Phink addresses security concerns by:

- **Automatically generating and testing** a diverse range of inputs
- **Uncovering** edge cases, logical flaws, and bugs leading to contract state reversion
- **Exploring** different execution paths by generating input mutation

This extensive testing identifies bugs and potential vulnerabilities in the development cycle, enabling developers to
address issues before deployment.
