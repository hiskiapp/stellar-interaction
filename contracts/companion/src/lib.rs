#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map, Vec};

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
    fn initialize(env: &Env, user: &Address, companion_public_id: &Symbol) -> Map<Symbol, i32> {
        let mut companion = Map::new(env);
        companion.set(symbol_short!("points"), 0);
        companion.set(symbol_short!("feed"), 0);
        companion.set(symbol_short!("pat"), 0);
        companion.set(symbol_short!("pickup"), 0);
        companion.set(symbol_short!("sleep"), 0);
        companion.set(symbol_short!("toy"), 0);
        
        let mut user_companions: Vec<Symbol> = env.storage().instance().get(user).unwrap_or_else(|| Vec::new(env));
        user_companions.push_back(companion_public_id.clone());
        env.storage().instance().set(user, &user_companions);
        
        env.storage().instance().set(&(user.clone(), companion_public_id.clone()), &companion);
        companion
    }

    pub fn interact(env: Env, user: Address, companion_public_id: Symbol, action: Symbol) -> bool {
        let mut companion: Map<Symbol, i32> = env.storage().instance().get(&(user.clone(), companion_public_id.clone()))
            .unwrap_or_else(|| Self::initialize(&env, &user, &companion_public_id));

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
        
        env.storage().instance().set(&(user, companion_public_id), &companion);
        true
    }

    // Get the total points for all companions of a user
    pub fn get_points(env: Env, user: Address) -> i32 {
        let user_companions: Vec<Symbol> = env.storage().instance().get(&user).unwrap_or_else(|| Vec::new(&env));
        let mut total_points = 0;

        for companion_public_id in user_companions.iter() {
            if let Some(companion) = env.storage().instance().get(&(user.clone(), companion_public_id)) {
                let companion: Map<Symbol, i32> = companion;
                total_points += companion.get(symbol_short!("points")).unwrap_or(0);
            }
        }

        total_points
    }

    // TODO: Implement point exchange for gift card NFT
    

    // TODO: Implement gift card NFT burning for code retrieval
}

mod test;
