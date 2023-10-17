use soroban_sdk::{token::Client, Address, Env};

pub fn transfer(e: &Env, token: &Address, to: &Address, amount: &i128) {
    Client::new(&e, &token).transfer(&e.current_contract_address(), &to, &amount);
}

pub fn transfer_from(e: &Env, token: &Address, from: &Address, amount: &i128) {
    Client::new(&e, &token).transfer_from(
        &e.current_contract_address(),
        &from,
        &e.current_contract_address(),
        &amount,
    );
}
