#![cfg(test)]

use crate::{MyToken, MyTokenClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_token_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let user = Address::generate(&env);
    
    let contract_id = env.register_contract(None, MyToken);
    let client = MyTokenClient::new(&env, &contract_id);

    client.__constructor(&owner, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));

    // Mint
    client.mint(&user, &1000);
    assert_eq!(client.balance(&user), 1000);

    // Sell
    client.sell(&user, &500);
    assert_eq!(client.balance(&user), 500);

    // Transfer
    let recipient = Address::generate(&env);
    client.transfer(&user, &recipient, &100);
    assert_eq!(client.balance(&user), 400);
    assert_eq!(client.balance(&recipient), 100);
}
