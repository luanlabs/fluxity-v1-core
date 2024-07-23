use soroban_sdk::{Address, Env};

use super::{errors, types};

pub trait IFluxity {
    fn initialize(e: Env, admin: Address, xlm: Address);
    fn get_admin(e: Env) -> Address;
    fn get_xlm(e: Env) -> Address;
    fn set_monthly_fee(e: Env, fee: i128);
    fn get_monthly_fee(e: Env) -> i128;
    fn get_latest_lockup_id(e: Env) -> u64;
    fn calculate_fee(e: Env, start_date: u64, end_date: u64) -> i128;
    fn get_lockup(e: Env, id: u64) -> Result<types::Lockup, errors::CustomErrors>;
    fn create_stream(e: Env, params: types::LockupInput) -> Result<u64, errors::CustomErrors>;
    fn create_vesting(e: Env, params: types::LockupInput) -> Result<u64, errors::CustomErrors>;
    fn cancel_lockup(e: Env, id: u64) -> Result<(i128, i128), errors::CustomErrors>;
    fn withdraw_lockup(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors>;
    fn topup_lockup(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors>;
}
