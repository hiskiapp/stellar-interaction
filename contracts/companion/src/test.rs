#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, Address, symbol_short};

#[test]
fn test_companion_interactions() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CompanionContract);
    let client = CompanionContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Test initialization
    let initial_data = client.initialize(&user);
    assert_eq!(initial_data.get(symbol_short!("points")), Some(0));
    assert_eq!(initial_data.get(symbol_short!("feed")), Some(0));
    assert_eq!(initial_data.get(symbol_short!("pat")), Some(0));
    assert_eq!(initial_data.get(symbol_short!("pickup")), Some(0));
    assert_eq!(initial_data.get(symbol_short!("sleep")), Some(0));
    assert_eq!(initial_data.get(symbol_short!("toy")), Some(0));

    // Test interactions
    assert!(client.interact(&user, &symbol_short!("feed")));
    assert!(client.interact(&user, &symbol_short!("pat")));
    assert!(client.interact(&user, &symbol_short!("pickup")));
    assert!(client.interact(&user, &symbol_short!("sleep")));
    assert!(client.interact(&user, &symbol_short!("toy")));

    // Test get_points (2 + 1 + 1 + 1 + 1 = 6 points)
    assert_eq!(client.get_points(&user), 6);

    // Test multiple interactions of the same type
    assert!(client.interact(&user, &symbol_short!("feed")));
    assert!(client.interact(&user, &symbol_short!("feed")));

    // Test get_points (6 + 2 + 2 = 10 points)
    assert_eq!(client.get_points(&user), 10);

    // Test invalid interaction
    assert!(!client.interact(&user, &symbol_short!("invalid")));

    // Test points remain unchanged after invalid interaction
    assert_eq!(client.get_points(&user), 10);

    // Test multiple users
    let user2 = Address::generate(&env);
    
    assert!(client.interact(&user2, &symbol_short!("feed")));
    assert!(client.interact(&user2, &symbol_short!("pat")));
    
    assert_eq!(client.get_points(&user2), 3);
    assert_eq!(client.get_points(&user), 10); // Original user's points should remain unchanged

    // Test maximum interactions
    for _ in 0..100 {
        client.interact(&user, &symbol_short!("feed"));
    }
    
    let max_points = 10 + (100 * FEED_POINTS);
    assert_eq!(client.get_points(&user), max_points);
}
