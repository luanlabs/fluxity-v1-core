#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod base;
use base::{token::transfer, *};

pub trait FluxityTrait {
    fn init(e: Env, admin: Address);
    fn get_stream(e: Env, id: u64) -> Result<types::LinearStreamType, errors::CustomErrors>;
    fn create_stream(
        e: Env,
        params: types::LinearStreamInputType,
    ) -> Result<u64, errors::CustomErrors>;
    fn cancel_stream(e: Env, id: u64) -> Result<(i128, i128), errors::CustomErrors>;
    fn withdraw_stream(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors>;
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
            None => Err(errors::CustomErrors::StreamNotFound),
            Some(stream) => Ok(stream),
        }
    }

    fn create_stream(
        e: Env,
        params: types::LinearStreamInputType,
    ) -> Result<u64, errors::CustomErrors> {
        params.sender.require_auth();

        if params.amount <= 0 {
            return Err(errors::CustomErrors::InvalidAmount);
        }

        if &params.sender == &params.receiver {
            return Err(errors::CustomErrors::InvalidReceiver);
        }

        if &params.start_date >= &params.end_date {
            return Err(errors::CustomErrors::InvalidStartDate);
        }

        if &params.cancellable_date < &params.start_date
            || &params.cancellable_date > &params.end_date
        {
            return Err(errors::CustomErrors::InvalidCancellableDate);
        }

        if &params.cliff_date < &params.start_date || &params.cliff_date > &params.end_date {
            return Err(errors::CustomErrors::InvalidCliffDate);
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
            return Err(errors::CustomErrors::StreamAlreadyCanceled);
        }

        let current_date = e.ledger().timestamp();

        if stream.end_date >= current_date {
            return Err(errors::CustomErrors::StreamAlreadySettled);
        }

        if stream.cancellable_date > current_date {
            return Err(errors::CustomErrors::StreamNotCancellableYet);
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

        transfer(&e, &stream.token, &stream.receiver, &receiver_amount);
        transfer(&e, &stream.token, &stream.sender, &sender_amount);

        events::publish_stream_cancelled_event(&e, id);

        Ok((sender_amount, receiver_amount))
    }

    fn withdraw_stream(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors> {
        let mut stream = storage::get_stream_by_id(&e, &id).unwrap();

        stream.receiver.require_auth();

        if stream.is_cancelled {
            return Err(errors::CustomErrors::StreamIsCanceled);
        }

        let current_date = e.ledger().timestamp();

        if current_date <= stream.cliff_date {
            return Ok(0);
        }

        let amounts = utils::calculate_amounts(
            stream.start_date,
            stream.end_date,
            stream.cliff_date,
            current_date,
            stream.amount,
        );

        if amount < 0 {
            return Err(errors::CustomErrors::AmountUnderflows);
        }

        let withdrawable = amounts.receiver_amount - stream.withdrawn;

        if withdrawable < amount {
            return Err(errors::CustomErrors::SpecifiedAmountIsGreaterThanWithdrawable);
        }

        let mut amount_to_transfer = amount;

        if amount == 0 {
            amount_to_transfer = withdrawable;
        }

        stream.withdrawn = stream.withdrawn + amount_to_transfer;

        storage::set_stream(&e, id, &stream);

        transfer(&e, &stream.token, &stream.receiver, &amount_to_transfer);

        // TODO: withdrawn?
        events::publish_stream_withdrawn_event(&e, id);

        Ok(amount_to_transfer)
    }
}

#[cfg(test)]
mod tests;
