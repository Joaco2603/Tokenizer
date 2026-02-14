use soroban_sdk::{Address, Env, MuxedAddress, String};
use stellar_tokens::fungible::Base;
use stellar_access::ownable;
use crate::events::handler::TokenEvents;

pub struct TokenManager;

impl TokenManager {
    pub fn initialize(e: &Env, owner: Address, name: String, symbol: String, decimals: u32) {
        Base::set_metadata(e, decimals, name, symbol);
        ownable::set_owner(e, &owner);
    }

    pub fn mint(e: &Env, to: &Address, amount: i128) {
        ownable::enforce_owner_auth(e);
        Base::mint(e, to, amount);
        TokenEvents::mint(e, ownable::owner(e), to.clone(), amount);
    }

    pub fn transfer(e: &Env, from: &Address, to: &MuxedAddress, amount: i128) {
        Base::transfer(e, from, to, amount);
        // Base emits events usually, but we can add ours if needed. 
        // Note: stellar-tokens Base::transfer emits Transfer event. 
        // TokenEvents::transfer(e, from.clone(), to.clone(), amount); // Redundant if Base does it
    }

    pub fn transfer_from(e: &Env, spender: &Address, from: &Address, to: &Address, amount: i128) {
        Base::transfer_from(e, spender, from, to, amount);
    }

    pub fn burn(e: &Env, from: &Address, amount: i128) {
        Base::burn(e, from, amount);
        TokenEvents::burn(e, from.clone(), amount);
    }

    pub fn sell(e: &Env, seller: &Address, amount: i128) {
        seller.require_auth();
        // Sell logic: separate from burn, but effectively burns tokens in exchange for "nothing" in this example
        // or could implement a logic to send XLM/other token back if this was a liquidity pool.
        // For now, it's just burning with a specific "Sell" event semantic.
        Base::burn(e, seller, amount);
        TokenEvents::sell(e, seller.clone(), amount);
    }
}
