use soroban_sdk::{testutils::Address as _, token::Client, Address, Env};

use crate::{Fluxity, FluxityClient};

pub struct SetupStreamTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub token: Client<'a>,
    pub amount: i128,
    pub contract: FluxityClient<'a>,
}

impl<'a> SetupStreamTest<'a> {
    pub fn setup() -> Self {
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
