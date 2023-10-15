use soroban_sdk::Env;

use super::data_key;

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
