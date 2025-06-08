// Advanced Heuristic Strategy Demonstration
use nice_hand_core::web_api_simple::{QuickPokerAPI, WebGameState};

fn main() {
    println!("🃏 Advanced Poker Heuristic Strategy Demo");
    println!("=========================================");
    
    let api = QuickPokerAPI::new();
    
    // Test scenario 1: Premium preflop hand
    println!("\n📋 Scenario 1: Premium Preflop Hand (AA)");
    println!("-{}", "-".repeat(49));
    
    let premium_state = WebGameState {
        hole_cards: [0, 13], // AA (Ace of Spades, Ace of Hearts)
        board: vec![],
        street: 0,
        pot: 150,
        to_call: 100,
        my_stack: 1000,
        opponent_stack: 1000,
    };
    
    let response = api.get_optimal_strategy(premium_state);
    println!("🎯 Recommended Action: {}", response.recommended_action);
    println!("💪 Hand Strength: {:.1}%", response.hand_strength * 100.0);
    println!("📊 Expected Value: {:.1} chips", response.expected_value);
    println!("🧠 Reasoning: {}", response.reasoning);
    
    // Test scenario 2: Marginal calling hand
    println!("\n📋 Scenario 2: Marginal Calling Hand (KQ offsuit)");
    println!("-{}", "-".repeat(49));
    
    let marginal_state = WebGameState {
        hole_cards: [11, 23], // KQ offsuit
        board: vec![],
        street: 0,
        pot: 200,
        to_call: 150,
        my_stack: 800,
        opponent_stack: 800,
    };
    
    let response2 = api.get_optimal_strategy(marginal_state);
    println!("🎯 Recommended Action: {}", response2.recommended_action);
    println!("💪 Hand Strength: {:.1}%", response2.hand_strength * 100.0);
    println!("📊 Expected Value: {:.1} chips", response2.expected_value);
    println!("🧠 Reasoning: {}", response2.reasoning);
    
    // Test scenario 3: Strong postflop hand (top pair)
    println!("\n📋 Scenario 3: Strong Postflop Hand (Top Pair)");
    println!("-{}", "-".repeat(49));
    
    let postflop_state = WebGameState {
        hole_cards: [0, 26], // A♠ K♠
        board: vec![1, 21, 34], // A♥ 9♠ J♥ - Top pair with great kicker
        street: 1,
        pot: 300,
        to_call: 0, // Checking to us
        my_stack: 700,
        opponent_stack: 700,
    };
    
    let response3 = api.get_optimal_strategy(postflop_state);
    println!("🎯 Recommended Action: {}", response3.recommended_action);
    println!("💪 Hand Strength: {:.1}%", response3.hand_strength * 100.0);
    println!("📊 Expected Value: {:.1} chips", response3.expected_value);
    println!("🧠 Reasoning: {}", response3.reasoning);
    
    // Performance test
    println!("\n📊 Performance Analysis");
    println!("-{}", "-".repeat(49));
    
    let start = std::time::Instant::now();
    let test_states: Vec<WebGameState> = (0u32..1000u32).map(|i| {
        WebGameState {
            hole_cards: [(i % 52) as u8, ((i + 13) % 52) as u8],
            board: if i % 3 == 0 { vec![] } else { vec![(i % 52) as u8, ((i + 1) % 52) as u8, ((i + 2) % 52) as u8] },
            street: if i % 3 == 0 { 0 } else { 1 },
            pot: 100 + (i % 500),
            to_call: i % 200,
            my_stack: 1000,
            opponent_stack: 1000,
        }
    }).collect();
    
    let _responses = api.get_strategies_batch(test_states);
    let duration = start.elapsed();
    
    println!("✅ Processed 1,000 decisions in {:?}", duration);
    println!("⚡ Average: {:.2}μs per decision", duration.as_micros() as f64 / 1000.0);
    
    println!("\n🎯 Heuristic Enhancement Complete!");
    println!("   ✓ Sophisticated hand evaluation");
    println!("   ✓ Advanced betting strategies");
    println!("   ✓ Context-aware decision making");
    println!("   ✓ Production-ready performance");
}
