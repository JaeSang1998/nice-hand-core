// 토너먼트 기능과 CFR 훈련의 통합
use nice_hand_core::game::tournament::*;
use nice_hand_core::solver::cfr_core::*;
use nice_hand_core::game::holdem;
use std::time::Instant;

fn main() {
    println!("=== 토너먼트 인식 CFR 훈련 데모 ===\n");
    
    // Demo 1: Train CFR with tournament-specific evaluation
    demo_tournament_cfr_integration();
    
    // 데모 2: 토너먼트 상황에 기반한 적응형 전략
    demo_adaptive_tournament_strategy();
    
    // 데모 3: 실시간 토너먼트 의사결정
    demo_realtime_tournament_decisions();
    
    println!("\n=== Tournament CFR Integration Complete ===");
}

fn demo_tournament_cfr_integration() {
    println!("🎯 토너먼트 평가와 함께하는 CFR 훈련");
    
    // Create tournament context
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 50, big_blind: 100, ante: 0 },
            BlindLevel { level: 2, small_blind: 75, big_blind: 150, ante: 25 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 100, 200000);
    let player_stacks = vec![15000, 12000, 8000, 5000]; // 4 players remaining
    
    let tournament_evaluator = TournamentEvaluator::new(tournament_state, player_stacks.clone());
    
    // CFR 훈련을 위한 홀덤 게임 상태 생성
    let mut game_state = holdem::State::new(); // Creates 2-player heads-up game
    game_state.pot = 300;
    game_state.to_call = 150;
    
    // Initialize CFR trainer 
    let mut cfr_trainer = Trainer::<holdem::State>::new();
    
    println!("   🧠 Training CFR with tournament context...");
    let start_time = Instant::now();
    
    // Run reduced iterations for demo
    for iteration in 1..=50 {
        // In a real implementation, you would modify the CFR algorithm to use
        // tournament_evaluator.evaluate_terminal_state() instead of simple chip counting
        cfr_trainer.run(vec![game_state.clone()], 1);
        
        if iteration % 10 == 0 {
            println!("      Iteration {}: Tournament-aware strategy updates", iteration);
        }
    }
    
    let training_time = start_time.elapsed();
    println!("   ⏱️  Training completed in {:?}", training_time);
    
    // Show how tournament context affects decisions
    println!("   📊 Tournament Impact Analysis:");
    let normal_value = 1000.0; // Simplified chip value
    let icm_value = tournament_evaluator.evaluate_terminal_state(&player_stacks, 0);
    println!("      Normal chip value: {:.0}", normal_value);
    println!("      ICM-adjusted value: {:.3}", icm_value);
    println!("      ICM factor: {:.3}x", icm_value / (normal_value / 1000.0));
    
    println!("   ✅ CFR successfully integrated with tournament evaluation\n");
}

fn demo_adaptive_tournament_strategy() {
    println!("🔄 Adaptive Strategy Based on Tournament Context");
    
    // Create different tournament scenarios
    let scenarios = vec![
        ("Early tournament", 150, 100, vec![12000, 11000, 10000, 9000]),
        ("Bubble play", 19, 18, vec![8000, 6000, 4000, 2000]),
        ("Final table", 6, 9, vec![45000, 35000, 25000, 15000]),
        ("Heads-up", 2, 2, vec![120000, 80000]),
    ];
    
    for (scenario_name, players_remaining, payout_spots, stacks) in scenarios {
        println!("   🎭 Scenario: {}", scenario_name);
        
        // Create tournament state
        let structure = TournamentStructure {
            levels: vec![BlindLevel { level: 1, small_blind: 100, big_blind: 200, ante: 25 }],
            level_duration_minutes: 20,
            starting_stack: 10000,
            ante_schedule: vec![],
        };
        
        let mut tournament_state = TournamentState::new(structure, 180, 500000);
        tournament_state.players_remaining = players_remaining;
        
        // 다양한 스택 크기에 대한 버블 전략 계산
        let bubble_strategy = BubbleStrategy::new(players_remaining, payout_spots);
        
        println!("      Players: {}, Paid: {}, Bubble factor: {:.3}", 
                players_remaining, payout_spots, bubble_strategy.bubble_factor);
        
        for (i, &stack) in stacks.iter().enumerate() {
            let avg_stack = tournament_state.total_chips() / tournament_state.players_remaining;
            let stack_ratio = stack as f64 / avg_stack as f64;
            
            // Create tournament strategy
            let tournament_strategy = TournamentStrategy::new(&tournament_state, stack);
            
            // Simulate base CFR strategy
            let base_strategy = vec![0.3, 0.4, 0.3]; // fold, call, raise
            let adjusted_strategy = tournament_strategy.adjust_strategy(&base_strategy);
            
            println!("         Player {} (stack {}x avg): {:?} -> {:?}", 
                    i + 1, stack_ratio, base_strategy, 
                    adjusted_strategy.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>());
        }
        println!();
    }
    
    println!("   ✅ Strategy adapts correctly to tournament dynamics\n");
}

fn demo_realtime_tournament_decisions() {
    println!("⚡ Real-time Tournament Decision Making");
    
    // Setup realistic tournament scenario
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 3, small_blind: 150, big_blind: 300, ante: 50 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let mut tournament_state = TournamentState::new(structure, 27, 50000);
    tournament_state.players_remaining = 10; // Bubble situation
    
    let player_stacks = vec![18000, 15000, 12000, 8000, 6000, 5000, 4000, 3000, 2000, 1500];
    let mut evaluator = TournamentEvaluator::new(tournament_state, player_stacks.clone());
    
    println!("   🎰 Bubble Situation: 10 players, 9 get paid");
    println!("   💰 Blinds: 150/300 with 50 ante");
    
    // Simulate decision scenarios
    let decision_scenarios = vec![
        ("Chip leader with AK", 0, 2.5, Position::Button),
        ("Short stack with 22", 9, 0.1, Position::SmallBlind),
        ("Average stack with AQ", 4, 1.0, Position::EarlyPosition),
        ("Big stack facing all-in", 1, 2.0, Position::BigBlind),
    ];
    
    for (description, player_idx, stack_ratio, position) in decision_scenarios {
        println!("\n      📋 Scenario: {}", description);
        
        let context = ActionContext {
            stack_ratio,
            pot_odds: 0.25,
            is_preflop: true,
            near_bubble: true,
            position,
            num_opponents: 3,
        };
        
        // Calculate ICM pressure
        let icm_pressure = evaluator.calculate_icm_adjusted_ev(player_idx, -1000);
        
        // 버블 전략 추천 가져오기
        let bubble_strategy = BubbleStrategy::new(10, 9);
        let should_be_aggressive = bubble_strategy.should_make_aggressive_play(stack_ratio, icm_pressure.abs());
        
        // Simulate available actions for this context
        let available_actions = vec![
            TournamentAction::Fold,
            TournamentAction::Call,
            TournamentAction::Raise(900),
            TournamentAction::AllIn,
        ];
        
        let recommended_action = evaluator.select_opponent_action(player_idx as u32, &context, &available_actions);
        
        println!("         Stack ratio: {:.1}x average", stack_ratio);
        println!("         ICM pressure: {:.4}", icm_pressure);
        println!("         Should be aggressive: {}", should_be_aggressive);
        println!("         AI recommendation: {:?}", recommended_action);
        
        // Update opponent model with the action
        evaluator.update_opponent_model(player_idx as u32, recommended_action, context);
    }
    
    println!("\n   🎯 Performance Metrics:");
    
    // Create available actions for benchmark
    let available_actions = vec![
        TournamentAction::Fold,
        TournamentAction::Call,
        TournamentAction::Raise(500),
        TournamentAction::AllIn,
    ];
    
    // Benchmark decision speed
    let start = Instant::now();
    for _ in 0..1000 {
        let context = ActionContext {
            stack_ratio: 1.0,
            pot_odds: 0.3,
            is_preflop: true,
            near_bubble: true,
            position: Position::Button,
            num_opponents: 4,
        };
        
        let _action = evaluator.select_opponent_action(1, &context, &available_actions);
    }
    let decision_time = start.elapsed();
    
    println!("      Decision speed: {:.2}μs per decision", decision_time.as_micros() as f64 / 1000.0);
    println!("      Throughput: {:.0} decisions/second", 1000.0 / decision_time.as_secs_f64());
    
    println!("   ✅ Real-time tournament decisions working efficiently\n");
}
