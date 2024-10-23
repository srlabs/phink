use crate::{
    contract::selectors::selector::Selector,
    ResultOf,
};
use anyhow::bail;

#[derive(Clone, Debug)]
pub struct SelectorDatabase {
    invariants: Vec<Selector>,
    messages: Vec<Selector>,
    payable_messages: Vec<Selector>,
}

impl Default for SelectorDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectorDatabase {
    pub fn new() -> Self {
        Self {
            invariants: Vec::default(),
            messages: Vec::default(),
            payable_messages: Vec::default(),
        }
    }

    pub fn contains_invariant(&self, selector: &Selector) -> bool {
        self.invariants.contains(selector)
    }

    pub fn contains_message(&self, selector: &Selector) -> bool {
        self.messages.contains(selector)
    }

    pub fn is_payable(&self, selector: &Selector) -> bool {
        self.payable_messages.contains(selector)
    }
    pub fn add_payables(&mut self, selectors: Vec<Selector>) {
        self.payable_messages.extend(selectors);
    }
    pub fn exists(&self, selector: Selector) -> bool {
        self.messages.contains(&selector) || self.invariants.contains(&selector)
    }

    pub fn add_invariants(&mut self, invariants: Vec<Selector>) {
        self.invariants.extend(invariants);
    }

    pub fn add_messages(&mut self, messages: Vec<Selector>) {
        self.messages.extend(messages);
    }

    pub fn get_unique_messages(self) -> ResultOf<Vec<Selector>> {
        if !self.messages.is_empty() && !self.invariants.is_empty() {
            return Ok(self
                .messages
                .into_iter()
                .filter(|msg| !self.invariants.contains(msg))
                .collect());
        }
        bail!("No messages were found in the database")
    }

    pub fn invariants(self) -> ResultOf<Vec<Selector>> {
        if !self.invariants.is_empty() {
            return Ok(self.invariants)
        }
        bail!("No invariants were found in the database")
    }

    pub fn messages(self) -> ResultOf<Vec<Selector>> {
        if !self.messages.is_empty() {
            return Ok(self.messages)
        }
        bail!("No messages were found in the database")
    }
}
