#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

pub trait FluxityTrait {
    fn init(e: Env, admin: Address);
    fn stream(e: Env);
    fn create_stream(e: Env);
    fn cancel_stream(e: Env);
    fn withdraw_stream(e: Env);
    fn top_up_stream(e: Env);
}

#[contract]
pub struct Fluxity;

#[contractimpl]
impl FluxityTrait for Fluxity {
    fn init(_e: Env, _admin: Address) {}
    fn stream(_e: Env) {}
    fn create_stream(_e: Env) {}
    fn cancel_stream(_e: Env) {}
    fn withdraw_stream(_e: Env) {}
    fn top_up_stream(_e: Env) {}
}

// #[cfg(test)]
// mod tests;
