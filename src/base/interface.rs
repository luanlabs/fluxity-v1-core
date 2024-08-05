use soroban_sdk::Env;

use super::{errors, types};

pub trait IFluxity {
    fn get_latest_lockup_id(e: Env) -> u64;
    fn get_lockup(e: Env, id: u64) -> Result<types::Lockup, errors::CustomErrors>;
    fn create_lockup(e: Env, params: types::LockupInput) -> Result<u64, errors::CustomErrors>;
    fn cancel_lockup(e: Env, id: u64) -> Result<(i128, i128), errors::CustomErrors>;
    fn withdraw_lockup(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors>;
    fn topup_lockup(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors>;
}
