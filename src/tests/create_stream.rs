use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{Fluxity, FluxityClient};

#[test]
fn test_if_we_can_create_an_stream() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Fluxity);
    let client = FluxityClient::new(&env, &contract_id);

    let amount = 200;
    let sender = Address::random(&env);
    let receiver = Address::random(&env);
    let now = env.ledger().timestamp();

    let token_id = env.register_stellar_asset_contract(sender.clone());
    let token_client = soroban_sdk::token::Client::new(&env, &token_id);
    let token_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id);

    token_admin_client.mint(&sender, &amount);

    token_client.approve(&sender, &client.address, &amount, &6311000);

    let params = crate::base::types::LinearStreamInputType {
        sender: sender.clone(),
        receiver,
        token: token_admin_client.address,
        amount,
        cancellable_date: now,
        cliff_date: now + 100,
        start_date: now,
        end_date: now + 1000,
    };

    let id = client.create_stream(&params);

    assert_eq!(token_client.balance(&sender), 0);
    assert_eq!(token_client.decimals(), 7);
    assert_eq!(id, 0);
}
