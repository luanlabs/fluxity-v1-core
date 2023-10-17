#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod base;
use base::*;

pub trait FluxityTrait {
    fn init(e: Env, admin: Address);
    fn get_stream(e: Env, id: u64) -> Result<types::LinearStreamType, errors::CustomErrors>;
    fn create_stream(
        e: Env,
        params: types::LinearStreamInputType,
    ) -> Result<u64, errors::CustomErrors>;
    fn cancel_stream(e: Env, id: u64) -> Result<(i128, i128), errors::CustomErrors>;
    fn withdraw_stream(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors>;
    fn top_up_stream(e: Env);
}

#[contract]
pub struct Fluxity;

#[contractimpl]
impl FluxityTrait for Fluxity {
    fn init(_e: Env, _admin: Address) {}

    fn get_stream(e: Env, id: u64) -> Result<types::LinearStreamType, errors::CustomErrors> {
        match e
            .storage()
            .persistent()
            .get(&data_key::DataKey::LinearStream(id))
        {
            None => Err(errors::CustomErrors::GetStreamNotFound),
            Some(stream) => Ok(stream),
        }
    }

    fn create_stream(
        e: Env,
        params: types::LinearStreamInputType,
    ) -> Result<u64, errors::CustomErrors> {
        params.sender.require_auth();

        if params.amount <= 0 {
            return Err(errors::CustomErrors::CreateStreamInvalidAmount);
        }

        if &params.sender == &params.receiver {
            return Err(errors::CustomErrors::CreateStreamInvalidReceiver);
        }

        if &params.start_date >= &params.end_date {
            return Err(errors::CustomErrors::CreateStreamInvalidStartDate);
        }

        if &params.cancellable_date < &params.start_date
            || &params.cancellable_date > &params.end_date
        {
            return Err(errors::CustomErrors::CreateStreamInvalidCancellableDate);
        }

        if &params.cliff_date < &params.start_date || &params.cliff_date > &params.end_date {
            return Err(errors::CustomErrors::CreateStreamInvalidCliffDate);
        }

        soroban_sdk::token::Client::new(&e, &params.token).transfer_from(
            &e.current_contract_address(),
            &params.sender,
            &e.current_contract_address(),
            &params.amount,
        );

        let id = storage::get_latest_stream_id(&e);
        let stream = params.into_linear_stream_type();

        storage::set_stream(&e, id, &stream);
        storage::increment_latest_stream_id(&e, &id);
        events::publish_stream_created_event(&e, id);

        Ok(id)
    }

    fn cancel_stream(e: Env, id: u64) -> Result<(i128, i128), errors::CustomErrors> {
        let mut stream = storage::get_stream_by_id(&e, &id).unwrap();

        stream.sender.require_auth();

        if stream.is_cancelled {
            return Err(errors::CustomErrors::CancelStreamAlreadyCanceled);
        }

        let current_date = e.ledger().timestamp();

        if stream.end_date >= current_date {
            return Err(errors::CustomErrors::CancelStreamAlreadySettled);
        }

        if stream.cancellable_date > current_date {
            return Err(errors::CustomErrors::CancelStreamNotCancellableYet);
        }

        let amounts = utils::calculate_amounts(
            stream.start_date,
            stream.end_date,
            stream.cliff_date,
            current_date,
            stream.amount,
        );

        let sender_amount = amounts.sender_amount;
        let receiver_amount = amounts.receiver_amount - stream.withdrawn;

        stream.is_cancelled = true;
        stream.withdrawn = amounts.receiver_amount;

        storage::set_stream(&e, id, &stream);

        let token = soroban_sdk::token::Client::new(&e, &stream.token);

        token.transfer(
            &e.current_contract_address(),
            &stream.receiver,
            &receiver_amount,
        );

        token.transfer(
            &e.current_contract_address(),
            &stream.sender,
            &sender_amount,
        );

        events::publish_stream_cancelled_event(&e, id);

        Ok((sender_amount, receiver_amount))
    }

    fn withdraw_stream(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors> {
        let mut stream = storage::get_stream_by_id(&e, &id).unwrap();

        stream.receiver.require_auth();

        let now = e.ledger().timestamp();

        if now <= stream.start_date {
            return Err(errors::CustomErrors::WithdrawStreamNotStartedYet);
        }

        // TODO: fix this
        Ok(2)
    }

    fn top_up_stream(_e: Env) {}
}

#[cfg(test)]
mod tests;
