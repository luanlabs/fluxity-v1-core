use soroban_sdk::{token::Client, Address, Env};

pub fn transfer(e: &Env, token: &Address, to: &Address, amount: &i128) {
    Client::new(&e, &token).transfer(&e.current_contract_address(), &to, &amount);
}
