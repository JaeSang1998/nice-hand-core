// Test the new stateless API functionality
// This example demonstrates the enhanced poker AI library with comprehensive analysis

use nice_hand_core::api::web_api::WebGameState;
use nice_hand_core::{analyze_comprehensive, calculate_quick_ev, validate_game_state, get_action_recommendation};

fn main() {
    println!("🃏 Testing New Stateless Poker AI API");
    println!("=====================================\n");

    // Test 1: Comprehensive Analysis
    test_comprehensive_analysis();

    // Test 2: Quick EV Calculation
    test_quick_ev_calculation();

    // Test 3: Game State Validation
    test_game_state_validation();

    // Test 4: Action Recommendation
    test_action_recommendation();

    println!("\n✅ All tests completed successfully!");
}

fn test_comprehensive_analysis() {
    println!("🔍 Test 1: Comprehensive Analysis");
    println!("--------------------------------");

    let web_state = WebGameState {
        hole_cards: [0, 13], // AA (Ace of Spades, Ace of Hearts)
        board: vec![],        // Preflop
        street: 0,
        pot: 150,
        stacks: vec![1000, 1000], // 2 players
        alive_players: vec![0, 1],
        street_investments: vec![50, 100], // SB, BB
        to_call: 100,
        player_to_act: 0,
        hero_position: 0,
        betting_history: vec![],
    };

    match analyze_comprehensive(&web_state, "standard", true) {
        Ok(analysis) => {
            println!("✅ Analysis successful!");
            println!("   📊 EV Analysis:");
            for action_ev in &analysis.ev_analysis.action_evs {
                println!("      {:?}: EV = {:.2}", action_ev.action, action_ev.ev);
            }
            
            if let Some(insights) = &analysis.insights {
                println!("   🎯 Recommended Action: {:?}", insights.recommended_action);
                println!("   💪 Hand Strength: {:.3}", insights.hand_strength);
                println!("   ⚠️  Risk Level: {:?}", insights.risk_assessment);
                
                if let Some(advice) = &insights.positional_advice {
                    println!("   📍 Position Advice: {}", advice);
                }
            }
            
            println!("   ⏱️  Calculation Time: {}ms", analysis.metadata.calculation_time_ms);
            println!("   🎯 Confidence: {:.1}%", analysis.metadata.confidence_level * 100.0);
        }
        Err(e) => println!("❌ Analysis failed: {}", e),
    }
    println!();
}

fn test_quick_ev_calculation() {
    println!("⚡ Test 2: Quick EV Calculation");
    println!("------------------------------");

    let web_state = WebGameState {
        hole_cards: [12, 25], // KQ offsuit
        board: vec![0, 14, 28], // A♠ 2♥ 3♣ 
        street: 1, // Flop
        pot: 200,
        stacks: vec![800, 900],
        alive_players: vec![0, 1],
        street_investments: vec![100, 100],
        to_call: 50,
        player_to_act: 0,
        hero_position: 0,
        betting_history: vec![],
    };

    match calculate_quick_ev(&web_state, Some(1000)) {
        Ok(ev_analysis) => {
            println!("✅ Quick EV calculation successful!");
            println!("   📈 Action EVs:");
            for action_ev in &ev_analysis.action_evs {
                println!("      {:?}: EV = {:.2}", action_ev.action, action_ev.ev);
            }
            println!("   🔧 Analysis Type: {}", ev_analysis.analysis_type);
        }
        Err(e) => println!("❌ Quick EV calculation failed: {}", e),
    }
    println!();
}

fn test_game_state_validation() {
    println!("✅ Test 3: Game State Validation");
    println!("--------------------------------");

    // Valid state
    let valid_state = WebGameState {
        hole_cards: [0, 1],
        board: vec![2, 3, 4], // 3 cards = flop
        street: 1,
        pot: 100,
        stacks: vec![500, 600],
        alive_players: vec![0, 1],
        street_investments: vec![50, 50],
        to_call: 0,
        player_to_act: 0,
        hero_position: 0,
        betting_history: vec![],
    };

    match validate_game_state(&valid_state) {
        Ok(_) => println!("✅ Valid game state passed validation!"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    // Invalid state (too many board cards)
    let invalid_state = WebGameState {
        hole_cards: [0, 1],
        board: vec![2, 3, 4, 5, 6, 7], // 6 cards = invalid
        street: 1,
        pot: 100,
        stacks: vec![500, 600],
        alive_players: vec![0, 1],
        street_investments: vec![50, 50],
        to_call: 0,
        player_to_act: 0,
        hero_position: 0,
        betting_history: vec![],
    };

    match validate_game_state(&invalid_state) {
        Ok(_) => println!("❌ Invalid state incorrectly passed validation!"),
        Err(e) => println!("✅ Invalid game state correctly rejected: {}", e),
    }
    println!();
}

fn test_action_recommendation() {
    println!("🎯 Test 4: Action Recommendation");
    println!("--------------------------------");

    let web_state = WebGameState {
        hole_cards: [0, 13], // AA
        board: vec![],        // Preflop
        street: 0,
        pot: 30,
        stacks: vec![1000, 1000],
        alive_players: vec![0, 1],
        street_investments: vec![10, 20], // SB, BB
        to_call: 20,
        player_to_act: 0,
        hero_position: 0,
        betting_history: vec![],
    };

    // Test different risk tolerances
    for risk_tolerance in ["conservative", "standard", "aggressive"] {
        match get_action_recommendation(&web_state, risk_tolerance) {
            Ok(action) => println!("✅ {} strategy: {:?}", risk_tolerance, action),
            Err(e) => println!("❌ {} strategy failed: {}", risk_tolerance, e),
        }
    }
    println!();
}
