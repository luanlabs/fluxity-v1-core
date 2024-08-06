use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    Address, IntoVal,
};

use crate::{base::errors, tests::setup::SetupStreamTest};

#[test]
fn test_take_xlm_fee() {
    let vars = SetupStreamTest::setup(2000);

    let receiver = Address::generate(&vars.env);
    let start_date = vars.env.ledger().timestamp();
    let end_date = start_date + 1000;

    let params = crate::base::types::LockupInput {
        receiver,
        end_date,
        start_date,
        is_vesting: false,
        amount: vars.amount,
        cliff_date: end_date,
        sender: vars.admin.clone(),
        cancellable_date: start_date,
        token: vars.token.address.clone(),
        rate: crate::base::types::Rate::Monthly,
    };

    let id = vars.contract.create_lockup(&params);

    assert_eq!(id, 0);
    assert_eq!(vars.token.decimals(), 7);
    assert_eq!(vars.token.balance(&vars.admin), 0);
    assert_eq!(vars.token.balance(&vars.contract.address), vars.amount);
}

Hey guys. I want to write a test for my application and want to call a contract function by
2 different users. How can I do that in Soroban tests?
