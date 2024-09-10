#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map};

// Define points for each interaction type
const FEED_POINTS: i32 = 2;
const PAT_POINTS: i32 = 1;
const PICKUP_POINTS: i32 = 1;
const SLEEP_POINTS: i32 = 1;
const TOY_POINTS: i32 = 1;

#[contract]
pub struct CompanionContract;

#[contractimpl]
impl CompanionContract {
    // Initialize the companion for a user
    pub fn initialize(env: Env, user: Address) -> Map<Symbol, i32> {
        let mut companion = Map::new(&env);
        companion.set(symbol_short!("points"), 0);
        companion.set(symbol_short!("feed"), 0);
        companion.set(symbol_short!("pat"), 0);
        companion.set(symbol_short!("pickup"), 0);
        companion.set(symbol_short!("sleep"), 0);
        companion.set(symbol_short!("toy"), 0);
        
        env.storage().instance().set(&user, &companion);
        companion
    }

    // Perform an interaction with the companion
    pub fn interact(env: Env, user: Address, action: Symbol) -> bool {
        let mut companion: Map<Symbol, i32> = env.storage().instance().get(&user).unwrap_or_else(|| Self::initialize(env.clone(), user.clone()));

        let points_to_add = match action {
            a if a == symbol_short!("feed") => FEED_POINTS,
            a if a == symbol_short!("pat") => PAT_POINTS,
            a if a == symbol_short!("pickup") => PICKUP_POINTS,
            a if a == symbol_short!("sleep") => SLEEP_POINTS,
            a if a == symbol_short!("toy") => TOY_POINTS,
            _ => return false,
        };
        
        let points = companion.get(symbol_short!("points")).unwrap_or(0) + points_to_add;
        companion.set(symbol_short!("points"), points);
        
        env.storage().instance().set(&user, &companion);
        true
    }

    // Get the current points for a user
    pub fn get_points(env: Env, user: Address) -> i32 {
        let companion: Map<Symbol, i32> = env.storage().instance().get(&user).unwrap_or_else(|| Self::initialize(env.clone(), user.clone()));
        companion.get(symbol_short!("points")).unwrap_or(0)
    }

    // TODO: Implement point exchange for gift card NFT
    

    // TODO: Implement gift card NFT burning for code retrieval
}

mod test;
