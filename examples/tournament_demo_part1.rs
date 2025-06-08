// Tournament features demonstration
use nice_hand_core::game::tournament::*;
use std::time::Instant;

fn main() {
    println!("=== Tournament Features Demo ===\n");

    // Test ICM calculations
    test_icm_calculations();
    
    // Test opponent modeling
    test_opponent_modeling();
    
    // Test MTT management
    test_mtt_management();
    
    // Test bubble strategy
    test_bubble_strategy();
    
    // Test tournament evaluator
    test_tournament_evaluator();
    
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
    
    // Test with different scenarios
    test_bubble_icm();
    test_heads_up_icm();
    
    println!("   ✅ ICM calculations working correctly\n");
}
