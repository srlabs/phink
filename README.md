
# 🐙 Phink  

**Phink** is a blazing-fast⚡ ink! smart-contract property-based coverage-guided fuzzer. It allows ink! developpers to integrate inviolable properties into their smart contract testing workflow, providing them with the tools to automatically detect vulnerabilities and ensure contract reliability before deployment. 

## Install  
  
```  
git clone https://github.com/kevin-valerio/phink
cd phink/
cargo ziggy run   
```  
  
## Example  
#### Creating an invariant  
You can find below some invariants created for the [dns](https://github.com/kevin-valerio/phink/blob/main/sample/dns/lib.rs) contract.

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
   
    
## Features and incoming ideas  
  
 - [x] Custom runtime integration 
 - [x] Invariants-based fuzzing
 - [x] Incorrect arithmetic, reentrancy and panic handers
 - [x] Handling ink! specific encoding and constructors
 - [x] Automatic contract instantiation
 - [ ] LLM-based invariant creation
 - [ ] Proper binary usage
 - [ ] Fuzzing guidance through WASM coverage
 - [ ] Custom fuzzing dashboard
 - [ ] Providing a given on-chain state
 - [ ] Handling multiple blocks within the same state
 - [ ] Crafting multiple `messages` in the same state
 - [ ] Integrating [LibAFL](https://github.com/AFLplusplus/LibAFL/) for a better synergy 
 
