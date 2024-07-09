use super::data_key::DataKey;
use soroban_sdk::Env;

const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_AMOUNT: u32 = 60 * DAY_IN_LEDGERS;
const LIFETIME_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;

pub fn extend_data_ttl(e: &Env, key: &DataKey) {
    e.storage()
        .persistent()
        .bump(key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn extend_contract_ttl(e: &Env) {
    e.storage().instance().bump(LIFETIME_THRESHOLD, BUMP_AMOUNT);
}
