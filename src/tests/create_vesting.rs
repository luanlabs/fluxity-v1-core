use soroban_sdk::{testutils::Address as _, Address};

use super::setup::SetupStreamTest;
use crate::base::{
    errors,
    types::{LockupInput, Rate},
};

#[test]
fn test_create_vesting_should_work() {
    let vars = SetupStreamTest::setup(1000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();
    let end_date = Rate::Daily as u64 * 2; // 2 days from now

    let params = LockupInput {
        amount: 1000,
        sender: vars.admin.clone(),
        receiver,
        end_date,
        cliff_date: now,
        start_date: now,
        cancellable_date: now,
        token: vars.token.address,
        rate: crate::base::types::Rate::Daily,
    };

    vars.contract.create_vesting(&params);
}

#[test]
fn test_create_vesting_should_store_is_vesting_to_true() {
    let vars = SetupStreamTest::setup(1000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();
    let end_date = Rate::Daily as u64 * 2; // 2 days from now

    let params = LockupInput {
        amount: 1000,
        sender: vars.admin.clone(),
        receiver,
        end_date,
        cliff_date: now,
        start_date: now,
        cancellable_date: now,
        token: vars.token.address,
        rate: crate::base::types::Rate::Daily,
    };

    let id = vars.contract.create_vesting(&params);

    let stream = vars.contract.get_lockup(&id);

    assert_eq!(stream.is_vesting, true);
}

#[test]
fn test_create_vesting_should_revert_when_amount_is_negative() {
    let vars = SetupStreamTest::setup(1000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();
    let end_date = Rate::Daily as u64 * 2; // 2 days from now

    let params = LockupInput {
        amount: -100,
        sender: vars.admin.clone(),
        receiver,
        end_date,
        cliff_date: now,
        start_date: now,
        cancellable_date: now,
        token: vars.token.address,
        rate: crate::base::types::Rate::Daily,
    };

    let result = vars.contract.try_create_vesting(&params);

    assert_eq!(result, Err(Ok(errors::CustomErrors::InvalidAmount)));
}

#[test]
fn test_create_vesting_should_revert_when_amount_is_zero() {
    let vars = SetupStreamTest::setup(1000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();
    let end_date = Rate::Daily as u64 * 2; // 2 days from now

    let params = LockupInput {
        amount: 0,
        sender: vars.admin.clone(),
        receiver,
        end_date,
        cliff_date: now,
        start_date: now,
        cancellable_date: now,
        token: vars.token.address,
        rate: crate::base::types::Rate::Daily,
    };

    let result = vars.contract.try_create_vesting(&params);

    assert_eq!(result, Err(Ok(errors::CustomErrors::InvalidAmount)));
}

#[test]
fn test_create_vesting_should_revert_when_sender_and_receiver_are_the_same() {
    let vars = SetupStreamTest::setup(1000);

    let now = vars.env.ledger().timestamp();
    let end_date = Rate::Daily as u64 * 2; // 2 days from now

    let params = LockupInput {
        amount: 1000,
        sender: vars.admin.clone(),
        receiver: vars.admin.clone(),
        end_date,
        cliff_date: now,
        start_date: now,
        cancellable_date: now,
        token: vars.token.address,
        rate: crate::base::types::Rate::Daily,
    };

    let result = vars.contract.try_create_vesting(&params);

    assert_eq!(result, Err(Ok(errors::CustomErrors::InvalidReceiver)));
}

#[test]
fn test_create_vesting_should_revert_when_start_date_is_equal_to_end_date() {
    let vars = SetupStreamTest::setup(1000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();
    // let end_date = Rate::Daily as u64 * 2; // 2 days from now

    let params = LockupInput {
        amount: 1000,
        sender: vars.admin.clone(),
        receiver,
        end_date: now,
        cliff_date: now,
        start_date: now,
        cancellable_date: now,
        token: vars.token.address,
        rate: crate::base::types::Rate::Daily,
    };

    let result = vars.contract.try_create_vesting(&params);

    assert_eq!(result, Err(Ok(errors::CustomErrors::InvalidStartDate)));
}

#[test]
fn test_create_vesting_should_revert_when_cancellable_date_is_greater_than_end_date() {
    let vars = SetupStreamTest::setup(1000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();
    let end_date = Rate::Daily as u64 * 2; // 2 days from now

    let params = LockupInput {
        amount: 1000,
        sender: vars.admin.clone(),
        receiver,
        end_date,
        cliff_date: now,
        start_date: now,
        cancellable_date: end_date + 1,
        token: vars.token.address,
        rate: crate::base::types::Rate::Daily,
    };

    let result = vars.contract.try_create_vesting(&params);

    assert_eq!(
        result,
        Err(Ok(errors::CustomErrors::InvalidCancellableDate))
    );
}
