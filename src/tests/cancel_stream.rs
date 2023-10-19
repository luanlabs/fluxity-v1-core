use soroban_sdk::{
    symbol_short,
    testutils::{Events, Ledger},
    IntoVal,
};

use crate::{base::errors, tests::setup::SetupStreamTest};

#[test]
fn test_stream_should_be_cancelled_after_creation() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(1000, 0, 100);

    let amounts = vars.contract.cancel_stream(&id);
    let stream = vars.contract.get_stream(&id);

    assert_eq!(vars.token.balance(&vars.contract.address), 0);
    assert_eq!(vars.token.balance(&vars.admin.clone()), vars.amount);
    assert_eq!(vars.token.balance(&vars.admin.clone()), amounts.0);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 0);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), amounts.1);
    assert!(stream.is_cancelled);
}

#[test]
fn test_cancel_stream_should_emit_event() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(1000, 0, 100);

    vars.contract.cancel_stream(&id);

    let events = vars.env.events().all();
    assert!(events.contains((
        vars.contract.address.clone(),
        (symbol_short!("STREAM"), symbol_short!("CANCELLED")).into_val(&vars.env),
        id.into_val(&vars.env)
    )))
}

#[test]
fn test_cancel_stream_should_transfer_tokens_to_both_sides() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(1000, 0, 100);

    let new_timestamp = 50;
    vars.move_ledger_timestamp_to(new_timestamp);
    assert_eq!(vars.env.ledger().get().timestamp, new_timestamp);

    let amounts = vars.contract.cancel_stream(&id);
    let stream = vars.contract.get_stream(&id);

    assert_eq!(vars.token.balance(&vars.contract.address), 0);
    assert_eq!(vars.token.balance(&vars.admin.clone()), vars.amount / 2);
    assert_eq!(vars.token.balance(&vars.admin.clone()), amounts.0);
    assert_eq!(
        vars.token.balance(&stream.receiver.clone()),
        vars.amount / 2
    );
    assert_eq!(vars.token.balance(&stream.receiver.clone()), amounts.1);
}

#[test]
fn test_cancel_stream_should_revert_when_stream_is_already_cancelled() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(1000, 0, 100);

    vars.contract.cancel_stream(&id);
    let result = vars.contract.try_cancel_stream(&id);

    assert_eq!(result, Err(Ok(errors::CustomErrors::StreamAlreadyCanceled)));
}

#[test]
fn test_cancel_stream_should_revert_when_stream_is_already_settled() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(1000, 0, 100);

    let new_timestamp = 100;
    vars.move_ledger_timestamp_to(new_timestamp);
    assert_eq!(vars.env.ledger().get().timestamp, new_timestamp);

    let result = vars.contract.try_cancel_stream(&id);

    assert_eq!(result, Err(Ok(errors::CustomErrors::StreamAlreadySettled)));
}

#[test]
fn test_cancel_stream_should_revert_when_cancellable_date_is_not_reached() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(1000, 50, 100);

    let new_timestamp = 20;
    vars.move_ledger_timestamp_to(new_timestamp);
    assert_eq!(vars.env.ledger().get().timestamp, new_timestamp);

    let result = vars.contract.try_cancel_stream(&id);

    assert_eq!(
        result,
        Err(Ok(errors::CustomErrors::StreamNotCancellableYet))
    );
}
