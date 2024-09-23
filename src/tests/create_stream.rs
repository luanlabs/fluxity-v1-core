use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    Address, IntoVal,
};

use crate::{base::errors, tests::setup::SetupStreamTest};

#[test]
fn test_stream_should_be_created() {
    let amount: i128 = 2000;
    let vars = SetupStreamTest::setup(amount);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let admin_balance = vars.token.balance(&vars.admin);

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    let id = vars.contract.create_lockup(&params);

    assert_eq!(id, 0);
    assert_eq!(vars.token.decimals(), 7);
    assert_eq!(vars.token.balance(&vars.admin), admin_balance - amount);
    assert_eq!(vars.token.balance(&vars.contract.address), vars.amount);
}

#[test]
fn test_stream_should_be_created_and_id_should_increment() {
    let amount: i128 = 2000;
    let vars = SetupStreamTest::setup(2000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let admin_balance = vars.token.balance(&vars.admin);

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    assert_eq!(vars.contract.get_latest_lockup_id(), 0);
    let id = vars.contract.create_lockup(&params);
    assert_eq!(vars.contract.get_latest_lockup_id(), 1);

    assert_eq!(id, 0);
    assert_eq!(vars.token.decimals(), 7);
    assert_eq!(vars.token.balance(&vars.admin), admin_balance - amount);
    assert_eq!(vars.token.balance(&vars.contract.address), vars.amount);
}

#[test]
fn test_stream_should_be_created_and_id_should_increment_by_200() {
    let vars = SetupStreamTest::setup(200_000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: 1,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    for i in 0..100 {
        assert_eq!(vars.contract.get_latest_lockup_id(), i);
        vars.contract.create_lockup(&params);
        assert_eq!(vars.contract.get_latest_lockup_id(), i + 1);
    }
}

#[test]
fn test_create_stream_should_emit_events() {
    let vars = SetupStreamTest::setup(2000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    vars.contract.create_lockup(&params);

    let events = vars.env.events().all();
    assert!(events.contains((
        vars.contract.address.clone(),
        (symbol_short!("LOCKUP"), symbol_short!("CREATED")).into_val(&vars.env),
        0u64.into_val(&vars.env)
    )));
}

#[test]
fn test_second_stream_should_have_incremented_id() {
    let amount: i128 = 2000;
    let vars = SetupStreamTest::setup(amount);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let admin_balance = vars.token.balance(&vars.admin);

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount / 2,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    let id0 = vars.contract.create_lockup(&params);
    let id1 = vars.contract.create_lockup(&params);

    assert_eq!(vars.token.balance(&vars.admin), admin_balance - amount);
    assert_eq!(id0, 0);
    assert_eq!(id1, 1);
}

#[test]
fn test_stream_should_revert_when_start_date_is_equal_to_end_date() {
    let vars = SetupStreamTest::setup(2000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    assert_eq!(
        vars.contract.try_create_lockup(&params),
        Err(Ok(errors::CustomErrors::InvalidStartDate))
    );
}

#[test]
fn test_stream_should_revert_when_start_date_is_greater_than_end_date() {
    let vars = SetupStreamTest::setup(2000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now,
        cliff_date: now + 2,
        start_date: now + 2,
        end_date: now,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    assert_eq!(
        vars.contract.try_create_lockup(&params),
        Err(Ok(errors::CustomErrors::InvalidStartDate))
    );
}

#[test]
fn test_stream_should_revert_when_cliff_date_is_less_than_start_date() {
    let vars = SetupStreamTest::setup(2000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now + 100,
        cliff_date: now,
        start_date: now + 100,
        end_date: now + 200,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    assert_eq!(
        vars.contract.try_create_lockup(&params),
        Err(Ok(errors::CustomErrors::InvalidCliffDate))
    );
}

#[test]
fn test_stream_should_revert_when_amount_is_zero() {
    let vars = SetupStreamTest::setup(2000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: 0,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    assert_eq!(
        vars.contract.try_create_lockup(&params),
        Err(Ok(errors::CustomErrors::InvalidAmount))
    );
}

#[test]
fn test_stream_should_revert_when_amount_is_negative() {
    let vars = SetupStreamTest::setup(2000);

    let receiver = Address::generate(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: -100,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    assert_eq!(
        vars.contract.try_create_lockup(&params),
        Err(Ok(errors::CustomErrors::InvalidAmount))
    );
}

#[test]
fn test_stream_should_revert_when_sender_and_receiver_are_the_same_address() {
    let vars = SetupStreamTest::setup(2000);

    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver: vars.admin.clone(),
        token: vars.token.address.clone(),
        amount: 100,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
        rate: crate::base::types::Rate::Monthly,
        is_vesting: false,
    };

    assert_eq!(
        vars.contract.try_create_lockup(&params),
        Err(Ok(errors::CustomErrors::InvalidReceiver))
    );
}
