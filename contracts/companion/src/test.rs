#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, Address, symbol_short, Symbol};

#[test]
fn test_companion_interactions() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CompanionContract);
    let client = CompanionContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let companion1 = Symbol::new(&env, "1234");
    let companion2 = Symbol::new(&env, "5678");

    // Test interactions with companion1
    assert!(client.interact(&user, &companion1, &symbol_short!("feed")));
    assert!(client.interact(&user, &companion1, &symbol_short!("pat")));
    assert!(client.interact(&user, &companion1, &symbol_short!("pickup")));
    assert!(client.interact(&user, &companion1, &symbol_short!("sleep")));
    assert!(client.interact(&user, &companion1, &symbol_short!("toy")));

    // Test get_points for companion1 (2 + 1 + 1 + 1 + 1 = 6 points)
    assert_eq!(client.get_points(&user), 6);

    // Test interactions with companion2
    assert!(client.interact(&user, &companion2, &symbol_short!("feed")));
    assert!(client.interact(&user, &companion2, &symbol_short!("pat")));

    // Test get_points for both companions (6 + 3 = 9 points)
    assert_eq!(client.get_points(&user), 9);

    // Test multiple interactions of the same type with companion1
    assert!(client.interact(&user, &companion1, &symbol_short!("feed")));
    assert!(client.interact(&user, &companion1, &symbol_short!("feed")));

    // Test get_points (9 + 2 + 2 = 13 points)
    assert_eq!(client.get_points(&user), 13);

    // Test invalid interaction
    assert!(!client.interact(&user, &companion1, &symbol_short!("invalid")));

    // Test points remain unchanged after invalid interaction
    assert_eq!(client.get_points(&user), 13);

    // Test multiple users
    let user2 = Address::generate(&env);
    let companion3 = Symbol::new(&env, "9999");
    
    assert!(client.interact(&user2, &companion3, &symbol_short!("feed")));
    assert!(client.interact(&user2, &companion3, &symbol_short!("pat")));
    
    assert_eq!(client.get_points(&user2), 3);
    assert_eq!(client.get_points(&user), 13); // Original user's points should remain unchanged

    // Test maximum interactions
    for _ in 0..100 {
        client.interact(&user, &companion1, &symbol_short!("feed"));
    }
    
    let max_points = 13 + (100 * FEED_POINTS);
    assert_eq!(client.get_points(&user), max_points);
}
