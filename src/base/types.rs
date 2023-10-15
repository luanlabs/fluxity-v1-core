use soroban_sdk::{contracttype, Address, Vec};

#[contracttype]
#[derive(Debug)]
pub struct LinearStreamType {
    pub sender: Address,
    pub receivers: Vec<Address>,
    pub token: Address,
    pub amount: i128,
    pub cancellable_date: u64,
    pub cliff_date: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub withdrawn: i128,
}
