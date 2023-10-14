#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug)]
#[repr(u64)]
pub enum CustomErrors {
    InvalidNumber = 1,
    InvalidReceiver = 2,
    InvalidDates = 3,
    NotFound = 404,
}

#[contracttype]
#[derive(Copy, Clone, Debug)]
enum DataKey {
    Admin,
    LinearStream(u64),
    LatestStreamId,
    // TODO: ADMIN? FEE? USER SCORES?
}

#[contracttype]
#[derive(Debug)]
pub struct LinearStreamType {
    sender: Address,
    receivers: Vec<Address>,
    token: Address, // ERC20
    amount: i128,
    cancellable_date: u64,
    cliff_date: u64,
    start_date: u64,
    end_date: u64,
}

pub trait FluxityTrait {
    fn init(e: Env, admin: Address);
    fn get_stream(e: Env, id: u64) -> Result<LinearStreamType, CustomErrors>;
    fn create_stream(e: Env, params: LinearStreamType) -> Result<u64, CustomErrors>;
    fn cancel_stream(e: Env);
    fn withdraw_stream(e: Env);
    fn top_up_stream(e: Env);
}

#[contract]
pub struct Fluxity;

#[contractimpl]
impl FluxityTrait for Fluxity {
    fn init(_e: Env, _admin: Address) {}
    fn get_stream(e: Env, id: u64) -> Result<LinearStreamType, CustomErrors> {
        match e.storage().persistent().get(&DataKey::LinearStream(id)) {
            None => Err(CustomErrors::NotFound),
            Some(stream) => Ok(stream),
        }
    }

    fn create_stream(e: Env, params: LinearStreamType) -> Result<u64, CustomErrors> {
        if params.amount <= 0 {
            return Err(CustomErrors::InvalidNumber);
        }

        for receiver in params.receivers.clone() {
            if params.sender == receiver {
                return Err(CustomErrors::InvalidReceiver);
            }
        }

        if &params.start_date >= &params.end_date {
            return Err(CustomErrors::InvalidDates);
        }

        soroban_sdk::token::Client::new(&e, &params.token).transfer_from(
            &e.current_contract_address(),
            &params.sender,
            &e.current_contract_address(),
            &params.amount,
        );

        let id = e
            .storage()
            .persistent()
            .get(&DataKey::LatestStreamId)
            .unwrap_or(0);

        e.storage()
            .persistent()
            .set(&DataKey::LinearStream(id), &params);

        e.storage()
            .persistent()
            .set(&DataKey::LatestStreamId, &(id + 1));

        Ok(id)
    }

    fn cancel_stream(_e: Env) {}
    fn withdraw_stream(_e: Env) {}
    fn top_up_stream(_e: Env) {}
}

// #[cfg(test)]
// mod tests;
