#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

mod base;
use base::*;

#[contracterror]
#[derive(Copy, Clone, Debug)]
#[repr(u64)]
pub enum CustomErrors {
    InvalidAmount = 4000,
    InvalidReceiver = 4001,
    InvalidStartDate = 4002,
    InvalidCliffDate = 4003,
    StreamNotFound = 4040,
}

pub trait FluxityTrait {
    fn init(e: Env, admin: Address);
    fn get_stream(e: Env, id: u64) -> Result<types::LinearStreamType, CustomErrors>;
    fn create_stream(e: Env, params: types::LinearStreamType) -> Result<u64, CustomErrors>;
    fn cancel_stream(e: Env);
    fn withdraw_stream(e: Env);
    fn top_up_stream(e: Env);
}

#[contract]
pub struct Fluxity;

#[contractimpl]
impl FluxityTrait for Fluxity {
    fn init(_e: Env, _admin: Address) {}
    fn get_stream(e: Env, id: u64) -> Result<types::LinearStreamType, CustomErrors> {
        match e
            .storage()
            .persistent()
            .get(&data_key::DataKey::LinearStream(id))
        {
            None => Err(CustomErrors::StreamNotFound),
            Some(stream) => Ok(stream),
        }
    }

    fn create_stream(e: Env, params: types::LinearStreamType) -> Result<u64, CustomErrors> {
        if params.amount <= 0 {
            return Err(CustomErrors::InvalidAmount);
        }

        for receiver in params.receivers.clone() {
            if params.sender == receiver {
                return Err(CustomErrors::InvalidReceiver);
            }
        }

        if &params.start_date >= &params.end_date {
            return Err(CustomErrors::InvalidStartDate);
        }

        if &params.cliff_date < &params.start_date || &params.cliff_date > &params.end_date {
            return Err(CustomErrors::InvalidCliffDate);
        }

        soroban_sdk::token::Client::new(&e, &params.token).transfer_from(
            &e.current_contract_address(),
            &params.sender,
            &e.current_contract_address(),
            &params.amount,
        );

        let id = storage::get_latest_stream_id(&e);

        e.storage()
            .persistent()
            .set(&data_key::DataKey::LinearStream(id), &params);

        storage::increment_latest_stream_id(&e, &id);

        events::publish_stream_created_event(&e, id);

        Ok(id)
    }

    fn cancel_stream(_e: Env) {}
    fn withdraw_stream(_e: Env) {}
    fn top_up_stream(_e: Env) {}
}

// #[cfg(test)]
// mod tests;
