#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

#[contract]
pub struct HarvestGuard;

#[derive(Clone)]
struct Delivery {
    farmer: Address,
    weight_kg: i128,
    crop_type: String,
    timestamp: u64,
    token_amount: i128,
}

#[contractimpl]
impl HarvestGuard {
    pub fn record_delivery(
        env: Env,
        operator: Address,
        farmer: Address,
        weight_kg: i128,
        crop_type: String,
        token_address: Address,
        price_per_kg: i128,
    ) -> Vec<u8> {
        operator.require_auth();
        
        let timestamp = env.ledger().timestamp();
        let token_amount = weight_kg * price_per_kg;
        
        let mut delivery_id = farmer.clone().into();
        delivery_id.extend_from_slice(&timestamp.to_le_bytes());
        let delivery_id_vec = Vec::from_array(&env, delivery_id);
        
        let delivery = Delivery {
            farmer: farmer.clone(),
            weight_kg,
            crop_type,
            timestamp,
            token_amount,
        };
        
        env.storage().persistent().set(&delivery_id_vec, &delivery);
        
        let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_address);
        token_client.mint(&farmer, &token_amount);
        
        delivery_id_vec
    }
    
    pub fn get_delivery(env: Env, delivery_id: Vec<u8>) -> (Address, i128, String, u64, i128) {
        let delivery: Delivery = env.storage().persistent().get(&delivery_id).unwrap();
        (delivery.farmer, delivery.weight_kg, delivery.crop_type, delivery.timestamp, delivery.token_amount)
    }
}

#[cfg(test)]
mod test;