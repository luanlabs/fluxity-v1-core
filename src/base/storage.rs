use soroban_sdk::Env;

use super::data_key;
use super::errors;
use super::extend_ttl::{extend_contract_ttl, extend_data_ttl};
use super::types;

pub fn get_lockup_by_id(e: &Env, id: &u64) -> Result<types::Lockup, errors::CustomErrors> {
    match e
        .storage()
        .persistent()
        .get(&data_key::DataKey::Lockup(*id))
    {
        None => Err(errors::CustomErrors::LockupNotFound),
        Some(stream) => Ok(stream),
    }
}

pub fn get_latest_lockup_id(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&data_key::DataKey::LatestLockupId)
        .unwrap_or(0)
}

pub fn increment_latest_stream_id(e: &Env, id: &u64) {
    e.storage()
        .instance()
        .set(&data_key::DataKey::LatestLockupId, &(id + 1));

    extend_contract_ttl(&e);
}

pub fn set_stream(e: &Env, id: u64, stream: &types::Lockup) {
    let key = data_key::DataKey::Lockup(id);

    e.storage().persistent().set(&key, stream);

    extend_data_ttl(&e, &key);
    extend_contract_ttl(&e);
}
