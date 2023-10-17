use soroban_sdk::Env;

use super::data_key;
use super::errors;
use super::types;

pub fn get_stream_by_id(
    e: &Env,
    id: &u64,
) -> Result<types::LinearStreamType, errors::CustomErrors> {
    match e
        .storage()
        .persistent()
        .get(&data_key::DataKey::LinearStream(*id))
    {
        None => Err(errors::CustomErrors::GetStreamNotFound),
        Some(stream) => Ok(stream),
    }
}

pub fn get_latest_stream_id(e: &Env) -> u64 {
    e.storage()
        .persistent()
        .get(&data_key::DataKey::LatestStreamId)
        .unwrap_or(0)
}

pub fn increment_latest_stream_id(e: &Env, id: &u64) {
    e.storage()
        .persistent()
        .set(&data_key::DataKey::LatestStreamId, &(id + 1));
}

pub fn set_stream(e: &Env, id: u64, stream: &types::LinearStreamType) {
    e.storage()
        .persistent()
        .set(&data_key::DataKey::LinearStream(id), stream);
}
