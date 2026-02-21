#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::token;

#[test]
fn test_esusu_rotation_and_payout() {
    // 1. Setup the Environment
    let env = Env::default();
    env.mock_all_auths(); // Bypass manual signatures for the test

    // 2. Setup the Actors (Your EkeOyibo DAO Members)
    let admin = Address::generate(&env);
    let member_1 = Address::generate(&env); // The first winner
    let member_2 = Address::generate(&env);
    let member_3 = Address::generate(&env);

    // 3. Setup the Mock Token (e.g., USDC or NGNC)
    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&env, &token_address);

    // Give everyone 10,000 tokens to start
    token_client.mint(&member_1, &10_000);
    token_client.mint(&member_2, &10_000);
    token_client.mint(&member_3, &10_000);

    // 4. Register the Esusu Contract
    let contract_id = env.register(EsusuContract, ());
    let esusu_client = EsusuContractClient::new(&env, &contract_id);

    // 5. Initialize the Pool
    // Target is 1,000 tokens per person, per round
    esusu_client.init(&admin, &token_address, &1000);

    // 6. Members Join the Pool
    esusu_client.join(&member_1);
    esusu_client.join(&member_2);
    esusu_client.join(&member_3);

    // --- ROUND 1 BEGINS --- //

    // 7. First two members contribute
    esusu_client.contribute(&member_1);
    esusu_client.contribute(&member_2);

    // Verify: The contract holds 2,000, and no payout has happened yet
    assert_eq!(token_client.balance(&contract_id), 2000);
    assert_eq!(token_client.balance(&member_1), 9000); // 10k - 1k

    // 8. The Final Contribution!
    // This should trigger the automatic payout to member_1
    esusu_client.contribute(&member_3);

    // 9. Verify the Magic
    // Member 1 started with 10k, paid 1k (down to 9k), and won the 3k pot = 12k total!
    assert_eq!(token_client.balance(&member_1), 12_000);
    
    // The contract should be empty again
    assert_eq!(token_client.balance(&contract_id), 0);

    // 10. Check the Rotation Status
    // It should now be Round 1 (the second index), and Member 2 is next up
    let (next_round, next_winner) = esusu_client.status();
    assert_eq!(next_round, 1);
    assert_eq!(next_winner, member_2);
}