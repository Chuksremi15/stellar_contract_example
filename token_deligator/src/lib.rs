#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct TokenEscrow;

#[contractimpl]
impl TokenEscrow {
    // Deposit tokens from the sender into this contract for a specific recipient
    pub fn deposit(env: Env, sender: Address, token: Address, amount: i128, recipient: Address) {
        // RULE 1: Verify the sender actually wants to move this money
        sender.require_auth();

        // RULE 2: Use the standard Token Client to move the funds
        let client = token::Client::new(&env, &token);
        
        // Transfer from sender to this contract address
        client.transfer(&sender, &env.current_contract_address(), &amount);

        // RULE 3: Save the recipient and amount so we know who to pay later
        env.storage().instance().set(&recipient, &amount);
    }

    pub fn claim(env: Env, recipient: Address, token: Address) {
        // RULE 4: Only the recipient can trigger the payout
        recipient.require_auth();

        // Retrieve the saved amount
        let amount: i128 = env.storage().instance().get(&recipient).unwrap_or(0);
        if amount <= 0 { panic!("No funds to claim"); }

        // Send funds from contract to recipient
        let client = token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &recipient, &amount);

        // Clear the storage
        env.storage().instance().remove(&recipient);
    }
}

#[cfg(test)]
mod test;



