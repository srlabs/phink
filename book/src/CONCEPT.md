# Concepts and terminology

## Concepts

### Fuzzing

**Fuzzing** is an automated software testing technique that involves providing random data inputs to
a program. The primary goal is to uncover anomalies, such as crashes and assertion failures. These are intriguing
because they pinpoint
potential vulnerabilities.

### Property-based fuzzing

**Property-based testing** involves specifying properties or invariants that your ink! contract should always satisfy.
In
Phink, these properties act as assertions. Phink makes it possible for developers to
define properties directly within ink! smart contracts. Such properties are then tested against varied
inputs. In this way, the contract maintains its invariants across all possible data conditions. But there is a final
twist in the fuzzing tale.

### Coverage-guided fuzzing

**Coverage-guided fuzzing** is a fuzzing strategy that focuses on maximizing code coverage during testing. It uses
feedback from code execution paths to guide input generation, focusing on unexplored parts of the code.
Phink instruments ink! smart contracts to track code coverage. Optimizing fuzzing efforts by targeting less examined
paths is what makes the game worth playing.

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
instrumentation traces execution paths, enabling coverage-guided techniques to generate more informed and
effective test cases.

---

### Coverage

**Coverage** measures how much of a program's code is tested during fuzzing. High coverage corresponds to a
good assessment of the contract's logic.

---

### Contract selectors

**ink! contract selectors** are unique identifiers for functions within ink! smart contracts. Selectors are derived from
function signatures and are used to call specific functions within a contract deployed on the blockchain.

---
