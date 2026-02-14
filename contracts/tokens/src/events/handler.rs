use soroban_sdk::{symbol_short, Address, Env};

pub struct TokenEvents;

impl TokenEvents {
    pub fn mint(e: &Env, admin: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("mint"), admin, to);
        e.events().publish(topics, amount);
    }

    pub fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("transfer"), from, to);
        e.events().publish(topics, amount);
    }

    pub fn burn(e: &Env, from: Address, amount: i128) {
        let topics = (symbol_short!("burn"), from);
        e.events().publish(topics, amount);
    }

    pub fn sell(e: &Env, seller: Address, amount: i128) {
        let topics = (symbol_short!("sell"), seller);
        e.events().publish(topics, amount);
    }
}
