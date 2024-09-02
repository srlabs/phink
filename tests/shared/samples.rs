#[derive(Debug, Clone, Copy)]
pub enum Sample {
    CrossMessageBug,
    DNS,
    Dummy,
    MultiContractCaller,
    Transfer,
}
impl Sample {
    #[must_use] pub fn path(&self) -> &str {
        match self {
            Sample::CrossMessageBug => "sample/cross_message_bug",
            Sample::DNS => "sample/dns",
            Sample::Dummy => "sample/dummy",
            Sample::MultiContractCaller => "sample/multi-contract-caller",
            Sample::Transfer => "sample/transfer",
        }
    }
}
