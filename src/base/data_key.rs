use soroban_sdk::contracttype;

#[contracttype]
#[derive(Copy, Clone, Debug)]
pub enum DataKey {
    Admin,
    LinearStream(u64),
    LatestStreamId,
    // TODO: ADMIN? FEE? USER SCORES?
}
