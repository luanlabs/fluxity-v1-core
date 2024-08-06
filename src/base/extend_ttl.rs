use super::{
    constants::{BUMP_AMOUNT, LIFETIME_THRESHOLD},
    data_key::DataKey,
};
use soroban_sdk::Env;

pub fn extend_data_ttl(e: &Env, key: &DataKey) {
    e.storage()
        .persistent()
        .bump(key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn extend_contract_ttl(e: &Env) {
    e.storage().instance().bump(LIFETIME_THRESHOLD, BUMP_AMOUNT);
}
