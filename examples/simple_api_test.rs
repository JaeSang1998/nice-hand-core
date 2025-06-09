// Test the new stateless API functionality

use nice_hand_core::api::web_api::WebGameState;

fn main() {
    println!("Testing New Stateless Poker AI API");
    
    let web_state = WebGameState {
        hole_cards: [0, 13],
        board: vec![],
        street: 0,
        pot: 150,
        stacks: vec![1000, 1000],
        alive_players: vec![0, 1],
        street_investments: vec![50, 100],
        to_call: 100,
        player_to_act: 0,
        hero_position: 0,
        betting_history: vec![],
    };

    println!("Created web state with hole cards: {:?}", web_state.hole_cards);
    println!("Pot size: {}", web_state.pot);
    
    // Test validation
    match nice_hand_core::validate_game_state(&web_state) {
        Ok(_) => println!("✅ Game state validation passed!"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    println!("✅ Basic API test completed!");
}
