use soroban_sdk::contracttype;

#[contracttype]
#[derive(Copy, Clone, Debug)]
pub enum DataKey {
    Lockup(u64),
    LatestLockupId,
}
