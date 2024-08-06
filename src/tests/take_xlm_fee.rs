use soroban_sdk::{
    testutils::{Address as _, MockAuth},
    Address, IntoVal, Val, Vec,
};

use crate::tests::setup::SetupStreamTest;

#[test]
fn test_take_xlm_fee() {
    let amount: i128 = 2000;
    let vars = SetupStreamTest::setup(amount);

    let sender = Address::generate(&vars.env);
    let receiver = Address::generate(&vars.env);

    let start_date = vars.env.ledger().timestamp();
    let end_date = start_date + 1000;

    vars.token
        .transfer(&vars.admin.clone(), &sender.clone(), &amount);

    let mut args: Vec<Val> = Vec::new(&vars.env);

    let el: u32 = 222;

    args.push_back(sender.clone().into_val(&vars.env));
    args.push_back(vars.contract.address.clone().into_val(&vars.env));
    args.push_back(amount.into_val(&vars.env));
    args.push_back(el.into_val(&vars.env));

    // from: Address, spender: Address, amount: i128, expiration_ledger: u32);

    vars.token
        .mock_auths(&[MockAuth {
            address: &sender,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: &vars.token.address,
                fn_name: "approve",
                args,
                sub_invokes: &[],
            },
        }])
        .approve(
            &sender.clone(),
            &vars.contract.address.clone(),
            &amount,
            &el,
        );

    let params = crate::base::types::LockupInput {
        receiver,
        end_date,
        start_date,
        is_vesting: false,
        amount: vars.amount,
        cliff_date: end_date,
        sender: sender.clone(),
        cancellable_date: start_date,
        token: vars.token.address.clone(),
        rate: crate::base::types::Rate::Monthly,
    };

    let id = vars
        .contract
        .mock_auths(&[MockAuth {
            address: &sender,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: &vars.contract.address,
                fn_name: "create_lockup",
                args: params.to_vec_val(&vars.env),
                sub_invokes: &[],
            },
        }])
        .create_lockup(&params);

    // test XLM?

    assert_eq!(id, 0);
    assert_eq!(vars.token.balance(&sender), 0);
}
