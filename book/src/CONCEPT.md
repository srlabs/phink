# Concepts and Terminology

## Concepts

### Fuzzing in general

Fuzzing is an automated software testing technique that involves providing invalid, unexpected, or random data inputs to
a program. The primary goal is to uncover anomalies, such as crashes, assertion failures, that signify
potential vulnerabilities.

### Property-based Fuzzing

Property-based testing involves specifying properties or invariants that your ink! contract should always satisfy. In
Phink, these properties act as assertions. Phink uses this approach by allowing developers to
define properties directly within ink! smart contracts. Such properties are then rigorously tested against varied
inputs, ensuring the contract maintains its invariants across all possible data conditions.

### Coverage-guided Fuzzing

Coverage-guided fuzzing is a fuzzing strategy that focuses on maximizing code coverage during testing. It uses
feedback from code execution paths to guide input generation, focusing on unexplored parts of the code.
Phink instruments smart contracts to track code coverage, optimizing its fuzzing efforts by targeting less examined
paths. This results in more thorough testing and potentially uncovers hard-to-find vulnerabilities.

## Terminology

### Corpus

A **corpus** refers to the collection of all input samples used during the testing process. It is
continuously updated with new inputs that lead to unique execution paths.

---

### Seed

A **seed** is an initial input provided to the fuzzer to start the testing process. Seeds serve as the starting point
for generating new test cases and are crucial for initializing a diverse and effective fuzzing campaign. A strong set of
seed inputs can significantly enhance the fuzzing campaign.

---

### Invariants

**Invariants** are conditions or properties that must remain true at all times during the execution of a program or
contract. In property-based testing, invariants are used as assertions to verify the consistent behavior of smart
contracts under various input conditions. Breaking an invariant indicates a potential bug or vulnerability.

---

### Instrumentation

**Instrumentation** involves modifying a program to collect runtime information such as code coverage data. In fuzzing,
instrumentation is used to trace execution paths, enabling coverage-guided techniques to generate more informed and
effective test cases.

---

### Coverage

**Coverage** is a measure of how much of a program's code is tested during fuzzing. High coverage corresponds to a
good assessment of the contracts logic.

---

### Contract Selectors

**ink! contract selectors** are unique identifiers for functions within ink! smart contracts. Selectors are derived from
function signatures and are used to call specific functions within a contract deployed on the blockchain.

---
