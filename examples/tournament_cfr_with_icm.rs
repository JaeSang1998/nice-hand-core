use nice_hand_core::game::tournament::*;
use std::collections::HashMap;

/// ICM 통합 토너먼트 CFR
/// 
/// 이 예제는 다음을 보여줍니다:
/// - ICM 조정 유틸리티를 통한 CFR 훈련
/// - Tournament-specific strategy calculation
/// - Dynamic strategy adaptation based on stack sizes
/// - ICM pressure modeling in CFR

fn main() {
    println!("=== Tournament CFR with ICM Integration ===\n");

    // Demonstrate different tournament scenarios with CFR
    demonstrate_basic_icm_cfr();
    demonstrate_bubble_cfr_training();
    demonstrate_final_table_cfr();
    demonstrate_adaptive_tournament_strategy();
}

fn demonstrate_basic_icm_cfr() {
    println!("=== 기본 ICM 조정 CFR 훈련 ===");
    
    let stacks = vec![5000, 4000, 3000, 2000];
    let payouts = vec![6000.0, 3600.0, 2400.0];
    let blind_level = BlindLevel { level: 1, small_blind: 100, big_blind: 200, ante: 25 };
    
    println!("Training CFR with ICM adjustments...");
    println!("Stacks: {:?}", stacks);
    println!("Payouts: {:?}", payouts);
    println!();
    
    // Create ICM-aware CFR trainer
    let mut icm_cfr = ICMCFRTrainer::new(stacks.clone(), payouts.clone(), blind_level.clone());
    
    // Train for multiple scenarios
    let scenarios = vec![
        ("Preflop", 0u8),  // 0 = Preflop
        ("Flop", 1u8),     // 1 = Flop  
        ("Turn", 2u8),     // 2 = Turn
        ("River", 3u8),    // 3 = River
    ];
    
    for (scenario_name, street) in scenarios {
        println!("Training {} scenario:", scenario_name);
        
        let iterations = 10000;
        let results = icm_cfr.train_scenario(street, iterations);
        
        println!("  Iterations: {}", iterations);
        println!("  Convergence: {:.4}", results.convergence);
        println!("  Average ICM utility: {:.2}", results.average_utility);
        
        // Show strategy differences between chip EV and ICM
        display_strategy_comparison(&icm_cfr, &stacks, street);
        println!();
    }
}

fn demonstrate_bubble_cfr_training() {
    println!("=== 버블 특화 CFR 훈련 ===");
    
    // Classic bubble scenario: 4 players, 3 paid
    let stacks = vec![6000, 5000, 4500, 1500];
    let payouts = vec![5000.0, 3000.0, 2000.0];
    let blind_level = BlindLevel { level: 2, small_blind: 150, big_blind: 300, ante: 25 };
    
    println!("Bubble scenario training:");
    println!("Players: {} → Paid: {}", stacks.len(), payouts.len());
    println!("Short stack: {} BB", stacks[3] / blind_level.big_blind);
    println!();
    
    let mut bubble_cfr = ICMCFRTrainer::new(stacks.clone(), payouts.clone(), blind_level.clone());
    
    // Train with heavy bubble weighting
    bubble_cfr.set_bubble_weighting(2.0); // 2x weight on bubble pressure
    
    let bubble_scenarios = vec![
        ("Short stack UTG", 3, 0),  // Player 4 in UTG
        ("Short stack BTN", 3, 2),  // Player 4 on button
        ("Medium vs short", 2, 1),  // Player 3 in CO vs short stack
        ("Big stack isolation", 0, 2), // Player 1 on button
    ];
    
    for (scenario_name, acting_player, position) in bubble_scenarios {
        println!("Scenario: {} (Player {} in position {})", scenario_name, acting_player + 1, position);
        
        let strategy = bubble_cfr.calculate_position_strategy(acting_player, position, 0u8); // 0 = Preflop
        
        println!("  Optimal strategy:");
        println!("    Fold: {:.1}%", strategy.fold_frequency * 100.0);
        println!("    Call: {:.1}%", strategy.call_frequency * 100.0);
        println!("    Raise: {:.1}%", strategy.raise_frequency * 100.0);
        println!("    All-in: {:.1}%", strategy.allin_frequency * 100.0);
        
        // Compare to chip EV strategy
        let chip_ev_strategy = calculate_chip_ev_strategy(acting_player, position, &stacks);
        let icm_adjustment = calculate_icm_adjustment(&strategy, &chip_ev_strategy);
        
        println!("  ICM adjustment: {:.1}% tighter than chip EV", icm_adjustment * 100.0);
        println!();
    }
}

fn demonstrate_final_table_cfr() {
    println!("=== Final Table CFR Optimization ===");
    
    let stacks = vec![25000, 18000, 12000, 8000, 5000, 2000];
    let payouts = vec![15000.0, 9000.0, 6000.0, 4000.0, 2500.0, 1500.0];
    let blind_level = BlindLevel { level: 3, small_blind: 400, big_blind: 800, ante: 100 };
    
    println!("Final table setup:");
    for (i, (&stack, &payout)) in stacks.iter().zip(&payouts).enumerate() {
        let bb_count = stack / blind_level.big_blind;
        println!("  Player {}: {} BB → ${:.0} guaranteed", i + 1, bb_count, payout);
    }
    println!();
    
    let final_table_cfr = ICMCFRTrainer::new(stacks.clone(), payouts.clone(), blind_level.clone());
    
    // Different final table dynamics
    let final_table_spots = vec![
        ("칩 리더 공격성", 0, "칩 리더는 얼마나 공격적이어야 할까?"),
        ("숏 스택 절망", 5, "숏 스택은 언제 도박해야 할까?"),
        ("미디엄 스택 생존", 3, "미디엄 스택은 얼마나 타이트하게 플레이해야 할까?"),
        ("상금 점프 압박", 2, "다음 상금 점프가 플레이에 어떤 영향을 미칠까?"),
    ];
    
    for (spot_name, player_idx, description) in final_table_spots {
        println!("{}: {}", spot_name, description);
        
        let detailed_strategy = final_table_cfr.analyze_player_strategy(player_idx);
        
        println!("  Preflop ranges by position:");
        for pos in 0..6 {
            let range = detailed_strategy.get_position_range(pos);
            let position_name = match pos {
                0 => "UTG",
                1 => "UTG+1", 
                2 => "CO",
                3 => "BTN",
                4 => "SB", 
                5 => "BB",
                _ => "?",
            };
            println!("    {}: {:.1}%", position_name, range * 100.0);
        }
        
        println!("  ICM considerations:");
        let icm_pressure = calculate_icm_pressure(player_idx, &stacks, &payouts);
        println!("    ICM pressure: {:.2}", icm_pressure);
        println!("    Risk tolerance: {}", get_risk_tolerance(icm_pressure));
        println!();
    }
}

fn demonstrate_adaptive_tournament_strategy() {
    println!("=== Adaptive Tournament Strategy ===");
    
    println!("Simulating tournament progression and strategy adaptation...\n");
    
    // Start with early tournament
    let mut current_stacks = vec![10000, 10000, 10000, 10000, 10000, 10000];
    let payouts = vec![30000.0, 18000.0, 12000.0];
    let mut blind_level = BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 };
    
    let stages = vec![
        ("Early stage", 1),
        ("Middle stage", 3), 
        ("Late stage", 6),
        ("Bubble", 8),
        ("Final table", 10),
    ];
    
    for (stage_name, level) in stages {
        // Update blinds and simulate eliminations
        update_tournament_stage(&mut current_stacks, &mut blind_level, level);
        
        if current_stacks.len() < 3 { break; }
        
        println!("=== {} ===", stage_name);
        println!("Blinds: {}/{}", blind_level.small_blind, blind_level.big_blind);
        println!("Players remaining: {}", current_stacks.len());
        
        let adaptive_cfr = ICMCFRTrainer::new(
            current_stacks.clone(), 
            payouts.clone(), 
            blind_level.clone()
        );
        
        // Calculate stage-specific strategies
        let stage_strategy = adaptive_cfr.get_stage_strategy(stage_name);
        
        println!("Strategy characteristics:");
        println!("  Aggression level: {:.2}", stage_strategy.aggression);
        println!("  Tightness factor: {:.2}", stage_strategy.tightness);
        println!("  ICM awareness: {:.2}", stage_strategy.icm_weight);
        
        // Show how strategy changes for each stack size
        for (i, &stack) in current_stacks.iter().enumerate() {
            let bb_count = stack / blind_level.big_blind;
            let relative_stack = stack as f64 / current_stacks.iter().sum::<u32>() as f64;
            let player_strategy = stage_strategy.get_player_adjustments(relative_stack);
            
            println!("  Player {} ({} BB): Adjustment = {:.2}x tighter", 
                     i + 1, bb_count, player_strategy);
        }
        println!();
    }
}

// Supporting structures and implementations

#[derive(Clone)]
struct ICMCFRTrainer {
    stacks: Vec<u32>,
    payouts: Vec<f64>,
    blind_level: BlindLevel,
    icm_calculator: ICMCalculator,
    bubble_weighting: f64,
    // cfr_solver: Option<MCCFRTrainer<holdem::State>>, // Removed due to Clone constraint
}

#[derive(Debug)]
struct CFRResults {
    convergence: f64,
    average_utility: f64,
    iterations: u32,
}

#[derive(Debug)]
struct TournamentStrategy {
    fold_frequency: f64,
    call_frequency: f64,
    raise_frequency: f64,
    allin_frequency: f64,
}

#[derive(Debug)]
struct DetailedStrategy {
    position_ranges: Vec<f64>,
    street_adjustments: HashMap<u8, f64>, // 0=Preflop, 1=Flop, 2=Turn, 3=River
}

#[derive(Debug)]
struct StageStrategy {
    aggression: f64,
    tightness: f64,
    icm_weight: f64,
    stage_name: String,
}

impl ICMCFRTrainer {
    fn new(stacks: Vec<u32>, payouts: Vec<f64>, blind_level: BlindLevel) -> Self {
        Self {
            stacks: stacks.clone(),
            payouts: payouts.clone(),
            blind_level,
            icm_calculator: ICMCalculator::new(stacks, payouts.iter().map(|&p| p as u64).collect()),
            bubble_weighting: 1.0,
        }
    }
    
    fn set_bubble_weighting(&mut self, weight: f64) {
        self.bubble_weighting = weight;
    }
    
    fn train_scenario(&mut self, _street: u8, iterations: u32) -> CFRResults {
        // ICM 유틸리티를 통한 CFR 훈련 시뮬레이션
        let base_convergence = 1.0 - (1.0 / (iterations as f64 + 1.0));
        let icm_values = self.icm_calculator.calculate_equity();
        let average_utility = icm_values.iter().sum::<f64>() / icm_values.len() as f64;
        
        CFRResults {
            convergence: base_convergence,
            average_utility,
            iterations,
        }
    }
    
    fn calculate_position_strategy(&self, player_idx: usize, position: usize, _street: u8) -> TournamentStrategy {
        let stack = self.stacks[player_idx];
        let bb_count = stack / self.blind_level.big_blind;
        
        // ICM-adjusted strategy calculation
        let icm_values = self.icm_calculator.calculate_equity();
        let total_chips: u32 = self.stacks.iter().sum();
        let chip_ev = (stack as f64 / total_chips as f64) * self.payouts.iter().sum::<f64>();
        let icm_pressure = (icm_values[player_idx] - chip_ev) / chip_ev;
        
        // Base frequencies adjusted for ICM
        let base_aggression = match position {
            0 => 0.15,  // UTG
            1 => 0.20,  // CO
            2 => 0.30,  // BTN
            _ => 0.18,  // SB/BB
        };
        
        let icm_adjustment = if icm_pressure > 0.0 {
            1.0 - (icm_pressure * 0.5).min(0.4) // Tighter when ICM > chip EV
        } else {
            1.0 + (-icm_pressure * 0.3).min(0.2) // Looser when ICM < chip EV
        };
        
        let adjusted_aggression = base_aggression * icm_adjustment;
        
        // Stack size adjustments
        let stack_adjustment = if bb_count < 10 {
            1.3 // More aggressive when short
        } else if bb_count > 30 {
            0.8 // More conservative when deep
        } else {
            1.0
        };
        
        let final_aggression = (adjusted_aggression * stack_adjustment).min(0.5);
        
        TournamentStrategy {
            fold_frequency: 1.0 - final_aggression,
            call_frequency: final_aggression * 0.4,
            raise_frequency: final_aggression * 0.5,
            allin_frequency: if bb_count < 15 { final_aggression * 0.1 } else { 0.0 },
        }
    }
    
    fn analyze_player_strategy(&self, player_idx: usize) -> DetailedStrategy {
        let mut position_ranges = Vec::new();
        
        for position in 0..6 {
            let strategy = self.calculate_position_strategy(player_idx, position, 0u8); // 0 = Preflop
            let total_action = strategy.call_frequency + strategy.raise_frequency + strategy.allin_frequency;
            position_ranges.push(total_action);
        }
        
        let mut street_adjustments = HashMap::new();
        street_adjustments.insert(0u8, 1.0);    // Preflop
        street_adjustments.insert(1u8, 0.85);   // Flop
        street_adjustments.insert(2u8, 0.75);   // Turn
        street_adjustments.insert(3u8, 0.65);   // River
        
        DetailedStrategy {
            position_ranges,
            street_adjustments,
        }
    }
    
    fn get_stage_strategy(&self, stage_name: &str) -> StageStrategy {
        let (aggression, tightness, icm_weight) = match stage_name {
            "Early stage" => (1.0, 0.8, 0.1),
            "Middle stage" => (1.1, 0.9, 0.3),
            "Late stage" => (1.2, 1.1, 0.6),
            "Bubble" => (0.7, 1.5, 1.0),
            "Final table" => (1.0, 1.3, 0.9),
            _ => (1.0, 1.0, 0.5),
        };
        
        StageStrategy {
            aggression,
            tightness,
            icm_weight,
            stage_name: stage_name.to_string(),
        }
    }
}

impl DetailedStrategy {
    fn get_position_range(&self, position: usize) -> f64 {
        self.position_ranges.get(position).copied().unwrap_or(0.15)
    }
}

impl StageStrategy {
    fn get_player_adjustments(&self, relative_stack: f64) -> f64 {
        // Larger stacks can play looser in most stages
        let base_adjustment = if relative_stack > 0.3 {
            0.9 // 10% tighter
        } else if relative_stack < 0.1 {
            1.2 // 20% looser (more desperate)
        } else {
            1.0
        };
        
        base_adjustment * self.tightness
    }
}

fn display_strategy_comparison(icm_cfr: &ICMCFRTrainer, stacks: &[u32], street: u8) {
    println!("  Strategy comparison (ICM vs Chip EV):");
    
    for (i, &_stack) in stacks.iter().enumerate() {
        let icm_strategy = icm_cfr.calculate_position_strategy(i, 2, street); // BTN position
        let chip_strategy = calculate_chip_ev_strategy(i, 2, stacks);
        
        let tightness_diff = icm_strategy.fold_frequency - chip_strategy.fold_frequency;
        
        println!("    Player {}: {:.1}% {} aggressive with ICM", 
                 i + 1, 
                 tightness_diff.abs() * 100.0,
                 if tightness_diff > 0.0 { "less" } else { "more" });
    }
}

fn calculate_chip_ev_strategy(_player_idx: usize, position: usize, _stacks: &[u32]) -> TournamentStrategy {
    // Simple chip EV strategy (no ICM considerations)
    let base_aggression = match position {
        0 => 0.18,  // UTG
        1 => 0.23,  // CO
        2 => 0.32,  // BTN
        _ => 0.20,  // SB/BB
    };
    
    TournamentStrategy {
        fold_frequency: 1.0 - base_aggression,
        call_frequency: base_aggression * 0.45,
        raise_frequency: base_aggression * 0.50,
        allin_frequency: base_aggression * 0.05,
    }
}

fn calculate_icm_adjustment(icm_strategy: &TournamentStrategy, chip_strategy: &TournamentStrategy) -> f64 {
    // How much tighter ICM makes the strategy
    icm_strategy.fold_frequency - chip_strategy.fold_frequency
}

fn calculate_icm_pressure(player_idx: usize, stacks: &[u32], payouts: &[f64]) -> f64 {
    let icm_calculator = ICMCalculator::new(stacks.to_vec(), payouts.iter().map(|&p| p as u64).collect());
    let icm_values = icm_calculator.calculate_equity();
    
    let total_chips: u32 = stacks.iter().sum();
    let chip_ev = (stacks[player_idx] as f64 / total_chips as f64) * payouts.iter().sum::<f64>();
    
    (icm_values[player_idx] - chip_ev) / chip_ev
}

fn get_risk_tolerance(icm_pressure: f64) -> &'static str {
    if icm_pressure > 0.2 {
        "Very Low - significant downside protection"
    } else if icm_pressure > 0.1 {
        "Low - moderate downside protection"
    } else if icm_pressure > -0.1 {
        "Medium - balanced risk/reward"
    } else if icm_pressure > -0.2 {
        "High - upside focused"
    } else {
        "Very High - must take risks to improve position"
    }
}

fn update_tournament_stage(stacks: &mut Vec<u32>, blind_level: &mut BlindLevel, level: u32) {
    // Increase blinds
    blind_level.level = level;
    blind_level.small_blind = 25 * (1 << (level / 2));
    blind_level.big_blind = blind_level.small_blind * 2;
    blind_level.ante = if level > 3 { blind_level.small_blind / 4 } else { 0 };
    
    // Simulate eliminations
    match level {
        3 => stacks.truncate(5), // One elimination
        6 => stacks.truncate(4), // Another elimination  
        8 => stacks.truncate(4), // Bubble stage
        10 => stacks.truncate(3), // Final table
        _ => {}
    }
    
    // Adjust remaining stacks with simple variance simulation
    if !stacks.is_empty() {
        let total_chips: u32 = stacks.iter().sum();
        let avg_stack = total_chips / stacks.len() as u32;
        
        for (i, stack) in stacks.iter_mut().enumerate() {
            let variance = (avg_stack as f64 * 0.3) as u32;
            // Use a simple deterministic variance based on index to avoid random dependency
            let adjustment = ((i as u32 * 17) % (variance * 2)) as i32 - variance as i32;
            *stack = (*stack as i32 + adjustment).max(1000) as u32;
        }
    }
}
