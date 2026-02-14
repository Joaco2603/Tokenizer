use soroban_sdk::{contract, contractimpl, Address, Env, MuxedAddress, String};
use stellar_tokens::fungible::{Base, FungibleToken};
use stellar_access::ownable::{self, Ownable};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{only_owner, when_not_paused, Upgradeable};

use crate::core::token::TokenManager;

#[derive(Upgradeable)]
#[contract]
pub struct MyToken;

#[contractimpl]
impl MyToken {
    pub fn __constructor(e: &Env, owner: Address, name: String, symbol: String) {
        // Hardcoded decimals for simplicity or pass as arg
        TokenManager::initialize(e, owner, name, symbol, 7);
    }

    #[only_owner]
    #[when_not_paused]
    pub fn mint(e: &Env, to: Address, amount: i128) {
        TokenManager::mint(e, &to, amount);
    }

    #[when_not_paused]
    pub fn sell(e: &Env, seller: Address, amount: i128) {
        TokenManager::sell(e, &seller, amount);
    }
}

#[contractimpl(contracttrait)]
impl FungibleToken for MyToken {
    type ContractType = Base;

    #[when_not_paused]
    fn transfer(e: &Env, from: Address, to: MuxedAddress, amount: i128) {
        TokenManager::transfer(e, &from, &to, amount);
    }

    #[when_not_paused]
    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        TokenManager::transfer_from(e, &spender, &from, &to, amount);
    }
}

// Implement other traits like Burnable directly or via Manager if preferred, 
// keeping it simple here.

#[contractimpl]
impl Pausable for MyToken {
    fn paused(e: &Env) -> bool {
        pausable::paused(e)
    }

    #[only_owner]
    fn pause(e: &Env, _caller: Address) {
        pausable::pause(e);
    }

    #[only_owner]
    fn unpause(e: &Env, _caller: Address) {
        pausable::unpause(e);
    }
}

#[contractimpl(contracttrait)]
impl Ownable for MyToken {}

impl UpgradeableInternal for MyToken {
    fn _require_auth(e: &Env, _operator: &Address) {
        ownable::enforce_owner_auth(e);
    }
}
