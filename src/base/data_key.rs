use soroban_sdk::contracttype;

#[contracttype]
#[derive(Copy, Clone, Debug)]
pub enum DataKey {
    Xlm,
    Admin,
    MonthlyFee,
    Lockup(u64),
    LatestLockupId,
}
