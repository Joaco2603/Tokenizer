use soroban_sdk::{contract, contractimpl, Address, Env, MuxedAddress, String};
use stellar_tokens::fungible::{Base, FungibleToken};
use stellar_tokens::fungible::burnable::FungibleBurnable;
use stellar_access::ownable::{self, Ownable};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{only_owner, when_not_paused, Upgradeable};

use crate::core::token::TokenManager;
use crate::storage::types::DataKey;

// SEP-0046 contract metadata embedded in the WASM binary.
soroban_sdk::contractmeta!(
    key = "Description",
    val = "MyToken — SEP-0041 fungible token with owner-gated minting, pausability, and upgradeability"
);
soroban_sdk::contractmeta!(key = "Version", val = "0.1.0");

#[derive(Upgradeable)]
#[contract]
pub struct MyToken;

#[contractimpl]
impl MyToken {
    /// One-time constructor (Protocol 22+ / CAP-0058).
    /// `decimals` is now a parameter instead of being hardcoded to 7 — use 7
    /// for XLM-compatible wallet display.
    pub fn __constructor(e: &Env, owner: Address, name: String, symbol: String, decimals: u32) {
        TokenManager::initialize(e, owner, name, symbol, decimals);
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

    /// Post-upgrade state migration. `new_version` must be strictly greater
    /// than the previously recorded version (starts at 0). Callable by owner
    /// only — ensures migrations are monotonic and idempotent.
    #[only_owner]
    pub fn migrate(e: &Env, new_version: u32) {
        let current: u32 = e
            .storage()
            .instance()
            .get(&DataKey::AppVersion)
            .unwrap_or(0);
        assert!(new_version > current);
        e.storage().instance().set(&DataKey::AppVersion, &new_version);
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

/// SEP-0041 requires `FungibleBurnable` for full compliance.
/// Both `burn` and `burn_from` are blocked while the contract is paused.
#[contractimpl(contracttrait)]
impl FungibleBurnable for MyToken {
    #[when_not_paused]
    fn burn(e: &Env, from: Address, amount: i128) {
        Base::burn(e, &from, amount);
    }

    #[when_not_paused]
    fn burn_from(e: &Env, spender: Address, from: Address, amount: i128) {
        Base::burn_from(e, &spender, &from, amount);
    }
}

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
