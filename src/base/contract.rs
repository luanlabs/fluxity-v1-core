use soroban_sdk::{contract, contractimpl, Address, Env};
use token::take_xlm_fee;
use utils::calculate_lockup_fee;

use self::{storage::get_lockup_by_id, utils::calculate_additional_time};

use super::*;

use interface::IFluxity;

#[contract]
pub struct Fluxity;

#[contractimpl]
impl IFluxity for Fluxity {
    /// Initializes the contract and sets admin for it
    ///
    /// # Examples
    ///
    /// ```
    /// let id = fluxity_client::initialize();
    /// ```
    fn initialize(e: Env, admin: Address, xlm: Address) {
        storage::set_admin(&e, admin);
        storage::set_xlm(&e, xlm);
    }

    /// Returns the admin of the Fluxity contract
    ///
    /// # Examples
    ///
    /// ```
    /// let id = fluxity_client::get_admin();
    /// ```
    fn get_admin(e: Env) -> Address {
        storage::get_admin(&e)
    }

    /// Sets the monthly fee for lockups. Only the admin can call this
    ///
    /// # Examples
    ///
    /// ```
    /// let id = fluxity_client::set_monthly_fee(200);
    /// ```
    fn set_monthly_fee(e: Env, fee: i128) {
        storage::set_monthly_fee(&e, fee);
    }

    /// Returns the monthly fee for lockups
    ///
    /// # Examples
    ///
    /// ```
    /// let id = fluxity_client::get_monthly_fee();
    /// ```
    fn get_monthly_fee(e: Env) -> i128 {
        storage::get_monthly_fee(&e)
    }

    /// Returns the latest lockup id
    ///
    /// # Examples
    ///
    /// ```
    /// let id = fluxity_client::get_latest_stream_id();
    /// ```
    fn get_latest_lockup_id(e: Env) -> u64 {
        storage::get_latest_lockup_id(&e)
    }

    fn calculate_fee(e: Env, start_date: u64, end_date: u64) -> i128 {
        let monthly_fee = storage::get_monthly_fee(&e);
        calculate_lockup_fee(start_date, end_date, monthly_fee)
    }

    /// Returns a lockup by id
    ///
    /// # Examples
    ///
    /// ```
    /// let lockup_id = 20;
    ///
    /// fluxity_client::get_lockup(&stream_id);
    /// ```
    fn get_lockup(e: Env, id: u64) -> Result<types::Lockup, errors::CustomErrors> {
        match e.storage().persistent().get(&data_key::DataKey::Lockup(id)) {
            None => Err(errors::CustomErrors::LockupNotFound),
            Some(lockup) => Ok(lockup),
        }
    }

    /// Creates an stream
    ///
    /// # Examples
    ///
    /// ```
    /// let params = StreamInput {
    ///     sender: Address::random(&env),
    ///     receiver: Address::random(&env),
    ///     token: Address::random(&env),
    ///     amount: 20000000,
    ///     start_date: now,
    ///     cancellable_date: now,
    ///     cliff_date: now + 100,
    ///     end_date: now + 1000,
    ///     rate: Rate::Daily
    /// };
    ///
    /// fluxity_client::create_stream(&params);
    /// ```
    fn create_stream(e: Env, params: types::LockupInput) -> Result<u64, errors::CustomErrors> {
        if params.amount <= 0 {
            return Err(errors::CustomErrors::InvalidAmount);
        }

        if &params.sender == &params.receiver {
            return Err(errors::CustomErrors::InvalidReceiver);
        }

        if &params.start_date >= &params.end_date {
            return Err(errors::CustomErrors::InvalidStartDate);
        }

        if &params.cancellable_date > &params.end_date {
            return Err(errors::CustomErrors::InvalidCancellableDate);
        }

        if &params.cliff_date < &params.start_date || &params.cliff_date > &params.end_date {
            return Err(errors::CustomErrors::InvalidCliffDate);
        }

        take_xlm_fee(
            &e,
            params.start_date,
            params.end_date,
            params.sender.clone(),
        );

        token::transfer_from(&e, &params.token, &params.sender, &params.amount);

        let id = storage::get_latest_lockup_id(&e);
        let mut lockup: types::Lockup = params.into();

        lockup.is_vesting = false;

        storage::set_lockup(&e, id, &lockup);
        storage::increment_latest_lockup_id(&e, &id);
        events::publish_lockup_created_event(&e, id);

        Ok(id)
    }

    /// Cancels a lockup
    ///
    /// # Examples
    ///
    /// ```
    /// let lockup_id = 20;
    ///
    /// fluxity_client::cancel_lockup(&lockup_id);
    /// ```
    fn cancel_lockup(e: Env, id: u64) -> Result<(i128, i128), errors::CustomErrors> {
        let mut lockup = storage::get_lockup_by_id(&e, &id).unwrap();

        lockup.sender.require_auth();

        if lockup.is_cancelled {
            return Err(errors::CustomErrors::LockupAlreadyCanceled);
        }

        let current_date = e.ledger().timestamp();

        if lockup.end_date <= current_date {
            return Err(errors::CustomErrors::LockupAlreadySettled);
        }

        if lockup.cancellable_date > current_date {
            return Err(errors::CustomErrors::LockupNotCancellableYet);
        }

        let mut amounts = utils::calculate_stream_amounts(
            lockup.start_date,
            lockup.end_date,
            lockup.cliff_date,
            current_date,
            lockup.amount,
        );

        if lockup.is_vesting {
            amounts = utils::calculate_vesting_amounts(
                lockup.start_date,
                lockup.end_date,
                lockup.cliff_date,
                current_date,
                lockup.rate,
                lockup.amount,
            );
        }

        let sender_amount = amounts.sender_amount;
        let receiver_amount = amounts.receiver_amount - lockup.withdrawn;

        lockup.is_cancelled = true;
        lockup.cancelled_date = current_date;
        lockup.withdrawn = amounts.receiver_amount;

        storage::set_lockup(&e, id, &lockup);

        if receiver_amount > 0 {
            token::transfer(&e, &lockup.token, &lockup.receiver, &receiver_amount);
        }

        if sender_amount > 0 {
            token::transfer(&e, &lockup.token, &lockup.sender, &sender_amount);
        }

        events::publish_lockup_cancelled_event(&e, id);

        Ok((sender_amount, receiver_amount))
    }

    /// Withdraws from a lockup, anyone call call this function even for others
    ///
    /// # Examples
    ///
    /// ```
    /// let lockup_id = 20;
    /// let amount_to_withdraw = 30000000 // Represents 3 in a 7-decimal token
    ///
    /// fluxity_client::withdraw_lockup(&stream_id, &amount_to_withdraw);
    /// ```
    fn withdraw_lockup(e: Env, id: u64, amount: i128) -> Result<i128, errors::CustomErrors> {
        let mut lockup = storage::get_lockup_by_id(&e, &id).unwrap();

        if amount < 0 {
            return Err(errors::CustomErrors::AmountUnderflows);
        }

        if lockup.is_cancelled {
            return Err(errors::CustomErrors::LockupIsCanceled);
        }

        let current_date = e.ledger().timestamp();

        if current_date <= lockup.start_date {
            return Err(errors::CustomErrors::LockupNotStartedYet);
        }

        if current_date <= lockup.cliff_date {
            return Ok(0);
        }

        let mut amounts = utils::calculate_stream_amounts(
            lockup.start_date,
            lockup.end_date,
            lockup.cliff_date,
            current_date,
            lockup.amount,
        );

        if lockup.is_vesting {
            amounts = utils::calculate_vesting_amounts(
                lockup.start_date,
                lockup.end_date,
                lockup.cliff_date,
                current_date,
                lockup.rate,
                lockup.amount,
            );
        }

        let withdrawable = amounts.receiver_amount - lockup.withdrawn;

        if withdrawable < amount {
            return Err(errors::CustomErrors::SpecifiedAmountIsGreaterThanWithdrawable);
        }

        let mut amount_to_transfer = amount;

        if amount == 0 {
            amount_to_transfer = withdrawable;
        }

        lockup.withdrawn = lockup.withdrawn + amount_to_transfer;

        storage::set_lockup(&e, id, &lockup);

        token::transfer(&e, &lockup.token, &lockup.receiver, &amount_to_transfer);

        events::publish_lockup_withdrawn_event(&e, id);

        Ok(amount_to_transfer)
    }

    /// Creates a vesting stream
    ///
    /// # Examples
    ///
    /// ```
    /// let params = VestingInput {
    ///     sender: Address::random(&env),
    ///     receiver: Address::random(&env),
    ///     token: Address::random(&env),
    ///     amount: 20000000,
    ///     start_date: now,
    ///     cancellable_date: now,
    ///     cliff_date: now + 100,
    ///     end_date: now + 1000,
    ///     rate: Rate::Daily
    /// };
    ///
    /// fluxity_client::create_vesting(&params);
    /// ```
    fn create_vesting(e: Env, params: types::LockupInput) -> Result<u64, errors::CustomErrors> {
        if params.amount <= 0 {
            return Err(errors::CustomErrors::InvalidAmount);
        }

        if &params.sender == &params.receiver {
            return Err(errors::CustomErrors::InvalidReceiver);
        }

        if &params.start_date >= &params.end_date {
            return Err(errors::CustomErrors::InvalidStartDate);
        }

        if &params.cancellable_date > &params.end_date {
            return Err(errors::CustomErrors::InvalidCancellableDate);
        }

        if &params.cliff_date < &params.start_date || &params.cliff_date > &params.end_date {
            return Err(errors::CustomErrors::InvalidCliffDate);
        }

        take_xlm_fee(
            &e,
            params.start_date,
            params.end_date,
            params.sender.clone(),
        );

        token::transfer_from(&e, &params.token, &params.sender, &params.amount);

        let id = storage::get_latest_lockup_id(&e);
        let mut lockup: types::Lockup = params.into();

        lockup.is_vesting = true;

        storage::set_lockup(&e, id, &lockup);
        storage::increment_latest_lockup_id(&e, &id);
        events::publish_lockup_created_event(&e, id);

        Ok(id)
    }

    /// Increases the duration and the amount of a lockup
    ///
    /// # Examples
    ///
    /// ```
    /// let lockup_id = 56;
    /// let adding_amount = 700000000;
    ///
    /// fluxity_client::topup_lockup(lockup_id, adding_amount);
    /// ```
    fn topup_lockup(e: Env, id: u64, adding_amount: i128) -> Result<i128, errors::CustomErrors> {
        let mut lockup = get_lockup_by_id(&e, &id).unwrap();

        lockup.sender.require_auth();

        if lockup.is_cancelled {
            return Err(errors::CustomErrors::LockupIsCanceled);
        }

        let current_date = e.ledger().timestamp();

        if lockup.end_date < current_date {
            return Err(errors::CustomErrors::LockupAlreadySettled);
        }

        let additional_duration = calculate_additional_time(&lockup, adding_amount);

        if lockup.cancelled_date == lockup.end_date {
            lockup.cancellable_date = lockup.cancellable_date + additional_duration;
        }

        if lockup.cliff_date == lockup.end_date {
            lockup.cliff_date = lockup.cliff_date + additional_duration;
        }

        lockup.amount = lockup.amount + adding_amount;
        lockup.end_date = lockup.end_date + additional_duration;

        storage::set_lockup(&e, id, &lockup);

        events::publish_lockup_topup_event(&e, id);

        Ok(lockup.amount)
    }
}
