use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum Rate {
    Daily = 86400,
    Weekly = 604800,
    Monthly = 2592000,
    Quarterly = 10368000,
    Annually = 31536000,
}

#[derive(Copy, Clone, Debug)]
pub struct Amounts {
    pub sender_amount: i128,
    pub receiver_amount: i128,
}

#[contracttype]
#[derive(Debug)]
pub struct StreamInputType {
    pub sender: Address,
    pub receiver: Address,
    pub token: Address,
    pub amount: i128,
    pub cancellable_date: u64,
    pub cliff_date: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub rate: Rate,
}

impl Into<StreamType> for StreamInputType {
    fn into(self) -> StreamType {
        StreamType {
            withdrawn: 0,
            is_cancelled: false,
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            token: self.token.clone(),
            amount: self.amount,
            cancellable_date: self.cancellable_date,
            cancelled_date: 0,
            cliff_date: self.cliff_date,
            start_date: self.start_date,
            end_date: self.end_date,
            // rate: Rate::Daily,
            rate: self.rate.clone(),
            is_vesting: false,
        }
    }
}

#[contracttype]
#[derive(Debug)]
pub struct VestingInputType {
    pub sender: Address,
    pub receiver: Address,
    pub token: Address,
    pub amount: i128,
    pub cancellable_date: u64,
    pub cliff_date: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub rate: Rate,
}

impl Into<StreamType> for VestingInputType {
    fn into(self) -> StreamType {
        StreamType {
            withdrawn: 0,
            is_cancelled: false,
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            token: self.token.clone(),
            amount: self.amount,
            cancellable_date: self.cancellable_date,
            cancelled_date: 0,
            cliff_date: self.cliff_date,
            start_date: self.start_date,
            end_date: self.end_date,
            rate: self.rate.clone(),
            is_vesting: true,
        }
    }
}

#[contracttype]
#[derive(Debug, PartialEq)]
pub struct StreamType {
    pub withdrawn: i128,
    pub is_cancelled: bool,
    pub sender: Address,
    pub receiver: Address,
    pub token: Address,
    pub amount: i128,
    pub cancellable_date: u64,
    pub cancelled_date: u64,
    pub cliff_date: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub rate: Rate,
    pub is_vesting: bool,
}
