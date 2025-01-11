use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    vec, Address, IntoVal, Val, Vec,
};

use super::setup::SetupStreamTest;

#[test]
fn test_take_zero_fee() {
    let vars = SetupStreamTest::setup(1000);

    let start_date = vars.env.ledger().timestamp();
    let end_date = start_date + 86400 * 10;

    let receiver = Address::random(&vars.env);

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: vars.admin.clone(),
        receiver,
        end_date,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cliff_date: end_date,
        start_date,
        cancellable_date: end_date,
        rate: crate::base::types::Rate::Monthly,
    };

    let balance_before = vars.xlm.balance(&vars.contract.get_admin());

    vars.contract.create_stream(&params);

    let balance_after = vars.xlm.balance(&vars.contract.get_admin());

    assert_eq!(balance_before, balance_after);
}

#[test]
fn test_take_fee_if_stream_is_more_than_one_month() {
    let vars = SetupStreamTest::setup(1000);

    let start_date = vars.env.ledger().timestamp();
    let end_date = start_date + (86400 * 50);
    let xlm_amount: i128 = 10000000;

    let receiver = Address::random(&vars.env);

    vars.contract.set_monthly_fee(&xlm_amount);

    let user = Address::random(&vars.env);

    // let auths: [MockAuth; 1] = [MockAuth {
    //     address: &user,
    //     invoke: &MockAuthInvoke {
    //         contract: &vars.contract.address,
    //         fn_name: "get_admin",
    //         args: ().into_val(&vars.env),
    //         sub_invokes: &[],
    //     },
    // }];
    //
    // vars.env.mock_auths(&auths);

    let balance_before = vars.xlm.balance(&vars.contract.get_admin());

    let params = crate::base::types::LockupInput {
        spender: vars.admin.clone(),
        sender: user.clone(),
        receiver,
        end_date,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cliff_date: end_date,
        start_date,
        cancellable_date: end_date,
        rate: crate::base::types::Rate::Monthly,
    };

    vars.token
        .transfer(&vars.admin.clone(), &user, &vars.amount);

    vars.xlm.transfer(&vars.admin.clone(), &user, &xlm_amount);

    vars.xlm
        .approve(&user, &vars.contract.address, &i128::MAX, &6311000);
    vars.token
        .approve(&user, &vars.contract.address, &i128::MAX, &6311000);

    assert_eq!(vars.xlm.balance(&user), xlm_amount);
    assert_eq!(vars.token.balance(&user), vars.amount);

    // vars.contract.create_stream(&params);

    // vars.contract
    //     .mock_auths(&[MockAuth {
    //         address: &user,
    //         invoke: &MockAuthInvoke {
    //             contract: &vars.contract.address,
    //             fn_name: "create_stream",
    //             args: (&params).into_val(&vars.env),
    //             sub_invokes: &[],
    //         },
    //     }])
    //     .create_stream(&params);

    // vars.token
    //     .mock_auths(&[MockAuth {
    //         address: &user,
    //         invoke: &MockAuthInvoke {
    //             contract: &vars.token.address,
    //             fn_name: "approve",
    //             args: (
    //                 &vars.admin,
    //                 &vars.contract.address,
    //                 &1000000000000000,
    //                 &6311000,
    //             )
    //                 .into_val(&vars.env),
    //             sub_invokes: &[],
    //         },
    //     }])
    //     .approve(
    //         &vars.admin.clone(),
    //         &vars.contract.address.clone(),
    //         &1000000000000000_i128,
    //         &6311000_u32,
    //     );
    //
    // // let create_stream_args: Vec<Val> = vec![&vars.env, params.into_val(&vars.env)];
    // // sub_invokes: &[MockAuthInvoke {
    // //     contract: &vars.contract.address,
    // //     fn_name: "create_stream",
    // //     sub_invokes: &[],
    // //     args: create_stream_args,
    // // }],
    //
    // let balance_after = vars.xlm.balance(&vars.contract.get_admin());
    // assert_eq!(balance_before, balance_after);
}
