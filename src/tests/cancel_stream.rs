use soroban_sdk::{testutils::Address as _, Address};

use crate::{base::errors, tests::setup::SetupStreamTest};

fn setup_cancel_stream(cancellable_date: u64) -> u64 {
    let vars = SetupStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount / 2,
        cliff_date: now,
        start_date: now,
        end_date: now + 1000,
        cancellable_date: now + cancellable_date,
    };

    let id = vars.contract.create_stream(&params);

    id
}

#[test]
fn test_stream_should_be_cancelled_after_creation() {
    let vars = SetupStreamTest::setup();
    let id = setup_cancel_stream(0);

    let amounts = vars.contract.try_cancel_stream(&id);

    assert_eq!(
        amounts,
        Err(Ok(errors::CustomErrors::StreamNotCancellableYet))
    );
}
