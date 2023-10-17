use soroban_sdk::{testutils::Address as _, token::Client, Address, Env};

use crate::{base::errors, Fluxity, FluxityClient};

struct CreateStreamTest<'a> {
    env: Env,
    admin: Address,
    token: Client<'a>,
    amount: i128,
    contract: FluxityClient<'a>,
}

impl<'a> CreateStreamTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::random(&env);

        let token_id = env.register_stellar_asset_contract(admin.clone());
        let token_client = soroban_sdk::token::Client::new(&env, &token_id);
        let token_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id);

        let contract_id = env.register_contract(None, Fluxity);
        let client = FluxityClient::new(&env, &contract_id);

        let amount = 2_0000_000;

        token_admin_client.mint(&admin, &amount);

        token_client.approve(&admin, &client.address, &amount, &6311000);

        Self {
            env,
            admin,
            amount,
            contract: client,
            token: token_client,
        }
    }
}

#[test]
fn test_stream_should_be_created() {
    let vars = CreateStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount / 2,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
    };

    let id = vars.contract.create_stream(&params);

    assert_eq!(vars.token.balance(&vars.admin), vars.amount / 2);
    assert_eq!(vars.token.decimals(), 7);
    assert_eq!(id, 0);
}

#[test]
fn test_second_stream_should_have_incremented_id() {
    let vars = CreateStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount / 2,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
    };

    let id0 = vars.contract.create_stream(&params);
    let id1 = vars.contract.create_stream(&params);

    assert_eq!(vars.token.balance(&vars.admin), 0);
    assert_eq!(id0, 0);
    assert_eq!(id1, 1);
}

#[test]
fn test_stream_should_revert_when_start_date_is_equal_to_end_date() {
    let vars = CreateStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
    };

    assert_eq!(
        vars.contract.try_create_stream(&params),
        Err(Ok(errors::CustomErrors::InvalidStartDate))
    );
}

#[test]
fn test_stream_should_revert_when_start_date_is_greater_than_end_date() {
    let vars = CreateStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now,
        cliff_date: now + 2,
        start_date: now + 2,
        end_date: now,
    };

    assert_eq!(
        vars.contract.try_create_stream(&params),
        Err(Ok(errors::CustomErrors::InvalidStartDate))
    );
}

#[test]
fn test_stream_should_revert_when_cancellable_date_is_less_than_start_date() {
    let vars = CreateStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now + 100,
        end_date: now + 200,
    };

    assert_eq!(
        vars.contract.try_create_stream(&params),
        Err(Ok(errors::CustomErrors::InvalidCancellableDate))
    );
}

#[test]
fn test_stream_should_revert_when_cliff_date_is_less_than_start_date() {
    let vars = CreateStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: vars.amount,
        cancellable_date: now + 100,
        cliff_date: now,
        start_date: now + 100,
        end_date: now + 200,
    };

    assert_eq!(
        vars.contract.try_create_stream(&params),
        Err(Ok(errors::CustomErrors::InvalidCliffDate))
    );
}

#[test]
fn test_stream_should_revert_when_amount_is_zero() {
    let vars = CreateStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: 0,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
    };

    assert_eq!(
        vars.contract.try_create_stream(&params),
        Err(Ok(errors::CustomErrors::InvalidAmount))
    );
}

#[test]
fn test_stream_should_revert_when_amount_is_negative() {
    let vars = CreateStreamTest::setup();

    let receiver = Address::random(&vars.env);
    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver,
        token: vars.token.address.clone(),
        amount: -100,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
    };

    assert_eq!(
        vars.contract.try_create_stream(&params),
        Err(Ok(errors::CustomErrors::InvalidAmount))
    );
}

#[test]
fn test_stream_should_revert_when_sender_and_receiver_are_the_same_address() {
    let vars = CreateStreamTest::setup();

    let now = vars.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: vars.admin.clone(),
        receiver: vars.admin.clone(),
        token: vars.token.address.clone(),
        amount: 100,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
    };

    assert_eq!(
        vars.contract.try_create_stream(&params),
        Err(Ok(errors::CustomErrors::InvalidReceiver))
    );
}
