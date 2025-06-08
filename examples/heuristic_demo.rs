// Advanced Heuristic Strategy Demonstration
// Shows the sophisticated poker logic in action with detailed analysis

use nice_hand_core::web_api_simple::{QuickPokerAPI, WebGameState};

fn main() {
    println!("🃏 Advanced Poker Heuristic Strategy Demo");
    println!("=========================================");
    
    let api = QuickPokerAPI::new();
    
    // Test scenario 1: Premium preflop hand
    println!("\n📋 Scenario 1: Premium Preflop Hand (AA)");
    println!("{}", "-".repeat(50));
    
    let premium_state = WebGameState {
        hole_cards: [0, 13], // AA (Ace of Spades, Ace of Hearts)
        board: vec![],
        street: 0,
        pot: 150,
        to_call: 100,
        my_stack: 1000,
        opponent_stack: 1000,
    };
    
    demonstrate_strategy(&api, premium_state, "Pocket Aces preflop facing a raise");
    
    // Test scenario 2: Marginal calling hand
    println!("\n📋 Scenario 2: Marginal Calling Hand (KQ offsuit)");
    println!("{}", "-".repeat(50));
    
    let marginal_state = WebGameState {
        hole_cards: [11, 23], // KQ offsuit
        board: vec![],
        street: 0,
        pot: 200,
        to_call: 150,
        my_stack: 800,
        opponent_stack: 800,
    };
    
    demonstrate_strategy(&api, marginal_state, "KQ offsuit facing large preflop raise");
    
    // Test scenario 3: Strong postflop hand (top pair)
    println!("\n📋 Scenario 3: Strong Postflop Hand (Top Pair)");
    println!("{}", "-".repeat(50));
    
    let postflop_state = WebGameState {
        hole_cards: [0, 26], // A♠ K♠
        board: vec![1, 21, 34], // A♥ 9♠ J♥ - Top pair with great kicker
        street: 1,
        pot: 300,
        to_call: 0, // Checking to us
        my_stack: 700,
        opponent_stack: 700,
    };
    
    demonstrate_strategy(&api, postflop_state, "Top pair Aces with King kicker on flop");
    
    // Test scenario 4: Flush draw
    println!("\n📋 Scenario 4: Flush Draw (Semi-bluff spot)");
    println!("{}", "-".repeat(50));
    
    let flush_draw_state = WebGameState {
        hole_cards: [26, 39], // K♠ Q♠
        board: vec![7, 20, 33], // 8♠ 8♥ 9♠ - Flush draw + straight draw
        street: 1,
        pot: 400,
        to_call: 200,
        my_stack: 600,
        opponent_stack: 600,
    };
    
    demonstrate_strategy(&api, flush_draw_state, "Flush draw facing bet on coordinated board");
    
    // Test scenario 5: Weak hand in bluff spot
    println!("\n📋 Scenario 5: Weak Hand Bluff Spot");
    println!("{}", "-".repeat(50));
    
    let bluff_state = WebGameState {
        hole_cards: [4, 17], // 5♠ 6♥
        board: vec![48, 49, 50], // K♠ Q♠ J♠ - Complete whiff
        street: 1,
        pot: 250,
        to_call: 0,
        my_stack: 750,
        opponent_stack: 750,
    };
    
    demonstrate_strategy(&api, bluff_state, "Complete air on high coordinated board");
    
    // Test scenario 6: Short stack all-in situation
    println!("\n📋 Scenario 6: Short Stack All-in Decision");
    println!("{}", "-".repeat(50));
    
    let short_stack_state = WebGameState {
        hole_cards: [32, 45], // 7♠ 7♥ 
        board: vec![],
        street: 0,
        pot: 400,
        to_call: 180, // Almost half our stack
        my_stack: 400,
        opponent_stack: 800,
    };
    
    demonstrate_strategy(&api, short_stack_state, "Pocket 7s short stack facing large raise");
    
    // Performance test
    println!("\n📊 Performance Analysis");
    println!("{}", "-".repeat(50));
    
    let start = std::time::Instant::now();
    let test_states: Vec<WebGameState> = (0u32..1000u32).map(|i| {
        WebGameState {
            hole_cards: [(i % 52) as u8, ((i + 13) % 52) as u8],
            board: if i % 3 == 0 { vec![] } else { vec![(i % 52) as u8, ((i + 1) % 52) as u8, ((i + 2) % 52) as u8] },
            street: if i % 3 == 0 { 0 } else { 1 },
            pot: 100 + (i % 500) as u32,
            to_call: (i % 200) as u32,
            my_stack: 1000,
            opponent_stack: 1000,
        }
    }).collect();
    
    let responses = api.get_strategies_batch(test_states);
    let duration = start.elapsed();
    
    println!("✅ Processed 1,000 decisions in {:?}", duration);
    println!("⚡ Average: {:.2}μs per decision", duration.as_micros() as f64 / 1000.0);
    
    // Action distribution analysis
    let mut action_counts = std::collections::HashMap::new();
    for response in &responses {
        *action_counts.entry(response.recommended_action.clone()).or_insert(0) += 1;
    }
    
    println!("\n📈 Action Distribution (1,000 random scenarios):");
    for (action, count) in action_counts {
        println!("  {} {}: {}% ({} decisions)", 
                 get_action_emoji(&action), action, 
                 (count as f64 / 10.0), count);
    }
    
    println!("\n🎯 Heuristic Enhancement Complete!");
    println!("   ✓ Sophisticated hand evaluation");
    println!("   ✓ Advanced betting strategies");
    println!("   ✓ Context-aware decision making");
    println!("   ✓ Production-ready performance");
}

fn demonstrate_strategy(api: &QuickPokerAPI, state: WebGameState, description: &str) {
    println!("📝 Situation: {}", description);
    
    let response = api.get_optimal_strategy(state.clone());
    
    println!("🎯 Recommended Action: {} {}", 
             get_action_emoji(&response.recommended_action), 
             response.recommended_action);
    println!("💪 Hand Strength: {:.1}%", response.hand_strength * 100.0);
    println!("📊 Expected Value: {:.1} chips", response.expected_value);
    println!("🎲 Confidence: {:.1}%", response.confidence * 100.0);
    
    if state.to_call > 0 {
        println!("💰 Pot Odds: {:.1}%", response.pot_odds * 100.0);
    }
    
    println!("🧠 Reasoning: {}", response.reasoning);
    
    println!("📈 Strategy Distribution:");
    let mut sorted_strategy: Vec<_> = response.strategy.iter().collect();
    sorted_strategy.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    
    for (action, probability) in sorted_strategy {
        if *probability > 0.01 { // Only show actions with >1% probability
            println!("   {} {}: {:.1}%", 
                     get_action_emoji(action), action, probability * 100.0);
        }
    }
}

fn get_action_emoji(action: &str) -> &'static str {
    match action {
        "fold" => "🛑",
        "check" => "✋",
        "call" => "📞",
        "bet_small" => "💰",
        "bet_large" => "💎",
        "raise" => "🚀",
        _ => "❓",
    }
}
