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
    let test = CreateStreamTest::setup();

    let receiver = Address::random(&test.env);
    let now = test.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: test.admin.clone(),
        receiver,
        token: test.token.address.clone(),
        amount: test.amount,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
    };

    let id = test.contract.create_stream(&params);

    assert_eq!(test.token.balance(&test.admin), 0);
    assert_eq!(test.token.decimals(), 7);
    assert_eq!(id, 0);
}

#[test]
fn test_stream_should_revert_if_start_date_is_equal_to_end_date() {
    let test = CreateStreamTest::setup();

    let receiver = Address::random(&test.env);
    let now = test.env.ledger().timestamp();

    let params = crate::base::types::LinearStreamInputType {
        sender: test.admin.clone(),
        receiver,
        token: test.token.address.clone(),
        amount: test.amount,
        cancellable_date: now,
        cliff_date: now,
        start_date: now,
        end_date: now,
    };

    assert_eq!(
        test.contract.try_create_stream(&params),
        Err(Ok(errors::CustomErrors::InvalidStartDate))
    );

    // assert_eq!(test.token.balance(&test.admin), 0);
    // assert_eq!(test.token.decimals(), 7);
    // assert_eq!(id, 0);
}
