use soroban_sdk::contracttype;

#[contracttype]
#[derive(Copy, Clone, Debug)]
pub enum DataKey {
    XLM,
    Admin,
    MonthlyFee,
    Lockup(u64),
    LatestLockupId,
}
