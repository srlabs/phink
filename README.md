# 🐙 Phink  

**Phink** is a blazing-fast⚡, property-based, coverage-guided fuzzer for ink! smart contracts. It enables developers to embed inviolable properties into their smart contract testing workflows, equipping them with automatic tools to detect vulnerabilities and ensure contract reliability before deployment.

> ⚠️ This project is actively under development with new features and improvements being made regularly. Contributions and feedback are welcome!


## Install  
  
```bash
git clone https://github.com/kevin-valerio/phink
cd phink/
cargo ziggy run   
```  
  
## Example  
#### Creating an invariant  
Below are some invariants created for the [dns](https://github.com/kevin-valerio/phink/blob/main/sample/dns/lib.rs) contract.


  ```rust
#[cfg(feature = "phink")]
#[ink(impl)]
impl DomainNameService {
    // This invariant ensures that `domains` doesn't contain the forbidden domain that nobody should regsiter 
    #[ink(message)]
    pub fn phink_assert_hash42_cant_be_registered(&self) {
        for i in 0..self.domains.len() {
            if let Some(domain) = self.domains.get(i) {
                // Invariant triggered! We caught an invalid domain in the storage...
                assert_ne!(domain.clone().as_mut(), FORBIDDEN_DOMAIN);
            }
        }
    }

    // This invariant ensures that nobody registed the forbidden number
    #[ink(message)]
    pub fn phink_assert_dangerous_number(&self) {
        let FORBIDDEN_NUMBER = 69;
        assert_ne!(self.dangerous_number, FORBIDDEN_NUMBER);
    }
}
```
   
    
## Features and upcoming ideas  
  
 - [x] Custom runtime integration 
 - [x] Invariants-based fuzzing
 - [x] Detection of incorrect arithmetic, reentrancy, and panic handlers
 - [x] Handling of ink! specific encoding and constructors
 - [x] Automatic contract instantiation
 - [ ] LLM-based invariant creation using [rust-llama](https://github.com/mdrokz/rust-llama.cpp)
 - [ ] Proper binary usage
 - [ ] Fuzzing guidance through WASM coverage
 - [ ] Custom fuzzing dashboard
 - [ ] Seeds and constants extraction out of code-base
 - [ ] Provision of a specified on-chain state
 - [ ] Handling of multiple blocks within the same state
 - [ ] Crafting multiple `messages` in the same transaction
 - [ ] Integration [LibAFL](https://github.com/AFLplusplus/LibAFL/) for enhanced synergy 
 
## Licence

Kinda WTFPL, but still under discussion.

