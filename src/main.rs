use nice_hand_core::{Trainer, holdem};

fn main() {
    println!("Nice Hand Core - Preference CFR Implementation for Texas Hold'em");
    
    // Texas Hold'em CFR Test
    println!("\n=== Texas Hold'em CFR Test ===");
    let mut holdem_trainer = Trainer::<holdem::State>::new();
    
    // Create a Hold'em state for testing
    let initial_state = holdem::State {
        hole: [[0, 1], [2, 3], [4, 5], [6, 7], [8, 9], [10, 11]],
        board: vec![],
        to_act: 0,
        street: 0,
        pot: 100,
        stack: [1000; 6],
        alive: [true, true, false, false, false, false], // 2 players
        invested: [15, 30, 0, 0, 0, 0], // Blinds posted
        to_call: 30,
        actions_taken: 0,
    };
    
    println!("Training Texas Hold'em with {} iterations...", 100);
    let start_time = std::time::Instant::now();
    
    holdem_trainer.run(vec![initial_state], 100);
    
    let elapsed = start_time.elapsed();
    println!("Hold'em training completed in {:?}! Nodes: {}", elapsed, holdem_trainer.nodes.len());
    
    // Show some strategy results
    for (info_key, node) in holdem_trainer.nodes.iter().take(3) {
        let avg_strategy = node.average();
        println!("InfoKey {}: Strategy {:?}", info_key, avg_strategy);
    }
    
    println!("\n=== CFR Implementation Successfully Applied for Texas Hold'em! ===");
}
