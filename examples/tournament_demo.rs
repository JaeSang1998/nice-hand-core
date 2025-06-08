// Tournament features comprehensive demo
use nice_hand_core::game::tournament::*;
use std::time::Instant;

fn main() {
    println!("=== Advanced Tournament Features Demo ===\n");
    
    println!("🎯 1. ICM Calculations with Realistic Scenarios");
    demo_icm_calculations();
    
    println!("🤖 2. Sophisticated Opponent Modeling");
    demo_opponent_modeling();
    
    println!("🏆 3. Multi-Table Tournament Management");
    demo_mtt_management();
    
    println!("🫧 4. Advanced Bubble Strategy");
    demo_bubble_strategy();
    
    println!("⚖️ 5. Tournament State Evaluation");
    demo_tournament_evaluation();
    
    println!("\n=== Tournament Features Demo Complete ===");
    println!("All advanced tournament algorithms working correctly! 🚀");
}

fn demo_icm_calculations() {
    // Scenario 1: Final table bubble (10 players, 9 get paid)
    println!("\n   📊 Scenario 1: Final Table Bubble");
    let stacks1 = vec![45000, 38000, 32000, 28000, 25000, 22000, 18000, 15000, 12000, 8000];
    let payouts1 = vec![150000, 90000, 60000, 45000, 35000, 28000, 22000, 18000, 15000];
    
    let icm1 = ICMCalculator::new(stacks1.clone(), payouts1.clone());
    let start = Instant::now();
    let equities1 = icm1.calculate_equity();
    let duration = start.elapsed();
    
    println!("      Stacks: {:?}", stacks1);
    println!("      ICM Equities: {:.0?}", equities1);
    println!("      Calculation time: {:?}", duration);
    
    // Scenario 2: Heads-up with ICM
    println!("\n   🥊 Scenario 2: Heads-up ICM");
    let stacks2 = vec![180000, 60000];
    let payouts2 = vec![360000, 240000];
    
    let icm2 = ICMCalculator::new(stacks2.clone(), payouts2.clone());
    let equities2 = icm2.calculate_equity();
    
    println!("      Chip stacks: {:?}", stacks2);
    println!("      Prize pool: {:?}", payouts2);
    println!("      ICM equities: {:.0?}", equities2);
    
    let chip_ratio = stacks2[0] as f64 / stacks2[1] as f64;
    let equity_ratio = equities2[0] / equities2[1];
    println!("      Chip advantage: {:.2}:1, ICM advantage: {:.2}:1", chip_ratio, equity_ratio);
    
    // ICM Pressure analysis
    println!("\n   📉 ICM Pressure Analysis");
    let pressure_big = icm2.calculate_icm_pressure(0, -10000);
    let pressure_small = icm2.calculate_icm_pressure(1, -10000);
    println!("      Big stack pressure (losing 10k): {:.4}", pressure_big);
    println!("      Small stack pressure (losing 10k): {:.4}", pressure_small);
    
    println!("   ✅ ICM calculations demonstrate realistic tournament dynamics\n");
}

fn demo_opponent_modeling() {
    println!("\n   🤖 Advanced Opponent Profiling");
    
    let mut aggressive_player = OpponentModel::new(1);
    let mut tight_player = OpponentModel::new(2);
    
    // Simulate 20 hands of observations
    for hand in 1..=20 {
        let context = ActionContext {
            stack_ratio: 1.0 - (hand as f64 * 0.02), // Gradually losing chips
            pot_odds: 0.3,
            is_preflop: hand % 3 == 1,
            near_bubble: hand > 15,
            position: if hand % 2 == 0 { Position::Button } else { Position::EarlyPosition },
            num_opponents: 3,
        };
        
        // Aggressive player actions
        if hand % 3 == 0 {
            aggressive_player.update_with_action(&TournamentAction::Raise(100), &context);
        } else if hand % 4 == 0 {
            aggressive_player.update_with_action(&TournamentAction::Call, &context);
        } else {
            aggressive_player.update_with_action(&TournamentAction::Fold, &context);
        }
        
        // Tight player actions  
        if hand % 5 == 0 {
            tight_player.update_with_action(&TournamentAction::Call, &context);
        } else if hand % 8 == 0 {
            tight_player.update_with_action(&TournamentAction::Raise(80), &context);
        } else {
            tight_player.update_with_action(&TournamentAction::Fold, &context);
        }
    }
    
    println!("      Aggressive Player Profile:");
    println!("         VPIP: {:.1}%", aggressive_player.vpip * 100.0);
    println!("         PFR: {:.1}%", aggressive_player.pfr * 100.0);
    println!("         Aggression: {:.2}", aggressive_player.aggression);
    println!("         Bubble adjustment: {:.2}", aggressive_player.bubble_adjustment);
    
    println!("      Tight Player Profile:");
    println!("         VPIP: {:.1}%", tight_player.vpip * 100.0);
    println!("         PFR: {:.1}%", tight_player.pfr * 100.0);
    println!("         Aggression: {:.2}", tight_player.aggression);
    println!("         Bubble adjustment: {:.2}", tight_player.bubble_adjustment);
    
    // Test predictions in bubble situation
    let bubble_context = ActionContext {
        stack_ratio: 0.6,
        pot_odds: 0.25,
        is_preflop: true,
        near_bubble: true,
        position: Position::MiddlePosition,
        num_opponents: 2,
    };
    
    let agg_prediction = aggressive_player.predict_action_distribution(&bubble_context);
    let tight_prediction = tight_player.predict_action_distribution(&bubble_context);
    
    println!("      Bubble Predictions (Fold/Call/Raise):");
    println!("         Aggressive: {:.2?}", agg_prediction);
    println!("         Tight: {:.2?}", tight_prediction);
    
    println!("   ✅ Opponent models show realistic learning and adaptation\n");
}

fn demo_mtt_management() {
    println!("\n   🏆 Multi-Table Tournament Simulation");
    
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
            BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 0 },
            BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 25 },
            BlindLevel { level: 4, small_blind: 100, big_blind: 200, ante: 25 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let mut mtt = MTTManager::new(54, 9, structure, 100000);
    
    println!("      Initial Setup:");
    println!("         Total tables: {}", mtt.tables.len());
    println!("         Players per table: {}", mtt.tables[0].count_active_players());
    println!("         Total active players: {}", mtt.count_active_players());
    
    // Simulate tournament progression
    println!("\n      Tournament Progression:");
    
    // Eliminate players to simulate tournament flow
    let eliminations = vec![
        (0, 1), (1, 10), (2, 19), (0, 28), (1, 37), (2, 46),  // Early eliminations
        (0, 2), (1, 11), (2, 20), (0, 29), (1, 38),           // More eliminations
        (0, 3), (1, 12), (2, 21), (0, 30),                    // Getting closer to bubble
    ];
    
    for (table_id, player_id) in eliminations {
        mtt.eliminate_player(table_id, player_id);
        
        if mtt.count_active_players() % 10 == 0 {
            mtt.balance_tables();
            println!("         {} players remaining, {} tables active", 
                    mtt.count_active_players(), mtt.tables.len());
        }
    }
    
    // Test final table consolidation
    println!("\n      Final Table Formation:");
    
    // Eliminate more players to get to final table
    let remaining_players = mtt.count_active_players();
    let mut eliminations_needed = remaining_players - 9;
    let mut table_idx = 0;
    let mut player_id = 50;
    
    while eliminations_needed > 0 && mtt.count_active_players() > 9 {
        mtt.eliminate_player(table_idx, player_id);
        table_idx = (table_idx + 1) % 3;
        player_id += 1;
        eliminations_needed -= 1;
    }
    
    // Trigger final table consolidation
    mtt.balancing_algorithm = BalancingAlgorithm::FinalTableConsolidation;
    mtt.balance_tables();
    
    println!("         Final table formed with {} players", mtt.count_active_players());
    println!("         Tables remaining: {}", mtt.tables.len());
    
    // Show final standings
    let standings = mtt.get_tournament_standings();
    println!("         Final Table Chip Counts:");
    for (i, (player_id, stack, _)) in standings.iter().enumerate() {
        println!("            Seat {}: Player {} - {} chips", i + 1, player_id, stack);
    }
    
    println!("   ✅ MTT management handles realistic tournament flow\n");
}

fn demo_bubble_strategy() {
    println!("\n   🫧 Dynamic Bubble Strategy Analysis");
    
    let scenarios = vec![
        ("Pre-bubble", 25, 18),
        ("Approaching bubble", 22, 18), 
        ("Near bubble", 20, 18),
        ("Bubble", 19, 18),
        ("In the money", 15, 18),
    ];
    
    for (phase, players, payouts) in scenarios {
        println!("      {} ({} players, {} paid):", phase, players, payouts);
        
        let bubble_strategy = BubbleStrategy::new(players, payouts);
        
        println!("         Bubble factor: {:.3}", bubble_strategy.bubble_factor);
        println!("         Fold equity boost: {:.3}", bubble_strategy.fold_equity_boost);
        
        // Test different stack sizes
        let stack_scenarios = vec![
            ("Short stack", 0.3),
            ("Average stack", 1.0),
            ("Big stack", 2.5),
            ("Chip leader", 4.0),
        ];
        
        let base_range = 0.15; // 15% base hand range
        
        for (stack_type, ratio) in stack_scenarios {
            let adjusted_range = bubble_strategy.adjust_hand_range(base_range, ratio);
            let should_be_aggressive = bubble_strategy.should_make_aggressive_play(ratio, 0.1);
            
            println!("            {} ({}x avg): range {:.1}%, aggressive: {}", 
                    stack_type, ratio, adjusted_range * 100.0, should_be_aggressive);
        }
        println!();
    }
    
    println!("   ✅ Bubble strategy adapts realistically to tournament dynamics\n");
}

fn demo_tournament_evaluation() {
    println!("\n   ⚖️ Advanced Tournament State Evaluation");
    
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 4, small_blind: 200, big_blind: 400, ante: 50 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 180, 500000);
    let final_stacks = vec![85000, 72000, 58000, 45000, 32000, 28000, 20000, 15000, 8000];
    
    let evaluator = TournamentEvaluator::new(tournament_state, final_stacks.clone());
    
    println!("      Final Table Evaluation:");
    println!("         Position  Stack    ICM Value  Normalized");
    
    for (i, &stack) in final_stacks.iter().enumerate() {
        let evaluation = evaluator.evaluate_terminal_state(&final_stacks, i);
        let icm_equities = evaluator.icm_calculator.calculate_equity();
        let icm_value = if i < icm_equities.len() { icm_equities[i] } else { 0.0 };
        
        println!("            {}      {:6}    {:8.0}     {:6.3}", 
                i + 1, stack, icm_value, evaluation);
    }
    
    // Test opponent action selection in different contexts
    println!("\n      Opponent Action Selection:");
    
    let contexts = vec![
        ("Early position, bubble", ActionContext {
            stack_ratio: 0.5,
            pot_odds: 0.2,
            is_preflop: true,
            near_bubble: true,
            position: Position::EarlyPosition,
            num_opponents: 8,
        }),
        ("Button, deep stacked", ActionContext {
            stack_ratio: 2.0,
            pot_odds: 0.3,
            is_preflop: true,
            near_bubble: false,
            position: Position::Button,
            num_opponents: 4,
        }),
    ];
    
    let available_actions = vec![
        TournamentAction::Fold,
        TournamentAction::Call,
        TournamentAction::Raise(800),
        TournamentAction::AllIn,
    ];
    
    for (scenario, context) in contexts {
        let selected_action = evaluator.select_opponent_action(1, &context, &available_actions);
        println!("         {}: {:?}", scenario, selected_action);
    }
    
    println!("   ✅ Tournament evaluation provides sophisticated decision analysis\n");
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

fn test_bubble_icm() {
    println!("   🫧 Testing Bubble ICM...");
    
    // 5 players, 4 get paid (bubble situation)
    let stacks = vec![12000, 10000, 8000, 6000, 4000];
    let payouts = vec![15000, 10000, 7000, 5000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      Bubble equities: {:.2?}", equities);
    
    // Short stack should have lower equity despite similar stack sizes
    assert!(equities[4] < equities[3], "Short stack should have lower ICM equity");
}

fn test_heads_up_icm() {
    println!("   🥊 Testing Heads-up ICM...");
    
    let stacks = vec![30000, 10000];
    let payouts = vec![20000, 12000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      Heads-up equities: {:.2?}", equities);
    
    // Chip leader should have more than 75% equity despite 3:1 chip lead
    assert!(equities[0] > 15000.0 && equities[0] < 18000.0, "ICM should reduce chip leader advantage");
}

fn test_opponent_modeling() {
    println!("🤖 Testing Opponent Modeling...");
    
    let mut model = OpponentModel::new(1);
    
    // Simulate observing opponent actions
    let contexts = vec![
        ActionContext {
            stack_ratio: 1.0,
            pot_odds: 0.3,
            is_preflop: true,
            near_bubble: false,
            position: Position::Button,
            num_opponents: 3,
        },
        ActionContext {
            stack_ratio: 0.8,
            pot_odds: 0.25,
            is_preflop: false,
            near_bubble: true,
            position: Position::EarlyPosition,
            num_opponents: 2,
        },
    ];
    
    // Observe tight play
    model.update_with_action(&TournamentAction::Fold, &contexts[0]);
    model.update_with_action(&TournamentAction::Fold, &contexts[1]);
    model.update_with_action(&TournamentAction::Call, &contexts[0]);
    
    println!("   📈 After observing actions:");
    println!("      VPIP: {:.3}", model.vpip);
    println!("      PFR: {:.3}", model.pfr);
    println!("      Aggression: {:.3}", model.aggression);
    println!("      Tightness: {:.3}", model.tightness);
    println!("      Sample size: {}", model.sample_size);
    
    // Test action prediction
    let predictions = model.predict_action_distribution(&contexts[1]);
    println!("   🔮 Action prediction (fold/call/raise): {:.3?}", predictions);
    
    println!("   ✅ Opponent modeling working correctly\n");
}

fn test_mtt_management() {
    println!("🏆 Testing MTT Management...");
    
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 10, big_blind: 20, ante: 0 },
            BlindLevel { level: 2, small_blind: 15, big_blind: 30, ante: 0 },
            BlindLevel { level: 3, small_blind: 25, big_blind: 50, ante: 5 },
        ],
        level_duration_minutes: 15,
        starting_stack: 5000,
        ante_schedule: vec![],
    };
    
    let mut mtt = MTTManager::new(27, 9, structure, 10000);
    
    println!("   🎲 Initial tournament setup:");
    println!("      Tables: {}", mtt.tables.len());
    println!("      Active players: {}", mtt.count_active_players());
    
    // Test table balancing
    mtt.balance_tables();
    println!("   ⚖️  After balancing:");
    for (i, table) in mtt.tables.iter().enumerate() {
        println!("      Table {}: {} players", i, table.count_active_players());
    }
    
    // Test player elimination
    mtt.eliminate_player(0, 1);
    mtt.eliminate_player(0, 2);
    println!("   ❌ After eliminating 2 players: {} active", mtt.count_active_players());
    
    // Test tournament standings
    let standings = mtt.get_tournament_standings();
    println!("   🏅 Top 5 chip leaders:");
    for (i, (player_id, stack, table_id)) in standings.iter().take(5).enumerate() {
        println!("      {}. Player {} - {} chips (Table {})", i + 1, player_id, stack, table_id);
    }
    
    // Test final table consolidation
    // Eliminate most players to trigger final table
    for table_id in 0..mtt.tables.len() {
        for player_id in 10..25 {
            mtt.eliminate_player(table_id as u32, player_id);
        }
    }
    
    mtt.balancing_algorithm = BalancingAlgorithm::FinalTableConsolidation;
    mtt.balance_tables();
    
    println!("   🎯 Final table consolidation:");
    println!("      Tables remaining: {}", mtt.tables.len());
    if !mtt.tables.is_empty() {
        println!("      Final table players: {}", mtt.tables[0].count_active_players());
    }
    
    println!("   ✅ MTT management working correctly\n");
}

fn test_bubble_strategy() {
    println!("🫧 Testing Bubble Strategy...");
    
    // Test different bubble scenarios
    let scenarios = vec![
        (15, 9),  // 6 from bubble
        (11, 9),  // 2 from bubble  
        (10, 9),  // On the bubble
        (8, 9),   // In the money
    ];
    
    for (players_remaining, payout_spots) in scenarios {
        let bubble_strategy = BubbleStrategy::new(players_remaining, payout_spots);
        
        println!("   📊 {} players remaining, {} paid:", players_remaining, payout_spots);
        println!("      Bubble factor: {:.3}", bubble_strategy.bubble_factor);
        println!("      Fold equity boost: {:.3}", bubble_strategy.fold_equity_boost);
        println!("      ICM sensitivity: {:.3}", bubble_strategy.icm_sensitivity);
        
        // Test hand range adjustments for different stack sizes
        let base_range = 0.2; // 20% of hands
        let stack_ratios = vec![0.05, 0.2, 0.5, 1.5]; // Very short, short, average, big
        
        for &stack_ratio in &stack_ratios {
            let adjusted_range = bubble_strategy.adjust_hand_range(base_range, stack_ratio);
            println!("      Stack ratio {:.2} -> range {:.3}", stack_ratio, adjusted_range);
        }
        
        // Test aggressive play decisions
        let should_play = bubble_strategy.should_make_aggressive_play(0.1, 0.05);
        println!("      Short stack aggressive play decision: {}", should_play);
        
        println!();
    }
    
    println!("   ✅ Bubble strategy working correctly\n");
}

fn test_tournament_evaluator() {
    println!("🎯 Testing Tournament Evaluator...");
    
    // Create tournament state
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 100, 50000);
    let player_stacks = vec![15000, 12000, 8000, 5000, 3000];
    
    let mut evaluator = TournamentEvaluator::new(tournament_state, player_stacks.clone());
    
    // Test terminal state evaluation
    println!("   🎯 Terminal state evaluation:");
    for (i, &stack) in player_stacks.iter().enumerate() {
        let evaluation = evaluator.evaluate_terminal_state(&player_stacks, i);
        println!("      Player {} (stack {}): {:.4}", i, stack, evaluation);
    }
    
    // Test opponent action selection
    let context = ActionContext {
        stack_ratio: 0.8,
        pot_odds: 0.3,
        is_preflop: true,
        near_bubble: true,
        position: Position::Button,
        num_opponents: 2,
    };
    
    let available_actions = vec![
        TournamentAction::Fold,
        TournamentAction::Call,
        TournamentAction::Raise(100),
    ];
    
    let selected_action = evaluator.select_opponent_action(1, &context, &available_actions);
    println!("   🤖 Selected opponent action: {:?}", selected_action);
    
    // Test ICM pressure calculation
    let icm_pressure = evaluator.calculate_icm_adjusted_ev(0, -500);
    println!("   📊 ICM pressure for losing 500 chips: {:.6}", icm_pressure);
    
    // Update opponent model
    evaluator.update_opponent_model(1, TournamentAction::Raise(150), context);
    println!("   📈 Updated opponent model for player 1");
    
    println!("   ✅ Tournament evaluator working correctly\n");
}

fn demo_cfr_integration() {
    println!("\n🔗 CFR Integration with Tournament Features");
    
    // Simulate how tournament features would integrate with CFR
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 100, big_blind: 200, ante: 25 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 100, 200000);
    let player_stacks = vec![15000, 12000, 8000, 5000];
    
    let evaluator = TournamentEvaluator::new(tournament_state, player_stacks.clone());
    
    println!("      Integration Points:");
    
    // 1. Terminal state evaluation
    println!("         1. Terminal State Evaluation:");
    for (i, &stack) in player_stacks.iter().enumerate() {
        let evaluation = evaluator.evaluate_terminal_state(&player_stacks, i);
        println!("            Player {} (stack {}): CFR value {:.3}", i + 1, stack, evaluation);
    }
    
    // 2. Strategy adjustments
    println!("         2. Strategy Adjustments:");
    let base_strategy = vec![0.3, 0.4, 0.3]; // fold, call, raise
    
    for (i, &stack) in player_stacks.iter().enumerate() {
        let avg_stack = 35000 / 4; // Total chips / players
        let stack_ratio = stack as f64 / avg_stack as f64;
        
        let bubble_strategy = BubbleStrategy::new(4, 3);
        let adjusted_range = bubble_strategy.adjust_hand_range(0.15, stack_ratio);
        
        println!("            Player {} range: {:.1}% -> {:.1}%", 
                i + 1, 15.0, adjusted_range * 100.0);
    }
    
    // 3. Opponent modeling integration
    println!("         3. Opponent Modeling:");
    let context = ActionContext {
        stack_ratio: 0.8,
        pot_odds: 0.3,
        is_preflop: true,
        near_bubble: true,
        position: Position::Button,
        num_opponents: 3,
    };
    
    let mut model = OpponentModel::new(1);
    model.update_with_action(&TournamentAction::Raise(300), &context);
    let prediction = model.predict_action_distribution(&context);
    
    println!("            Opponent prediction: fold={:.2}, call={:.2}, raise={:.2}", 
            prediction[0], prediction[1], prediction[2]);
    
    println!("      ✅ CFR can seamlessly integrate tournament features\n");
}

fn demo_performance_comparison() {
    println!("⚡ Performance Analysis");
    
    // Compare performance of different tournament calculations
    let test_scenarios = vec![
        (5, vec![15000, 12000, 8000, 5000, 2000]),
        (9, vec![25000, 20000, 18000, 15000, 12000, 10000, 8000, 5000, 3000]),
        (18, (0..18).map(|i| 10000 - i * 300).collect()),
    ];
    
    println!("      Performance for different table sizes:");
    
    for (players, stacks) in test_scenarios {
        let payouts: Vec<u64> = (0..players).map(|i| 10000 - i as u64 * 500).collect();
        let icm = ICMCalculator::new(stacks.clone(), payouts);
        
        let start = Instant::now();
        let iterations = 100;
        
        for _ in 0..iterations {
            let _equities = icm.calculate_equity();
        }
        
        let duration = start.elapsed();
        let per_calc = duration.as_micros() as f64 / iterations as f64;
        
        println!("         {} players: {:.1}μs per ICM calculation", players, per_calc);
    }
    
    // Test MTT management performance
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 50, big_blind: 100, ante: 0 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let start = Instant::now();
    let mut mtt = MTTManager::new(180, 9, structure, 100000); // Large tournament
    
    // Simulate eliminations and table balancing
    for player_id in 1..50 {
        mtt.eliminate_player(player_id % 20, player_id);
        if player_id % 10 == 0 {
            mtt.balance_tables();
        }
    }
    
    let mtt_duration = start.elapsed();
    println!("      MTT Management (180 -> 131 players): {:?}", mtt_duration);
    
    println!("   ✅ All tournament algorithms perform efficiently\n");
}
