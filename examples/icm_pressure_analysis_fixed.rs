use nice_hand_core::game::tournament::*;

/// ICM Pressure Analysis Tool
/// 
/// This example demonstrates:
/// - ICM calculations in various tournament scenarios
/// - Bubble pressure analysis
/// - Pay jump impact on strategy
/// - Risk vs reward assessment in different stack sizes

fn main() {
    println!("=== ICM Pressure Analysis Tool ===\n");

    // Analyze different tournament scenarios
    analyze_bubble_scenario();
    analyze_final_table_scenario();
    analyze_pay_jump_scenarios();
    analyze_chip_ev_vs_dollar_ev();
    analyze_stack_size_impact();
}

fn analyze_bubble_scenario() {
    println!("=== Bubble Scenario Analysis ===");
    println!("Scenario: 4 players remaining, 3 get paid\n");

    let stacks = vec![8000, 6000, 4000, 2000];
    let payouts = vec![5000.0, 3000.0, 2000.0];

    let icm_calculator = ICMCalculator::new(stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
    let icm_values = icm_calculator.calculate_equity();

    println!("Stack distributions and ICM analysis:");
    let total_chips: u32 = stacks.iter().sum();
    let total_payouts: f64 = payouts.iter().sum();

    for (i, (&stack, &icm_value)) in stacks.iter().zip(&icm_values).enumerate() {
        let chip_percentage = (stack as f64 / total_chips as f64) * 100.0;
        let chip_ev = (stack as f64 / total_chips as f64) * total_payouts;
        let icm_pressure = ((icm_value - chip_ev) / chip_ev) * 100.0;
        let bubble_factor = calculate_bubble_factor(stack, &stacks, &payouts);

        println!("Player {} (Stack: {}):", i + 1, stack);
        println!("  Chip %: {:.1}%", chip_percentage);
        println!("  Chip EV: ${:.2}", chip_ev);
        println!("  ICM Value: ${:.2}", icm_value);
        println!("  ICM Pressure: {:.1}%", icm_pressure);
        println!("  Bubble Factor: {:.3}", bubble_factor);
        println!("  Strategy: {}", get_bubble_strategy_advice(stack, &stacks, bubble_factor));
        println!();
    }

    // Analyze specific situations
    analyze_all_in_scenarios(&stacks, &payouts);
}

fn analyze_final_table_scenario() {
    println!("=== Final Table Analysis ===");
    println!("Scenario: 6 players at final table with significant pay jumps\n");

    let stacks = vec![25000, 18000, 12000, 8000, 5000, 2000];
    let payouts = vec![15000.0, 9000.0, 6000.0, 4000.0, 2500.0, 1500.0];

    let icm_calculator = ICMCalculator::new(stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
    let icm_values = icm_calculator.calculate_equity();

    println!("Final table ICM dynamics:");
    let total_chips: u32 = stacks.iter().sum();

    for (i, (&stack, &icm_value)) in stacks.iter().zip(&icm_values).enumerate() {
        let chip_ev = (stack as f64 / total_chips as f64) * payouts.iter().sum::<f64>();
        let icm_pressure = ((icm_value - chip_ev) / chip_ev) * 100.0;
        
        let next_payout_jump = if i < payouts.len() - 1 {
            payouts[i] - payouts[i + 1]
        } else {
            payouts[i]
        };

        println!("Position {} (Stack: {}):", i + 1, stack);
        println!("  ICM Value: ${:.2}", icm_value);
        println!("  ICM Pressure: {:.1}%", icm_pressure);
        println!("  Next Pay Jump: ${:.2}", next_payout_jump);
        println!("  Strategy: {}", get_final_table_strategy(i, stack, &stacks, icm_pressure));
        println!();
    }
}

fn analyze_pay_jump_scenarios() {
    println!("=== Pay Jump Impact Analysis ===");
    
    // Compare scenarios with different payout structures
    let stacks = vec![6000, 4000, 3000, 2000];
    
    println!("Same stacks, different payout structures:\n");
    
    // Flat payout structure
    let flat_payouts = vec![2500.0, 2500.0, 2500.0, 2500.0];
    analyze_payout_structure("Flat Structure", &stacks, &flat_payouts);
    
    // Winner-take-all
    let winner_takes_all = vec![10000.0];
    analyze_payout_structure("Winner Take All", &stacks, &winner_takes_all);
    
    // Standard tournament structure
    let standard_payouts = vec![5000.0, 3000.0, 2000.0];
    analyze_payout_structure("Standard (3 paid)", &stacks, &standard_payouts);
    
    // Top-heavy structure
    let top_heavy = vec![7000.0, 2000.0, 1000.0];
    analyze_payout_structure("Top Heavy", &stacks, &top_heavy);
}

fn analyze_payout_structure(name: &str, stacks: &[u32], payouts: &[f64]) {
    let icm_calculator = ICMCalculator::new(stacks.to_vec(), payouts.iter().map(|&x| x as u64).collect());
    let icm_values = icm_calculator.calculate_equity();
    
    println!("{}: ", name);
    let total_chips: u32 = stacks.iter().sum();
    let total_payouts: f64 = payouts.iter().sum();
    
    for (i, (&stack, &icm_value)) in stacks.iter().zip(&icm_values).enumerate() {
        let chip_ev = (stack as f64 / total_chips as f64) * total_payouts;
        let icm_premium = icm_value - chip_ev;
        println!("  Player {}: ICM Premium: ${:.2}", i + 1, icm_premium);
    }
    println!();
}

fn analyze_chip_ev_vs_dollar_ev() {
    println!("=== Chip EV vs Dollar EV Analysis ===");
    
    let base_stacks = vec![5000, 4000, 3000, 2000, 1000];
    let payouts = vec![7000.0, 4000.0, 2500.0, 1500.0, 1000.0];
    
    println!("Analyzing various all-in scenarios:\n");
    
    // Scenario 1: Short stack doubles up
    println!("Scenario 1: Short stack (1000) doubles through medium stack (3000)");
    let scenario1_win = vec![5000, 4000, 1000, 2000, 2000];
    let scenario1_lose = vec![5000, 4000, 4000, 2000, 0];
    
    analyze_scenario_comparison("Short stack wins", &base_stacks, &scenario1_win, &payouts, 4);
    analyze_scenario_comparison("Short stack loses", &base_stacks, &scenario1_lose, &payouts, 4);
    
    println!();
    
    // Scenario 2: Big stack vs medium stack
    println!("Scenario 2: Big stack (5000) vs medium stack (4000)");
    let scenario2_big_wins = vec![9000, 0, 3000, 2000, 1000];
    let scenario2_medium_wins = vec![1000, 8000, 3000, 2000, 1000];
    
    analyze_scenario_comparison("Big stack wins", &base_stacks, &scenario2_big_wins, &payouts, 0);
    analyze_scenario_comparison("Medium stack wins", &base_stacks, &scenario2_medium_wins, &payouts, 1);
}

fn analyze_scenario_comparison(scenario_name: &str, before: &[u32], after: &[u32], payouts: &[f64], acting_player: usize) {
    let icm_calculator_before = ICMCalculator::new(before.to_vec(), payouts.iter().map(|&x| x as u64).collect());
    let before_icm = icm_calculator_before.calculate_equity();
    
    let icm_calculator_after = ICMCalculator::new(after.to_vec(), payouts.iter().map(|&x| x as u64).collect());
    let after_icm = icm_calculator_after.calculate_equity();
    
    let ev_change = after_icm[acting_player] - before_icm[acting_player];
    
    println!("  {}: EV change for Player {}: ${:.2}", 
             scenario_name, acting_player + 1, ev_change);
}

fn analyze_stack_size_impact() {
    println!("=== Stack Size Impact on ICM Pressure ===");
    
    let payouts = vec![5000.0, 3000.0, 2000.0];
    
    println!("How ICM pressure changes with stack size (3 players, bubble situation):\n");
    
    // Test different stack distributions
    let test_scenarios = vec![
        ("Short Stack", vec![1000, 4000, 5000]),
        ("Medium Stack", vec![3000, 3000, 4000]),
        ("Big Stack", vec![5000, 3000, 2000]),
        ("Chip Leader", vec![7000, 2000, 1000]),
    ];
    
    for (scenario_name, stacks) in test_scenarios {
        let icm_calculator = ICMCalculator::new(stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
        let icm_values = icm_calculator.calculate_equity();
        
        let total_chips: u32 = stacks.iter().sum();
        let total_payouts: f64 = payouts.iter().sum();
        
        println!("{} scenario:", scenario_name);
        for (i, (&stack, &icm_value)) in stacks.iter().zip(&icm_values).enumerate() {
            let chip_ev = (stack as f64 / total_chips as f64) * total_payouts;
            let icm_pressure = ((icm_value - chip_ev) / chip_ev) * 100.0;
            let risk_tolerance = calculate_risk_tolerance(stack, &stacks, icm_pressure);
            
            println!("  Player {} ({} chips): ICM Pressure: {:.1}%, Risk Tolerance: {}", 
                     i + 1, stack, icm_pressure, risk_tolerance);
        }
        println!();
    }
}

fn analyze_all_in_scenarios(stacks: &[u32], payouts: &[f64]) {
    println!("All-in scenario analysis:");
    
    let icm_calculator = ICMCalculator::new(stacks.to_vec(), payouts.iter().map(|&x| x as u64).collect());
    let current_icm = icm_calculator.calculate_equity();
    
    // Short stack (player 4) goes all-in
    let short_stack_pos = 3; // 0-indexed, player 4
    let short_stack = stacks[short_stack_pos];
    
    println!("If short stack (Player 4, {} chips) goes all-in:", short_stack);
    
    // Analyze call profitability for each other player
    for i in 0..stacks.len() {
        if i == short_stack_pos { continue; }
        
        let caller_stack = stacks[i];
        if caller_stack <= short_stack { continue; } // Can't cover the all-in
        
        // Calculate scenarios: call and win vs call and lose
        let mut win_stacks = stacks.to_vec();
        let mut lose_stacks = stacks.to_vec();
        
        win_stacks[i] += short_stack;
        win_stacks[short_stack_pos] = 0;
        
        lose_stacks[i] -= short_stack;
        lose_stacks[short_stack_pos] = short_stack * 2;
        
        let win_calculator = ICMCalculator::new(win_stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
        let win_icm = win_calculator.calculate_equity();
        
        let lose_calculator = ICMCalculator::new(lose_stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
        let lose_icm = lose_calculator.calculate_equity();
        
        // Calculate minimum winning percentage needed
        let current_value = current_icm[i];
        let win_value = win_icm[i];
        let lose_value = lose_icm[i];
        
        let breakeven_percentage = (current_value - lose_value) / (win_value - lose_value);
        
        println!("  Player {} call analysis:", i + 1);
        println!("    Current ICM: ${:.2}", current_value);
        println!("    If win: ${:.2} (gain: ${:.2})", win_value, win_value - current_value);
        println!("    If lose: ${:.2} (loss: ${:.2})", lose_value, current_value - lose_value);
        println!("    Breakeven %: {:.1}%", breakeven_percentage * 100.0);
        println!("    Recommendation: {}", 
                 if breakeven_percentage > 0.6 { "Tight call - need strong hand" }
                 else if breakeven_percentage > 0.4 { "Standard call range" }
                 else { "Wide call range acceptable" });
        println!();
    }
}

fn calculate_bubble_factor(stack: u32, all_stacks: &[u32], payouts: &[f64]) -> f64 {
    let players_remaining = all_stacks.len();
    let paid_positions = payouts.len();
    
    if players_remaining <= paid_positions {
        return 1.0; // In the money
    }
    
    let excess_players = players_remaining - paid_positions;
    let stack_rank = all_stacks.iter()
        .filter(|&&s| s > stack)
        .count() + 1;
    
    // Factor considers both bubble distance and relative stack size
    let base_factor = 1.0 - (excess_players as f64 / 6.0).min(0.9);
    let stack_adjustment = (stack_rank as f64 / players_remaining as f64).min(1.0);
    
    (base_factor * stack_adjustment).max(0.1)
}

fn get_bubble_strategy_advice(stack: u32, all_stacks: &[u32], bubble_factor: f64) -> &'static str {
    let total_chips: u32 = all_stacks.iter().sum();
    let stack_percentage = (stack as f64 / total_chips as f64) * 100.0;
    
    match (stack_percentage, bubble_factor) {
        (p, _f) if p > 40.0 => "Aggressive - pressure smaller stacks",
        (p, _f) if p > 30.0 => "Selective aggression - target weak spots",
        (p, _f) if p > 20.0 => "Cautious - avoid big confrontations",
        (p, _f) if p > 10.0 => "Survival mode - very tight, wait for premium hands",
        _ => "Desperate - must find spots to double up"
    }
}

fn get_final_table_strategy(position: usize, _stack: u32, _all_stacks: &[u32], icm_pressure: f64) -> &'static str {
    match (position, icm_pressure) {
        (0..=1, p) if p > 10.0 => "Conservative chip leader - protect lead",
        (0..=1, _) => "Aggressive chip leader - build massive lead", 
        (2..=3, p) if p > 5.0 => "Balanced - mix of aggression and caution",
        (2..=3, _) => "Standard play - take calculated risks",
        (4..=5, p) if p < -5.0 => "Aggressive short stack - need to accumulate",
        _ => "Tight - survive to next pay jump"
    }
}

fn calculate_risk_tolerance(_stack: u32, _all_stacks: &[u32], icm_pressure: f64) -> &'static str {
    match icm_pressure {
        p if p > 10.0 => "Low - protect stack advantage",
        p if p > 5.0 => "Medium-Low - selective spots only",
        p if p > -5.0 => "Medium - standard risk assessment",
        p if p > -10.0 => "High - need to accumulate chips",
        _ => "Very High - desperate situations require big risks"
    }
}
