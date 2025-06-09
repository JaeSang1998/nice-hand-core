// Tournament-CFR Integration Complete Demo
// CFR 전략과 함께하는 실시간 토너먼트 의사결정을 보여줍니다

use nice_hand_core::{
    TournamentHoldem, TournamentHoldemState, TournamentCFRTrainer,
    game::tournament::{TournamentState, TournamentStructure, BlindLevel},
    game::holdem::State as HoldemState,
    solver::cfr_core::Game,
};
use std::time::Instant;

fn main() {
    println!("🎯 Tournament-CFR Integration Complete Demo");
    println!("{}", "=".repeat(50));
    
    // Phase 1: Setup tournament structure
    demo_tournament_setup();
    
    // Phase 2: ICM calculations and bubble play
    demo_icm_bubble_strategy();
    
    // Phase 3: Tournament-specific game states
    demo_tournament_game_states();
    
    // Phase 4: CFR training integration
    demo_cfr_integration();
    
    // Phase 5: 실시간 의사결정
    demo_realtime_decisions();
    
    println!("\n🎉 Tournament-CFR Integration Demo Complete!");
}

fn demo_tournament_setup() {
    println!("\n📋 Phase 1: Tournament Structure Setup");
    println!("{}", "-".repeat(30));
    
    // Create tournament structure with progressive blinds
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
            BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 0 },
            BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 25 },
            BlindLevel { level: 4, small_blind: 100, big_blind: 200, ante: 25 },
            BlindLevel { level: 5, small_blind: 150, big_blind: 300, ante: 50 },
        ],
        level_duration_minutes: 15,
        starting_stack: 1500,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 100, 10000);
    
    println!("🏆 Tournament: {} players, ${} prize pool", 
             tournament_state.total_players, tournament_state.prize_pool);
    println!("💰 Payout structure: {} positions paid", 
             tournament_state.payout_structure.len());
    println!("🃏 Starting stacks: {} chips", tournament_state.structure.starting_stack);
    
    let (sb, bb, ante) = tournament_state.current_blinds();
    println!("🎲 Current blinds: {}/{} (ante: {})", sb, bb, ante);
}

fn demo_icm_bubble_strategy() {
    println!("\n💹 Phase 2: ICM Calculations & Bubble Strategy");
    println!("{}", "-".repeat(30));
    
    // Create tournament state near the bubble
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 4, small_blind: 100, big_blind: 200, ante: 25 },
        ],
        level_duration_minutes: 15,
        starting_stack: 1500,
        ante_schedule: vec![],
    };
    
    let mut tournament_state = TournamentState::new(structure, 100, 10000);
    tournament_state.players_remaining = 12; // Near bubble (top 10 get paid)
    
    // Simulate different stack scenarios
    let scenarios = vec![
        ("Big Stack", vec![8000, 5000, 4000, 3000, 2000]),
        ("Average Stack", vec![4500, 4500, 4500, 4500, 4500]),
        ("Short Stack", vec![1000, 6000, 5000, 4000, 4000]),
    ];
    
    for (scenario_name, stacks) in scenarios {
        println!("\n📊 Scenario: {}", scenario_name);
        
        // Create tournament game for this scenario
        let tournament_game = TournamentHoldem::new(tournament_state.clone(), stacks.clone());
        let icm_values = &tournament_game.evaluator.icm_calculator.calculate_equity();
        
        println!("  Stack sizes: {:?}", stacks);
        println!("  ICM values:  {:?}", 
                 icm_values.iter().map(|v| format!("${:.0}", v)).collect::<Vec<_>>());
        
        // Calculate bubble pressure for each player
        for (i, (&stack, &icm_value)) in stacks.iter().zip(icm_values.iter()).enumerate() {
            let avg_stack = stacks.iter().sum::<u32>() as f64 / stacks.len() as f64;
            let stack_ratio = stack as f64 / avg_stack;
            let bubble_pressure = calculate_bubble_pressure(&tournament_state, stack, stack_ratio);
            
            println!("    Player {}: Stack ratio {:.2}, ICM ${:.0}, Bubble pressure {:.2}", 
                     i + 1, stack_ratio, icm_value, bubble_pressure);
        }
    }
}

fn demo_tournament_game_states() {
    println!("\n🎮 Phase 3: Tournament-Specific Game States");
    println!("{}", "-".repeat(30));
    
    // Create heads-up tournament scenario for simplicity
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 25 },
        ],
        level_duration_minutes: 15,
        starting_stack: 1500,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 2, 1000);
    let holdem_state = HoldemState::new(); // Creates 2-player heads-up game
    let player_stacks = vec![1200, 800]; // Asymmetric stacks
    
    let tournament_holdem_state = TournamentHoldemState::new_tournament_hand(
        holdem_state,
        tournament_state,
        player_stacks.clone(),
    );
    
    println!("🎯 Tournament Hold'em State Created:");
    println!("  Player stacks: {:?}", player_stacks);
    println!("  ICM values: {:?}", 
             tournament_holdem_state.icm_values.iter()
                 .map(|v| format!("${:.0}", v)).collect::<Vec<_>>());
    println!("  Bubble pressure: {:.2}", tournament_holdem_state.bubble_pressure);
    
    // Test legal actions
    let legal_actions = TournamentHoldem::legal_actions(&tournament_holdem_state);
    println!("  Legal actions: {:?}", legal_actions);
    
    // Test state transitions
    if let Some(&first_action) = legal_actions.first() {
        let next_state = TournamentHoldem::next_state(&tournament_holdem_state, first_action);
        println!("  After {:?}: New state created successfully", first_action);
        
        if let Some(current_player) = TournamentHoldem::current_player(&next_state) {
            println!("    Next to act: Player {}", current_player + 1);
        } else {
            println!("    Game is terminal or chance node");
        }
    }
}

fn demo_cfr_integration() {
    println!("\n🧠 Phase 4: CFR Training Integration");
    println!("{}", "-".repeat(30));
    
    // Create minimal tournament for CFR training
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 50, big_blind: 100, ante: 0 },
        ],
        level_duration_minutes: 15,
        starting_stack: 1000,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 2, 500);
    let player_stacks = vec![1000, 1000];
    
    println!("🏗️  Creating tournament CFR trainer...");
    let mut trainer = TournamentCFRTrainer::new(tournament_state.clone(), player_stacks.clone());
    
    // Create training scenario
    let holdem_state = HoldemState::new();
    let tournament_holdem_state = TournamentHoldemState::new_tournament_hand(
        holdem_state,
        tournament_state,
        player_stacks,
    );
    
    let roots = vec![tournament_holdem_state];
    
    println!("🚀 Running minimal CFR training (1 iteration for demo)...");
    let start_time = Instant::now();
    
    // Run just 1 iteration to demonstrate functionality
    trainer.train_tournament_strategy(1, &roots);
    
    let training_time = start_time.elapsed();
    println!("✅ Training completed in {:.2?}", training_time);
    println!("📈 CFR nodes created: {}", trainer.base_trainer.nodes.len());
    
    // Demonstrate strategy evaluation
    println!("\n🎯 Tournament Strategy Evaluation:");
    let strategy = trainer.get_tournament_strategy(&roots[0], 0);
    println!("  Player 1 strategy: {:?}", strategy);
    
    if strategy.len() >= 3 {
        println!("    Fold probability: {:.1}", strategy[0] * 100.0);
        println!("    Call probability: {:.1}", strategy[1] * 100.0);  
        println!("    Raise probability: {:.1}", strategy[2] * 100.0);
    }
}

fn demo_realtime_decisions() {
    println!("\n⚡ Phase 5: Real-Time Tournament Decisions");
    println!("{}", "-".repeat(30));
    
    // Simulate late-stage tournament scenario
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 8, small_blind: 400, big_blind: 800, ante: 100 },
        ],
        level_duration_minutes: 15,
        starting_stack: 1500,
        ante_schedule: vec![],
    };
    
    let mut tournament_state = TournamentState::new(structure, 20, 5000);
    tournament_state.players_remaining = 3; // Final table!
    
    let scenarios = vec![
        ("Chip Leader vs Short Stacks", vec![12000, 3000, 1000]),
        ("Even Stacks Final 3", vec![5300, 5400, 5300]),
        ("Bubble Boy Scenario", vec![8000, 7000, 1000]),
    ];
    
    for (scenario_name, stacks) in scenarios {
        println!("\n🎪 Scenario: {}", scenario_name);
        
        let tournament_game = TournamentHoldem::new(tournament_state.clone(), stacks.clone());
        
        // Calculate decision metrics for each player
        for (player_idx, &stack) in stacks.iter().enumerate() {
            let icm_values = tournament_game.evaluator.icm_calculator.calculate_equity();
            let icm_value = icm_values.get(player_idx).unwrap_or(&0.0);
            
            // Simulate ICM pressure calculation
            let icm_pressure = tournament_game.evaluator.icm_calculator
                .calculate_icm_pressure(player_idx, 500); // Losing 500 chips
            
            let avg_stack = stacks.iter().sum::<u32>() as f64 / stacks.len() as f64;
            let stack_ratio = stack as f64 / avg_stack;
            
            println!("  Player {} (${} stack):", player_idx + 1, stack);
            println!("    ICM value: ${:.0}", icm_value);
            println!("    Stack ratio: {:.2}x average", stack_ratio);
            println!("    ICM pressure: {:.4} per chip", icm_pressure);
            
            // Recommend strategy adjustment
            let strategy_note = if stack_ratio < 0.3 {
                "🔥 HIGH RISK: Preserve chips, fold marginal hands"
            } else if stack_ratio > 1.5 {
                "💪 AGGRESSION: Apply pressure with big stack"
            } else {
                "⚖️  BALANCED: Standard tournament play"
            };
            println!("    Strategy: {}", strategy_note);
        }
    }
    
    println!("\n📊 Tournament Decision Framework:");
    println!("  ✓ ICM calculations for equity evaluation");
    println!("  ✓ Bubble pressure analysis");
    println!("  ✓ Stack-based strategy adjustments");
    println!("  ✓ CFR-trained optimal responses");
    println!("  ✓ Real-time tournament context awareness");
}

fn calculate_bubble_pressure(tournament_state: &TournamentState, _stack: u32, stack_ratio: f64) -> f64 {
    let payout_spots = tournament_state.payout_structure.len() as u32;
    let players_from_money = tournament_state.players_remaining.saturating_sub(payout_spots);
    
    // Base bubble pressure
    let base_pressure = if players_from_money <= 3 {
        1.0 - (players_from_money as f64 / 10.0)
    } else {
        0.1
    };
    
    // Stack-based adjustment
    let stack_adjustment = if stack_ratio < 0.5 {
        2.0 // Short stacks feel more pressure
    } else if stack_ratio > 1.5 {
        0.5 // Big stacks feel less pressure
    } else {
        1.0
    };
    
    (base_pressure * stack_adjustment).min(1.0).max(0.0)
}
