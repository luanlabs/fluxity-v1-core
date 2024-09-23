use soroban_sdk::{
    testutils::{Address as _, MockAuth},
    Address, IntoVal, Val, Vec,
};

use crate::tests::setup::SetupStreamTest;

#[test]
fn test_no_xlm_fee_should_be_taken() {
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

    let admin_xlm_balance_before = vars.xlm.balance(&vars.admin);

    let params = crate::base::types::LockupInput {
        receiver,
        end_date,
        start_date,
        is_vesting: false,
        amount: vars.amount,
        cliff_date: end_date,
        sender: sender.clone(),
        spender: sender.clone(),
        cancellable_date: start_date,
        token: vars.token.address.clone(),
        rate: crate::base::types::Rate::Monthly,
    };

    let id = vars.contract.create_lockup(&params);

    // let id = vars
    //     .contract
    //     .mock_auths(&[MockAuth {
    //         address: &sender,
    //         invoke: &soroban_sdk::testutils::MockAuthInvoke {
    //             contract: &vars.contract.address,
    //             fn_name: "create_lockup",
    //             args: params.to_vec_val(&vars.env),
    //             sub_invokes: &[],
    //         },
    //     }])
    //     .create_lockup(&params);
    //
    let admin_xlm_balance_after = vars.xlm.balance(&vars.admin);

    assert_eq!(id, 0);
    assert_eq!(vars.token.balance(&sender), 0);
    assert_eq!(admin_xlm_balance_after, admin_xlm_balance_before);
}

#[test]
fn test_xlm_fee_should_be_taken() {
    let amount: i128 = 2000;
    let vars = SetupStreamTest::setup(amount);

    let sender = Address::generate(&vars.env);
    let receiver = Address::generate(&vars.env);

    let xlm_fee_one_month: i128 = 10000000;
    let start_date = vars.env.ledger().timestamp();
    let end_date = start_date + 1000;

    vars.contract.set_monthly_fee(&xlm_fee_one_month);

    vars.token.transfer(&vars.admin, &sender, &amount);
    vars.xlm.transfer(&vars.admin, &sender, &xlm_fee_one_month);

    vars.token
        .approve(&sender, &vars.contract.address, &amount, &6311000);
    vars.xlm.approve(
        &sender,
        &vars.contract.address,
        &xlm_fee_one_month,
        &6311000,
    );

    assert_eq!(vars.token.balance(&sender), amount);
    assert_eq!(vars.xlm.balance(&sender), xlm_fee_one_month);

    let params = crate::base::types::LockupInput {
        receiver,
        end_date,
        start_date,
        is_vesting: false,
        amount: vars.amount,
        cliff_date: end_date,
        sender: sender.clone(),
        spender: sender.clone(),
        cancellable_date: start_date,
        token: vars.token.address.clone(),
        rate: crate::base::types::Rate::Monthly,
    };

    let xlm_address = vars.contract.get_xlm();
    let admin_address = vars.contract.get_admin();
    let monthly_fee = vars.contract.get_monthly_fee();

    assert_eq!(admin_address, vars.admin);
    assert_eq!(xlm_address, vars.xlm.address);
    assert_eq!(monthly_fee, xlm_fee_one_month);

    let x1 = vars.xlm.balance(&vars.contract.address);
    let admin_xlm_balance_before = vars.xlm.balance(&vars.admin);

    let id = vars.contract.create_lockup(&params);

    let x2 = vars.xlm.balance(&vars.contract.address);
    let admin_xlm_balance_after = vars.xlm.balance(&vars.admin);

    assert_eq!(x1, x2);

    assert_eq!(id, 0);
    assert_eq!(vars.token.balance(&sender), 0);
    assert_eq!(
        admin_xlm_balance_after,
        admin_xlm_balance_before + xlm_fee_one_month
    );
}

#[test]
fn test_xlm_fee_should_be_taken_multiple_times_for_nth_month() {
    let amount: i128 = 2000;
    let vars = SetupStreamTest::setup(amount);

    let sender = Address::generate(&vars.env);
    let receiver = Address::generate(&vars.env);

    let xlm_fee_one_month: i128 = 10000000;
    let start_date = vars.env.ledger().timestamp();
    let end_date = start_date + (86400 * 45);

    vars.contract.set_monthly_fee(&xlm_fee_one_month);

    vars.token.transfer(&vars.admin, &sender, &amount);
    vars.xlm
        .transfer(&vars.admin, &sender, &(xlm_fee_one_month * 2));

    vars.token
        .approve(&sender, &vars.contract.address, &amount, &6311000);
    vars.xlm.approve(
        &sender,
        &vars.contract.address,
        &(xlm_fee_one_month * 2),
        &6311000,
    );

    assert_eq!(vars.token.balance(&sender), amount);
    assert_eq!(vars.xlm.balance(&sender), xlm_fee_one_month * 2);

    let admin_xlm_balance_before = vars.xlm.balance(&vars.admin);

    let params = crate::base::types::LockupInput {
        receiver,
        end_date,
        start_date,
        is_vesting: false,
        amount: vars.amount,
        cliff_date: end_date,
        sender: sender.clone(),
        spender: sender.clone(),
        cancellable_date: start_date,
        token: vars.token.address.clone(),
        rate: crate::base::types::Rate::Monthly,
    };

    let id = vars.contract.create_lockup(&params);

    let admin_xlm_balance_after = vars.xlm.balance(&vars.admin);

    assert_eq!(id, 0);
    assert_eq!(vars.token.balance(&sender), 0);
    assert_eq!(
        admin_xlm_balance_after,
        admin_xlm_balance_before + xlm_fee_one_month * 2
    );
}
