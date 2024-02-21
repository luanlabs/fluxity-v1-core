use soroban_sdk::Env;

use super::data_key;
use super::errors;
use super::extend_ttl::{extend_contract_ttl, extend_data_ttl};
use super::types;

pub fn get_stream_by_id(e: &Env, id: &u64) -> Result<types::StreamType, errors::CustomErrors> {
    match e
        .storage()
        .persistent()
        .get(&data_key::DataKey::Stream(*id))
    {
        None => Err(errors::CustomErrors::StreamNotFound),
        Some(stream) => Ok(stream),
    }
}

pub fn get_latest_stream_id(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&data_key::DataKey::LatestStreamId)
        .unwrap_or(0)
}

pub fn increment_latest_stream_id(e: &Env, id: &u64) {
    e.storage()
        .instance()
        .set(&data_key::DataKey::LatestStreamId, &(id + 1));

    extend_contract_ttl(&e);
}

pub fn set_stream(e: &Env, id: u64, stream: &types::StreamType) {
    let key = data_key::DataKey::Stream(id);

    e.storage().persistent().set(&key, stream);

    extend_data_ttl(&e, &key);
    extend_contract_ttl(&e);
}
