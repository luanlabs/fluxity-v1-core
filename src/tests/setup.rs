use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token::Client,
    Address, Env,
};

use crate::{Fluxity, FluxityClient};

pub struct SetupStreamTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub token: Client<'a>,
    pub amount: i128,
    pub contract: FluxityClient<'a>,
}

impl<'a> SetupStreamTest<'a> {
    pub fn setup(amount: i128) -> Self {
        let env = Env::default();

        env.mock_all_auths();

        let admin = Address::random(&env);

        let token_id = env.register_stellar_asset_contract(admin.clone());
        let token_client = soroban_sdk::token::Client::new(&env, &token_id);
        let token_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id);

        let contract_id = env.register_contract(None, Fluxity);
        let client = FluxityClient::new(&env, &contract_id);

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

    pub fn setup_with_stream_created(
        amount: i128,
        cancellable_date: u64,
        end_date: u64,
    ) -> (Self, u64) {
        let vars = Self::setup(amount);

        let receiver = Address::random(&vars.env);
        let now = vars.env.ledger().timestamp();

        let params = crate::base::types::LinearStreamInputType {
            sender: vars.admin.clone(),
            receiver,
            token: vars.token.address.clone(),
            amount: vars.amount,
            cliff_date: now,
            start_date: now,
            end_date: now + end_date,
            cancellable_date: now + cancellable_date,
            rate: crate::base::types::Rate::Monthly,
        };

        let id = vars.contract.create_stream(&params);

        assert_eq!(vars.contract.get_stream(&0).sender, vars.admin.clone());
        assert_eq!(vars.token.decimals(), 7);
        assert_eq!(vars.token.balance(&vars.admin), 0);
        assert_eq!(vars.token.balance(&vars.contract.address), vars.amount);

        (vars, id)
    }

    pub fn move_ledger_timestamp_to(&self, timestamp: u64) {
        self.env.ledger().set(LedgerInfo {
            timestamp,
            ..self.env.ledger().get()
        });
    }
}
