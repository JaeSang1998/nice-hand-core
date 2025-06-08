use nice_hand_core::{Trainer, holdem};

fn main() {
    println!("Simple CFR Test - Testing Hold'em");
    
    // Test Hold'em CFR (Testing infinite recursion fixes) 
    println!("\n=== Hold'em CFR Test (Testing infinite recursion fixes) ===");
    let mut holdem_trainer = Trainer::<holdem::State>::new();
    
    // Create a simple 2-player Hold'em state using the correct structure
    let initial_state = holdem::State {
        hole: [[0, 1], [2, 3], [4, 5], [6, 7], [8, 9], [10, 11]],
        board: vec![],
        to_act: 0,
        street: 0,
        pot: 100,
        stack: [1000; 6],
        alive: [true, true, false, false, false, false], // Only 2 players
        invested: [15, 30, 0, 0, 0, 0], // Blinds posted
        to_call: 30,
        actions_taken: 0,
    };
    
    println!("Training Hold'em with 50 iterations (testing for infinite recursion)...");
    let start_time = std::time::Instant::now();
    
    holdem_trainer.run(vec![initial_state], 50);
    
    let elapsed = start_time.elapsed();
    println!("Hold'em training completed in {:?}! Nodes created: {}", elapsed, holdem_trainer.nodes.len());
    
    if holdem_trainer.nodes.len() > 0 {
        println!("✅ Hold'em CFR training successful - no infinite recursion detected!");
        
        // Show some example strategies (first few nodes)
        for (info_key, node) in holdem_trainer.nodes.iter().take(3) {
            let avg_strategy = node.average();
            println!("InfoKey {}: Strategy {:?}", info_key, avg_strategy);
        }
    } else {
        println!("❌ No nodes created - there may still be an issue");
    }
}
