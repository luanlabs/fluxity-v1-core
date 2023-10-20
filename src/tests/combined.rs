use crate::base::errors;

use super::setup::{SetupStreamTest, StreamFields};

#[test]
fn test_cancel_stream_should_transfer_when_withdrawn_is_not_zero() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(50);

    let withdraw_amount = vars.contract.withdraw_stream(&id, &200);
    let amounts = vars.contract.cancel_stream(&id);
    let stream = vars.contract.get_stream(&id);

    assert_eq!(withdraw_amount, 200);
    assert_eq!(amounts.0, 500);
    assert_eq!(amounts.1, 300);

    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 0);
    assert_eq!(vars.token.balance(&vars.admin.clone()), 500);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 500);
}

#[test]
fn test_cancel_stream_should_not_transfer_when_stream_is_fully_withdrawn() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(100);

    vars.contract.withdraw_stream(&id, &0);
    let stream = vars.contract.get_stream(&id);

    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 0);
    assert_eq!(vars.token.balance(&vars.admin.clone()), 0);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 1000);

    let result = vars.contract.try_cancel_stream(&id);

    assert_eq!(result, Err(Ok(errors::CustomErrors::StreamAlreadySettled)));
}
