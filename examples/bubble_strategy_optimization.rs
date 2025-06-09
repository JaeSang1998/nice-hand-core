use nice_hand_core::game::tournament::*;
use nice_hand_core::ICMCalculator;

/// Bubble Strategy Optimization Tool
/// 
/// This example demonstrates:
/// - Optimal bubble play strategies
/// - Stack-specific adjustments
/// - Position and opponent-aware bubble play
/// - Mathematical analysis of bubble decisions

fn main() {
    println!("=== Bubble Strategy Optimization ===\n");

    // Analyze different bubble scenarios
    analyze_standard_bubble();
    analyze_super_bubble();
    analyze_stone_bubble();
    optimize_bubble_strategies();
    analyze_position_on_bubble();
}

fn analyze_standard_bubble() {
    println!("=== Standard Bubble Analysis ===");
    println!("Scenario: 4 players, 3 get paid, fairly even stacks\n");

    let stacks = vec![6000, 5000, 4500, 4500];
    let payouts = vec![50000, 30000, 20000]; // Converted to u64
    let blind_level = BlindLevel { level: 1, small_blind: 200, big_blind: 400, ante: 50 };

    let bubble_strategy = BubbleStrategy::new(stacks.len() as u32, payouts.len() as u32);
    
    println!("Stack distribution:");
    for (i, &stack) in stacks.iter().enumerate() {
        let bb_count = stack / blind_level.big_blind;
        println!("  Player {}: {} chips ({} BB)", i + 1, stack, bb_count);
    }
    println!("Blinds: {}/{} + {} ante\n", blind_level.small_blind, blind_level.big_blind, blind_level.ante);

    // Analyze each player's optimal strategy
    for (i, &stack) in stacks.iter().enumerate() {
        analyze_player_bubble_strategy(i, stack, &stacks, &payouts, &blind_level, &bubble_strategy);
    }
}

fn analyze_super_bubble() {
    println!("=== Super Bubble Analysis ===");
    println!("Scenario: 5 players, 4 get paid, one very short stack\n");

    let stacks = vec![8000, 6000, 4000, 3000, 500];
    let payouts = vec![35000, 25000, 18000, 12000]; // Converted to u64
    let blind_level = BlindLevel { level: 2, small_blind: 300, big_blind: 600, ante: 75 };

    let _bubble_strategy = BubbleStrategy::new(stacks.len() as u32, payouts.len() as u32);
    
    println!("Super bubble dynamics (very short stack creates protective bubble):");
    
    for (i, &stack) in stacks.iter().enumerate() {
        let bb_count = stack / blind_level.big_blind;
        let protection_level = calculate_protection_level(i, &stacks);
        let strategy = get_super_bubble_strategy(i, stack, &stacks, &blind_level);
        
        println!("Player {} ({} BB, protection: {:.2}): {}", 
                 i + 1, bb_count, protection_level, strategy);
    }
}

fn analyze_stone_bubble() {
    println!("=== Stone Bubble Analysis ===");
    println!("Scenario: 6 players, 5 get paid, two very short stacks\n");

    let stacks = vec![10000, 8000, 6000, 4000, 800, 600];
    let payouts = vec![35000, 25000, 18000, 12000, 10000]; // Converted to u64
    let blind_level = BlindLevel { level: 3, small_blind: 200, big_blind: 400, ante: 50 };

    println!("Stone bubble scenario - multiple short stacks create extreme protection:");
    
    // Calculate stone bubble factor
    let stone_factor = calculate_stone_bubble_factor(&stacks, &payouts);
    println!("Stone bubble factor: {:.3} (higher = more protection for medium stacks)\n", stone_factor);
    
    for (i, &stack) in stacks.iter().enumerate() {
        let bb_count = stack / blind_level.big_blind;
        let protection_level = calculate_protection_level(i, &stacks);
        let strategy = get_stone_bubble_strategy(i, stack, &stacks, protection_level);
        
        println!("Player {} ({} BB, protection: {:.2}): {}", 
                 i + 1, bb_count, protection_level, strategy);
    }
}

fn optimize_bubble_strategies() {
    println!("=== Bubble Strategy Optimization ===");
    println!("Optimizing strategies for each stack size:\n");

    // Test various stack configurations
    let test_configurations = vec![
        ("Short Stack (8 BB)", vec![1200, 6000, 5000, 4000], vec![50000, 30000, 20000]),
        ("Medium Stack (15 BB)", vec![6000, 6000, 5000, 3000], vec![50000, 30000, 20000]),
        ("Big Stack (25 BB)", vec![10000, 5000, 3000, 2000], vec![50000, 30000, 20000]),
        ("Chip Leader (35 BB)", vec![14000, 3000, 2000, 1000], vec![50000, 30000, 20000]),
    ];

    for (scenario_name, stacks, payouts) in test_configurations {
        println!("{} scenario:", scenario_name);
        analyze_optimal_strategy_for_hero(&stacks, &payouts, 0);
        println!();
    }
}

fn analyze_position_on_bubble() {
    println!("=== Position Analysis on Bubble ===");
    println!("How position affects bubble strategy:\n");

    let stacks = vec![5000, 4500, 4000, 3500];
    let _payouts = vec![50000, 30000, 20000]; // Converted to u64
    let blind_level = BlindLevel { level: 4, small_blind: 150, big_blind: 300, ante: 25 };

    for position in 0..stacks.len() {
        println!("Position {}: {}", position + 1, get_position_strategy(position, &stacks, &blind_level));
    }
}

fn analyze_player_bubble_strategy(
    player_idx: usize,
    stack: u32,
    all_stacks: &[u32],
    payouts: &[u64],
    _blind_level: &BlindLevel,
    _bubble_strategy: &BubbleStrategy
) {
    // Calculate ICM pressure
    let icm_calculator = ICMCalculator::new(all_stacks.to_vec(), payouts.to_vec());
    let icm_values = icm_calculator.calculate_equity();
    
    let total_chips: u32 = all_stacks.iter().sum();
    let chip_percentage = (stack as f64 / total_chips as f64) * 100.0;
    let icm_percentage = icm_values[player_idx] * 100.0;
    let icm_pressure = icm_percentage - chip_percentage;
    
    let strategy = match icm_pressure {
        p if p > 5.0 => "Conservative - preserve chip lead",
        p if p > 0.0 => "Balanced - standard ranges",
        p if p > -5.0 => "Slightly aggressive - accumulate chips",
        _ => "Aggressive - desperate for chips"
    };
    
    println!("Player {} analysis: {:.1}% chips, {:.1}% ICM equity, Strategy: {}",
             player_idx + 1, chip_percentage, icm_percentage, strategy);
}

fn get_super_bubble_strategy(_player_idx: usize, stack: u32, all_stacks: &[u32], _blind_level: &BlindLevel) -> &'static str {
    let total_chips: u32 = all_stacks.iter().sum();
    let stack_percentage = (stack as f64 / total_chips as f64) * 100.0;
    let shortest_stack = *all_stacks.iter().min().unwrap();
    
    if stack == shortest_stack {
        "Desperate - shove any decent hand"
    } else if stack_percentage > 35.0 {
        "Exploitative - attack weak spots but avoid big confrontations"
    } else if stack_percentage > 20.0 {
        "Protected - play tight, let short stack bust"
    } else {
        "Cautious aggression - balance survival with chip accumulation"
    }
}

fn get_stone_bubble_strategy(_player_idx: usize, stack: u32, all_stacks: &[u32], protection_level: f64) -> &'static str {
    let total_chips: u32 = all_stacks.iter().sum();
    let stack_percentage = (stack as f64 / total_chips as f64) * 100.0;
    
    match (stack_percentage, protection_level) {
        (p, _pr) if p < 5.0 => "All-in mode - any live cards",
        (p, pr) if p < 15.0 && pr < 0.3 => "Desperate - wide shoving range", 
        (p, pr) if p < 15.0 && pr > 0.7 => "Optimistic survival - fold to pressure",
        (p, pr) if p > 30.0 && pr > 0.8 => "Hyper-aggressive - abuse protection",
        (p, pr) if p > 20.0 && pr > 0.5 => "Selective aggression - pick good spots",
        _ => "Balanced - standard bubble adjustments"
    }
}

fn calculate_protection_level(player_idx: usize, stacks: &[u32]) -> f64 {
    let player_stack = stacks[player_idx];
    let shorter_stacks = stacks.iter().filter(|&&s| s < player_stack).count();
    let total_players = stacks.len();
    
    (shorter_stacks as f64) / (total_players as f64 - 1.0)
}

fn calculate_stone_bubble_factor(stacks: &[u32], _payouts: &[u64]) -> f64 {
    let total_chips: u32 = stacks.iter().sum();
    let mut sorted_stacks = stacks.to_vec();
    sorted_stacks.sort();
    
    // Calculate how many very short stacks exist
    let avg_stack = total_chips / stacks.len() as u32;
    let very_short_threshold = avg_stack / 4;
    let very_short_count = sorted_stacks.iter().filter(|&&s| s < very_short_threshold).count();
    
    // Stone factor increases with more short stacks
    very_short_count as f64 / stacks.len() as f64
}

fn analyze_optimal_strategy_for_hero(stacks: &[u32], payouts: &[u64], hero_position: usize) {
    let icm_calculator = ICMCalculator::new(stacks.to_vec(), payouts.to_vec());
    let current_icm = icm_calculator.calculate_equity();
    
    println!("  Hero (position {}): {:.1}% ICM equity", 
             hero_position + 1, current_icm[hero_position] * 100.0);
    
    // Simulate different scenarios
    for change_percentage in vec![-20, -10, 10, 20] {
        let mut new_stacks = stacks.to_vec();
        let stack_change = (stacks[hero_position] as i32 * change_percentage / 100) as i32;
        new_stacks[hero_position] = (new_stacks[hero_position] as i32 + stack_change).max(0) as u32;
        
        // Redistribute chips to maintain total
        let total_change = -stack_change;
        let others_change = total_change / ((stacks.len() - 1) as i32);
        for i in 0..stacks.len() {
            if i != hero_position {
                new_stacks[i] = (new_stacks[i] as i32 + others_change).max(0) as u32;
            }
        }
        
        let new_icm = ICMCalculator::new(new_stacks, payouts.to_vec());
        let new_equities = new_icm.calculate_equity();
        let equity_change = (new_equities[hero_position] - current_icm[hero_position]) * 100.0;
        
        println!("    Stack change {:+}%: {:+.1}% equity change", change_percentage, equity_change);
    }
}

fn get_position_strategy(position: usize, _stacks: &[u32], _blind_level: &BlindLevel) -> &'static str {
    match position {
        0 => "Small Blind - very tight, avoid marginal spots",
        1 => "Big Blind - defend narrowly, pot odds secondary to ICM",
        2 => "Early Position - premium hands only, set up for later streets",
        _ => "Late Position - exploit tight play, but avoid big pots without premium hands"
    }
}
