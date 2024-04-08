use crate::base::errors::CustomErrors;

use super::setup::{SetupStreamTest, StreamFields};

#[test]
fn test_stream_should_be_withdrawable_by_receiver() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(50);

    let amount = vars.contract.withdraw_lockup(&id, &0);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(amount, 500);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 500);
    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 500);
}

#[test]
fn test_withdraw_stream_should_take_everything_at_end_date() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(100);

    let amount = vars.contract.withdraw_lockup(&id, &0);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(amount, 1000);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 1000);
    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 0);
}

#[test]
fn test_withdraw_stream_should_take_everything_after_end_date() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(200);

    let amount = vars.contract.withdraw_lockup(&id, &0);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(amount, 1000);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 1000);
    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 0);
}

#[test]
fn test_withdraw_stream_should_be_callable_multiple_times_with_custom_amount() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(50);

    let amount0 = vars.contract.withdraw_lockup(&id, &100);
    let amount1 = vars.contract.withdraw_lockup(&id, &100);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(amount0, 100);
    assert_eq!(amount1, 100);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 200);
    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 800);
}

#[test]
fn test_withdraw_stream_should_be_callable_multiple_times() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(50);

    let amount0 = vars.contract.withdraw_lockup(&id, &100);
    let amount1 = vars.contract.withdraw_lockup(&id, &0);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(amount0, 100);
    assert_eq!(amount1, 400);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 500);
    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 500);
}

#[test]
fn test_withdraw_stream_should_return_zero_if_called_before_cliff_date() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields {
        cliff_date: 50,
        ..Default::default()
    });

    vars.move_ledger_timestamp_to(50);

    let amount = vars.contract.withdraw_lockup(&id, &0);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(amount, 0);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 0);
    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 1000);
}

#[test]
fn test_withdraw_stream_should_bounce_if_called_after_cliff_date() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields {
        cliff_date: 50,
        ..Default::default()
    });

    vars.move_ledger_timestamp_to(51);

    let amount = vars.contract.withdraw_lockup(&id, &0);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(amount, 510);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 510);
    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 490);
}

#[test]
fn test_withdraw_stream_should_change_stream_state() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(50);

    vars.contract.withdraw_lockup(&id, &0);

    let stream = vars.contract.get_lockup(&id);

    assert_eq!(stream.withdrawn, 500);
}

#[test]
fn test_withdraw_stream_should_revert_when_stream_is_not_started_yet() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    let result = vars.contract.try_withdraw_lockup(&id, &0);

    assert_eq!(result, Err(Ok(CustomErrors::LockupNotStartedYet)));
}

#[test]
fn test_withdraw_stream_should_revert_when_amount_is_negative() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    let result = vars.contract.try_withdraw_lockup(&id, &-10);

    assert_eq!(result, Err(Ok(CustomErrors::AmountUnderflows)));
}
