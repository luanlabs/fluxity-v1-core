use soroban_sdk::{token::Client, Address, Env};

use super::{
    storage::{get_monthly_fee, get_xlm},
    utils::calculate_lockup_fee,
};

pub fn transfer(e: &Env, token: &Address, to: &Address, amount: &i128) {
    Client::new(e, token).transfer(&e.current_contract_address(), to, amount);
}

pub fn transfer_from(e: &Env, token: &Address, from: &Address, amount: &i128) {
    Client::new(e, token).transfer_from(
        &e.current_contract_address(),
        from,
        &e.current_contract_address(),
        amount,
    );
}

pub fn take_xlm_fee(e: &Env, start_date: u64, end_date: u64, sender: Address, admin: Address) {
    let xlm = get_xlm(e);
    let monthly_fee = get_monthly_fee(e);
    let fee = calculate_lockup_fee(start_date, end_date, monthly_fee);

    if fee > 0 {
        Client::new(e, &xlm).transfer_from(&e.current_contract_address(), &sender, &admin, &fee);
    }
}
