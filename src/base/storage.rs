use soroban_sdk::Address;
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

pub fn increment_latest_lockup_id(e: &Env, id: &u64) {
    e.storage()
        .instance()
        .set(&data_key::DataKey::LatestLockupId, &(id + 1));

    extend_contract_ttl(&e);
}

pub fn set_lockup(e: &Env, id: u64, stream: &types::Lockup) {
    let key = data_key::DataKey::Lockup(id);

    e.storage().persistent().set(&key, stream);

    extend_data_ttl(&e, &key);
    extend_contract_ttl(&e);
}

pub fn set_admin(e: &Env, admin: Address) {
    let key = data_key::DataKey::Admin;

    e.storage().instance().set(&key, &admin);

    extend_data_ttl(&e, &key);
    extend_contract_ttl(&e);
}

pub fn get_admin(e: &Env) -> Address {
    let key = data_key::DataKey::Admin;

    e.storage().instance().get(&key).unwrap()
}

pub fn set_monthly_fee(e: &Env, fee: i128) {
    let key = data_key::DataKey::MonthlyFee;

    e.storage().instance().set(&key, &fee);

    extend_data_ttl(&e, &key);
    extend_contract_ttl(&e);
}

pub fn get_monthly_fee(e: &Env) -> i128 {
    let key = data_key::DataKey::MonthlyFee;

    e.storage().instance().get(&key).unwrap_or(0)
}

pub fn set_xlm(e: &Env, xlm: Address) {
    let key = data_key::DataKey::XLM;

    e.storage().instance().set(&key, &xlm);

    extend_data_ttl(&e, &key);
    extend_contract_ttl(&e);
}

pub fn get_xlm(e: &Env) -> Address {
    let key = data_key::DataKey::XLM;

    e.storage().instance().get(&key).unwrap()
}
