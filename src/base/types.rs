use soroban_sdk::{contracttype, Address, Env, IntoVal, Val, Vec};

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
#[derive(Debug, Clone)]
pub struct LockupInput {
    pub spender: Address,
    pub sender: Address,
    pub receiver: Address,
    pub token: Address,
    pub amount: i128,
    pub cancellable_date: u64,
    pub cliff_date: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub rate: Rate,
    pub is_vesting: bool,
}

#[contracttype]
#[derive(Debug, PartialEq)]
pub struct Lockup {
    pub withdrawn: i128,
    pub is_cancelled: bool,
    pub spender: Address,
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

impl Into<Lockup> for LockupInput {
    fn into(self) -> Lockup {
        Lockup {
            withdrawn: 0,
            is_cancelled: false,
            spender: self.spender.clone(),
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
            rate: self.rate,
            is_vesting: self.is_vesting,
        }
    }
}

impl LockupInput {
    pub fn to_vec_val(&self, env: &Env) -> Vec<Val> {
        let mut v = Vec::new(env);

        v.push_back(self.sender.clone().into_val(env));
        v.push_back(self.receiver.clone().into_val(env));
        v.push_back(self.token.clone().into_val(env));
        v.push_back(self.amount.clone().into_val(env));
        v.push_back(self.cancellable_date.clone().into_val(env));
        v.push_back(self.cliff_date.clone().into_val(env));
        v.push_back(self.start_date.clone().into_val(env));
        v.push_back(self.end_date.clone().into_val(env));
        v.push_back(self.rate.clone().into_val(env));
        v.push_back(self.is_vesting.clone().into_val(env));

        v
    }
}
