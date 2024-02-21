use soroban_sdk::contracttype;

#[contracttype]
#[derive(Copy, Clone, Debug)]
pub enum DataKey {
    Stream(u64),
    LatestStreamId,
}
