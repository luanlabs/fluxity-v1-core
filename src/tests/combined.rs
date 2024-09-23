use crate::base::errors;

use super::setup::{SetupStreamTest, StreamFields};

#[test]
fn test_cancel_stream_should_transfer_when_withdrawn_is_not_zero() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(50);

    let token_balance = vars.token.balance(&vars.admin);
    let withdraw_amount = vars.contract.withdraw_lockup(&id, &200);
    let amounts = vars.contract.cancel_lockup(&id);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(withdraw_amount, 200);
    assert_eq!(amounts.0, 500);
    assert_eq!(amounts.1, 300);

    assert_eq!(vars.token.balance(&vars.contract.address), 0);
    assert_eq!(
        vars.token.balance(&vars.admin),
        token_balance + vars.amount / 2
    );
    assert_eq!(vars.token.balance(&stream.receiver), 500);
}

#[test]
fn test_cancel_stream_should_not_transfer_when_stream_is_fully_withdrawn() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    vars.move_ledger_timestamp_to(100);

    let admin_balance = vars.token.balance(&vars.admin);

    vars.contract.withdraw_lockup(&id, &0);
    let stream = vars.contract.get_lockup(&id);

    assert_eq!(vars.token.balance(&vars.contract.address.clone()), 0);
    assert_eq!(vars.token.balance(&vars.admin.clone()), admin_balance);
    assert_eq!(vars.token.balance(&stream.receiver.clone()), 1000);

    let result = vars.contract.try_cancel_lockup(&id);

    assert_eq!(result, Err(Ok(errors::CustomErrors::LockupAlreadySettled)));
}
