#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec};

// Keys for persistent storage
const MEMBERS: Symbol = symbol_short!("MEMBERS");
const ROUND: Symbol = symbol_short!("ROUND");
const TARGET: Symbol = symbol_short!("TARGET"); 
const TOKEN: Symbol = symbol_short!("TOKEN");
const ADMIN: Symbol = symbol_short!("ADMIN");

#[contract]
pub struct EsusuContract;

#[contractimpl]
impl EsusuContract {
    /// 1. Initialize the Esusu pool.
    /// @param admin: The address that oversees the pool.
    /// @param token: The address of the token to be used (e.g., USDC or NGNC).
    /// @param amount: The fixed contribution required from each member per round.
    pub fn init(env: Env, admin: Address, token: Address, amount: i128) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token);
        env.storage().instance().set(&TARGET, &amount);
        env.storage().instance().set(&ROUND, &0u32);
        env.storage().instance().set(&MEMBERS, &vec![&env]);
    }

    /// 2. Join the pool.
    /// Members must join before the rounds start to keep the math stable.
    pub fn join(env: Env, member: Address) {
        member.require_auth();
        let mut members: Vec<Address> = env.storage().instance().get(&MEMBERS).unwrap_or(vec![&env]);
        
        if members.contains(&member) { 
            panic!("Address is already a member"); 
        }
        
        members.push_back(member);
        env.storage().instance().set(&MEMBERS, &members);
    }

    /// 3. Contribute and Payout.
    /// When the final member pays for the round, the contract automatically triggers the payout.
    pub fn contribute(env: Env, contributor: Address) {
        contributor.require_auth();

        let token: Address = env.storage().instance().get(&TOKEN).unwrap();
        let amount: i128 = env.storage().instance().get(&TARGET).unwrap();
        let mut members: Vec<Address> = env.storage().instance().get(&MEMBERS).unwrap();
        let mut current_round: u32 = env.storage().instance().get(&ROUND).unwrap();

        // Transfer funds from contributor to the contract escrow
        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&contributor, &env.current_contract_address(), &amount);

        // Logic: Is the pot ready for payout?
        // In a true Esusu, the payout happens once everyone has contributed for that round.
        let total_members = members.len() as i128;
        let expected_pot = amount * total_members;
        let current_balance = client.balance(&env.current_contract_address());

        if current_balance >= expected_pot {
            // Determine the winner based on the rotation round
            let winner = members.get(current_round).expect("Winner index out of bounds");
            
            // Payout the total balance of the contract to the winner
            client.transfer(&env.current_contract_address(), &winner, &current_balance);

            // Increment the round for the next person in line
            // The modulo (%) operator ensures it wraps back to the first member after the last one
            current_round = (current_round + 1) % (members.len() as u32);
            env.storage().instance().set(&ROUND, &current_round);
        }
    }

    /// View helper to check the current round and next winner
    pub fn status(env: Env) -> (u32, Address) {
        let members: Vec<Address> = env.storage().instance().get(&MEMBERS).unwrap();
        let round: u32 = env.storage().instance().get(&ROUND).unwrap();
        (round, members.get(round).unwrap())
    }
}