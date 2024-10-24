<center>
<img src="https://raw.githubusercontent.com/srlabs/phink/refs/heads/main/assets/phink.png" alt="phink" width="250"/>

# Introduction

</center>

## Overview of Phink

**Phink** is a blazing-fast⚡, property-based, coverage-guided fuzzer for ink! smart contracts. It lets developers
embed inviolable properties into smart contract testing workflows, equipping teams with automatic tools to detect
vulnerabilities and ensure contract reliability before deployment.

### Dashboard overview

<img src="https://raw.githubusercontent.com/srlabs/phink/refs/heads/main/assets/dashboard.png" alt="phink"/>

### Key features

#### Property-based testing

Phink requires developers to define properties directly within ink! smart contracts. By prefixing functions with
`phink`, such as `fn phink_assert_abc_always_true()`, you create properties that
act as assertions. During testing, the fuzzer checks these properties against every input, which is a set of ink!
messages. If a
property’s assertion fails, this
triggers a panic. An invariant has been broken! This method ensures thorough validation of contract logic
and behavior.

#### Coverage-guided fuzzing

In order to become coverage-guided, Phink needs to instrument the ink! smart contract.
Feedback is transmitted to the `pallet_contract` via the `debug_message`.
Although the fuzzer
currently adds feedback on each line executed, Phink is designed to evolve. It will eventually monitor coverage across
new
edges and code branches.

### Why use Phink

Phink addresses security concerns in these three main ways:

1. **Automatically generate and test** a diverse range of inputs
2. **Detect** edge cases, logical flaws, and bugs leading to contract state reversion
3. **Explore** different execution paths by generating input mutation

This extensive testing identifies bugs and potential flaws early in the development cycle, empowering teams to
fix vulnerabilities before deployment and deliver safer applications.
