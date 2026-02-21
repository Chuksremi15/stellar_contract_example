#![cfg(test)]
use super::*;
use soroban_sdk::Env;

#[test]
fn test_increment() {
    // 1. Setup the environment
    let env = Env::default();
    
    // 2. Register the contract
    let contract_id = env.register(IncrementContract, ());
    
    // 3. Create the client
    let client = IncrementContractClient::new(&env, &contract_id);

    // 4. Test the logic
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}