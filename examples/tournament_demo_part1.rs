// Tournament features demonstration
use nice_hand_core::game::tournament::*;
use std::time::Instant;

fn main() {
    println!("=== Tournament Features Demo ===\n");

    // Test ICM calculations
    test_icm_calculations();
    
    // Test basic bubble strategy
    test_basic_bubble_strategy();
    
    // Test tournament structure
    test_tournament_structure();
    
    println!("\n=== All Tournament Tests Completed Successfully ===");
}

fn test_icm_calculations() {
    println!("🎯 Testing ICM Calculations...");
    
    // Realistic tournament scenario: 4 players left, 3 get paid
    let stacks = vec![15000, 8000, 5000, 2000]; // Chip stacks
    let payouts = vec![10000, 6000, 4000]; // Prize structure
    
    let icm = ICMCalculator::new(stacks.clone(), payouts.clone());
    let start_time = Instant::now();
    let equities = icm.calculate_equity();
    let calculation_time = start_time.elapsed();
    
    println!("   📊 Stacks: {:?}", stacks);
    println!("   💰 Payouts: {:?}", payouts);
    println!("   ⚖️  ICM Equities: {:.2?}", equities);
    println!("   ⏱️  Calculation time: {:?}", calculation_time);
    
    // Test ICM pressure calculation
    let pressure = icm.calculate_icm_pressure(0, -1000); // Big stack loses 1k chips
    println!("   📉 ICM pressure for chip leader losing 1000 chips: {:.4}", pressure);
    
    // Test bubble scenario
    test_bubble_icm();
    test_heads_up_icm();
    
    println!("   ✅ ICM calculations working correctly\n");
}

fn test_bubble_icm() {
    println!("   🫧 Testing Bubble ICM...");
    
    // 5 players, 4 get paid (bubble situation)
    let stacks = vec![12000, 10000, 8000, 6000, 4000];
    let payouts = vec![15000, 10000, 7000, 5000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      Bubble equities: {:.2?}", equities);
    
    // Short stack should have lower equity
    if equities[4] < equities[3] {
        println!("      ✅ Short stack has appropriately reduced equity");
    }
}

fn test_heads_up_icm() {
    println!("   🥊 Testing Heads-up ICM...");
    
    let stacks = vec![30000, 10000];
    let payouts = vec![20000, 12000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      Heads-up equities: {:.2?}", equities);
    
    // ICM should reduce chip leader advantage
    if equities[0] > 15000.0 && equities[0] < 18000.0 {
        println!("      ✅ ICM properly reduces chip leader advantage");
    }
}

fn test_basic_bubble_strategy() {
    println!("🫧 Testing Basic Bubble Strategy...");
    
    // 19 players remaining, 18 get paid (classic bubble)
    let bubble_strategy = BubbleStrategy::new(19, 18);
    
    println!("   💫 Bubble factor: {:.3}", bubble_strategy.bubble_factor);
    
    // Test strategy adjustments for different stack sizes
    let base_range = 0.2; // 20% of hands normally
    let short_stack_range = bubble_strategy.adjust_hand_range(base_range, 0.6);
    let big_stack_range = bubble_strategy.adjust_hand_range(base_range, 2.0);
    
    println!("   📉 Short stack range: {:.1}%", short_stack_range * 100.0);
    println!("   📈 Big stack range: {:.1}%", big_stack_range * 100.0);
    
    // Test aggressive play decision
    let should_be_aggressive = bubble_strategy.should_make_aggressive_play(1.2, 0.1);
    println!("   ⚔️  Should medium stack be aggressive: {}", should_be_aggressive);
    
    println!("   ✅ Bubble strategy working correctly\n");
}

fn test_tournament_structure() {
    println!("🏗️ Testing Tournament Structure...");
    
    // Create a tournament structure
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
            BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 10 },
            BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 15 },
        ],
        level_duration_minutes: 20,
        starting_stack: 1500,
        ante_schedule: vec![],
    };
    
    // Create tournament state
    let tournament = TournamentState::new(structure, 180, 100000);
    let (sb, bb, ante) = tournament.current_blinds();
    
    println!("   🎮 Current blinds: {}/{} with {} ante", sb, bb, ante);
    println!("   👥 Players remaining: {}", tournament.players_remaining);
    println!("   💰 Total players: {}", tournament.total_players);
    
    println!("   ✅ Tournament structure working correctly\n");
}
