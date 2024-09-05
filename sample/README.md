# Sample Contracts for Phink Fuzzer

A lit of sample ink! smart-contracts for Phink. This directory contains a variety of smart contracts
designed to test different aspects of Phink's fuzzing capabilities.

## Available Samples

- **dns/**: Contains a smart contract with complex logic to test Phink's ability to handle more intricate contract
  scenarios.

- **cross_message_bug/**: Includes a contract with a bug triggered by a specific order of Ink! message function calls.

- **dummy/**: Features a contract with nested `if` statements to evaluate Phink's coverage-guided fuzzing.

- **multi-contract-caller/**: Provides contracts designed to test interactions between multiple contracts and to see how
  well Phink handles contracts calling other contracts.

- **transfer/**: Contains a contract that checks for invariant bugs related to invalid transferred amounts, ensuring
  that Phink correctly identifies and handles invalid transfer scenarios.

## Instrumentation

To instrument any of these sample contracts for fuzzing, use the following command:

```bash
phink instrument sample/<test_contract>
```

Replace `<test_contract>` with the path to the specific contract you want to instrument.