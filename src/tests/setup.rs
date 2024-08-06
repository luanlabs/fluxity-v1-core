use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token::Client,
    Address, Env,
};

use crate::{
    base::types::{LockupInput, Rate},
    Fluxity, FluxityClient,
};

pub struct StreamFields {
    pub amount: i128,
    pub start_date: u64,
    pub end_date: u64,
    pub cliff_date: u64,
    pub cancellable_date: u64,
}

pub struct VestingFields {
    pub rate: Rate,
    pub amount: i128,
    pub start_date: u64,
    pub end_date: u64,
    pub cliff_date: u64,
    pub cancellable_date: u64,
}

impl Default for VestingFields {
    fn default() -> Self {
        Self {
            amount: 1000,
            start_date: 0,
            end_date: 100,
            cliff_date: 0,
            cancellable_date: 0,
            rate: Rate::Daily,
        }
    }
}

impl Default for StreamFields {
    fn default() -> Self {
        Self {
            amount: 1000,
            start_date: 0,
            end_date: 100,
            cliff_date: 0,
            cancellable_date: 0,
        }
    }
}

pub struct SetupStreamTest<'a> {
    pub env: Env,
    pub amount: i128,
    pub admin: Address,
    pub xlm: Client<'a>,
    pub token: Client<'a>,
    pub contract: FluxityClient<'a>,
}

impl<'a> SetupStreamTest<'a> {
    pub fn setup(amount: i128) -> Self {
        let env = Env::default();

        env.mock_all_auths();

        let admin = Address::generate(&env);

        let xlm_id = env.register_stellar_asset_contract(admin.clone());
        let xlm_client = soroban_sdk::token::Client::new(&env, &xlm_id);
        let xlm_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &xlm_id);

        let token_id = env.register_stellar_asset_contract(admin.clone());
        let token_client = soroban_sdk::token::Client::new(&env, &token_id);
        let token_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id);

        let contract_id = env.register_contract(None, Fluxity);
        let client = FluxityClient::new(&env, &contract_id);

        client.initialize(&admin, &xlm_client.address);
        client.set_monthly_fee(&0);

        xlm_admin_client.mint(&admin, &amount);
        token_admin_client.mint(&admin, &amount);

        token_client.approve(&admin, &client.address, &amount, &6311000);

        Self {
            env,
            admin,
            amount,
            xlm: xlm_client,
            contract: client,
            token: token_client,
        }
    }

    pub fn setup_with_stream_created(fields: StreamFields) -> (Self, u64) {
        let vars = Self::setup(fields.amount);

        let receiver = Address::generate(&vars.env);
        let now = vars.env.ledger().timestamp();

        let params = crate::base::types::LockupInput {
            sender: vars.admin.clone(),
            receiver,
            token: vars.token.address.clone(),
            amount: vars.amount,
            cliff_date: now + fields.cliff_date,
            start_date: now + fields.start_date,
            end_date: now + fields.end_date,
            cancellable_date: now + fields.cancellable_date,
            rate: crate::base::types::Rate::Monthly,
            is_vesting: false,
        };

        let id = vars.contract.create_lockup(&params);

        assert_eq!(vars.contract.get_lockup(&0).sender, vars.admin.clone());
        assert_eq!(vars.token.decimals(), 7);
        assert_eq!(vars.token.balance(&vars.admin), 0);
        assert_eq!(vars.token.balance(&vars.contract.address), vars.amount);

        (vars, id)
    }

    pub fn setup_with_vesting_created(fields: VestingFields) -> (Self, u64) {
        let vars = Self::setup(fields.amount);

        let receiver = Address::generate(&vars.env);
        let now = vars.env.ledger().timestamp();

        let params = LockupInput {
            sender: vars.admin.clone(),
            receiver,
            amount: fields.amount,
            rate: fields.rate,
            end_date: now + fields.end_date,
            cliff_date: now + fields.cliff_date,
            cancellable_date: now + fields.cancellable_date,
            start_date: now + fields.start_date,
            token: vars.token.address.clone(),
            is_vesting: true,
        };

        let id = vars.contract.create_lockup(&params);

        (vars, id)
    }

    pub fn move_ledger_timestamp_to(&self, timestamp: u64) {
        self.env.ledger().set(LedgerInfo {
            timestamp,
            ..self.env.ledger().get()
        });
    }
}
