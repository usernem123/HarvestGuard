#![no_std]

use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct HarvestGuardContract;

#[contractimpl]
impl HarvestGuardContract {
    pub fn hello(env: Env, name: String) -> String {
        String::from_str(&env, &format!("Hello, {}", name))
    }
}