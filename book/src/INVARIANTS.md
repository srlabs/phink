# Invariants

Invariants are **fundamental properties that must always hold** true in a smart-contract, regardless of any operations
performed. They help ensure that certain logical conditions remain constant throughout the
execution of the contract, preventing potential vulnerabilities and ensuring its reliability.

We suggest to use **integrity** and **unit tests** from your codebase to get inspiration to generate good invariants.

## Creating good invariants for Ink! smart-contracts

Below are some guidelines to help you design robust invariants:

1. **Understand the Contract's Logic**: Before crafting invariants, deeply understand the core logic and expected
   behaviors of your smart contract.

2. **Identify Critical Properties**: Determine critical properties or conditions that must hold **true**. This could
   involve
   state variables, transaction outcomes, or other interdependent conditions.

3. **Consider Corner Cases**: Think about edge cases and potential attack vectors. Invariants should be designed to
   capture unexpected or extreme scenarios.

4. **Focus on Consistency**: Consider properties that ensure data consistency across state changes. This might involve
   ensuring balances are correctly updated or ownership is properly maintained.

5. **Keep it Simple**: While considering complex scenarios, ensure your invariants are straightforward to encourage
   maintainability and clarity.

## Example invariant in ink! smart-contracts

Here is a template to get you started on writing invariants for ink! smart contracts:

```rust
#[cfg(feature = "phink")]
#[ink(impl)]
impl DomainNameService {
    /// Example invariant:
    #[ink(message)]
    #[cfg(feature = "phink")]
    pub fn phink_balance_invariant(&self) {
        // Ensure total supply equals sum of individual balances
        assert_eq!(self.total_supply, self.calculate_total_balances(), "Balance invariant violated!");
    }
}
```

### Important Annotations Explained

- **`#[cfg(feature = "phink")]`**: Ensures the function is only compiled when the "phink" feature is enabled.
- **`#[ink(message)]`**: Marks the function as an executable entry defined by the ink! framework.
- **Function Naming**: Begin with "phink_" to indicate the purpose and correlation to fuzz testing.

## Creating Invariants with LLM

Large Language Models (LLMs) offer a good (_lazy, yes..._) approach to generate invariants by interpreting the logic and
identifying properties from the contract code. Here is an example prompt system you could use to generate a base of
invariants

**Example Prompt:**

```markdown
You are provided with Rust files containing an ink! smart contract. Your task is to generate invariants, which are
inviolable properties that a fuzzer will check to ensure the contract's quality and correctness. Please adhere to the
following requirements while writing the invariants:

1. Ensure that the `impl` block is annotated with `#[cfg(feature = "phink")] #[ink(impl)]`.
2. Confirm that the `impl DomainNameService` is the main implementation block of the contract.
3. Each invariant must be annotated with:
    - `#[ink(message)]`
    - `#[cfg(feature = "phink")]`
    - Function names must start with "phink_".
4. Each invariant function must contain at least one assertion statement, such as `assert`, `assert_ne`, `panic`, etc.
5. Be creative and consider corner cases to ensure the thoroughness of the invariants.

   Example Output:

```rust
#[cfg(feature = "phink")]
#[ink(impl)]
impl DomainNameService {
    // This invariant ensures that `domains` doesn't contain the forbidden domain that nobody should register 
    #[ink(message)]
    #[cfg(feature = "phink")]
    pub fn phink_assert_hash42_cant_be_registered(&self) {
        for i in 0..self.domains.len() {
            if let Some(domain) = self.domains.get(i) {
                // Invariant triggered! We caught an invalid domain in the storage...
                assert_ne!(domain.clone().as_mut(), FORBIDDEN_DOMAIN);
            }
        }
    }
}
` ` `
```