#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_escrow_flow() {
    let env = Env::default();
    env.mock_all_auths(); // This mimics the digital signature process for testing

    // 1. Create test addresses for our actors
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    // 2. Setup a "Dummy Token" to act as our currency
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_address = token_contract.address();
    let token_client = token::Client::new(&env, &token_address);
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);

    // 3. Give the sender some starting money (1000 units)
    token_admin_client.mint(&sender, &1000);

    // 4. Register our Escrow Contract
    let contract_id = env.register(TokenEscrow, ());
    let escrow_client = TokenEscrowClient::new(&env, &contract_id);

    // 5. EXECUTE DEPOSIT: Sender puts 500 into escrow for recipient
    escrow_client.deposit(&sender, &token_address, &500, &recipient);

    // Verify: Sender should have 500 left, Contract should have 500
    assert_eq!(token_client.balance(&sender), 500);
    assert_eq!(token_client.balance(&contract_id), 500);

    // 6. EXECUTE CLAIM: Recipient takes the 500
    escrow_client.claim(&recipient, &token_address);

    // Final Verification
    assert_eq!(token_client.balance(&recipient), 500);
    assert_eq!(token_client.balance(&contract_id), 0);
}