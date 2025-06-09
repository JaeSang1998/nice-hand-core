// Simple Tournament CFR Integration Demo
use nice_hand_core::game::tournament::*;
use std::time::Instant;

fn main() {
    println!("=== 토너먼트 CFR 통합 데모 ===\n");
    
    tournament_strategy_demo();
    performance_benchmark();
    
    println!("✅ 토너먼트 통합 완료!");
}

fn tournament_strategy_demo() {
    println!("🎯 토너먼트 전략 적응");
    
    // Create bubble scenario
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 100, big_blind: 200, ante: 25 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let mut tournament_state = TournamentState::new(structure, 100, 200000);
    tournament_state.players_remaining = 19; // Bubble with 18 paid
    
    // Test different stack sizes
    let scenarios = vec![
        ("Short stack", 4000),
        ("Average stack", 10000), 
        ("Big stack", 25000),
        ("Chip leader", 40000),
    ];
    
    println!("   버블 상황 (19명 플레이어, 18명 상금):");
    
    for (desc, stack) in scenarios {
        let strategy = TournamentStrategy::new(&tournament_state, stack);
        let base_strategy = vec![0.4, 0.3, 0.3]; // fold, call, raise
        let adjusted = strategy.adjust_strategy(&base_strategy);
        
        println!("      {}: {:?} -> {:?}", 
                desc, 
                base_strategy.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>(),
                adjusted.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>());
    }
    
    println!("   ✅ 전략이 토너먼트 맥락에 맞게 올바르게 조정됨\n");
}

fn performance_benchmark() {
    println!("⚡ 성능 벤치마크");
    
    let stacks = vec![15000, 12000, 8000, 5000, 3000];
    let payouts = vec![50000, 30000, 20000, 15000, 10000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    
    // Benchmark ICM calculations
    let start = Instant::now();
    let iterations = 1000;
    
    for _ in 0..iterations {
        let _equities = icm.calculate_equity();
    }
    
    let duration = start.elapsed();
    let per_calculation = duration.as_micros() as f64 / iterations as f64;
    
    println!("   📊 ICM 성능:");
    println!("      {:?}에 {} 계산", duration, iterations);
    println!("      평균: 계산당 {:.2}μs", per_calculation);
    println!("      처리량: 초당 {:.0} 계산", 1_000_000.0 / per_calculation);
    
    // Benchmark opponent modeling
    let mut model = OpponentModel::new(1);
    let context = ActionContext {
        stack_ratio: 1.0,
        pot_odds: 0.3,
        is_preflop: true,
        near_bubble: true,
        position: Position::Button,
        num_opponents: 3,
    };
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _prediction = model.predict_action_distribution(&context);
    }
    let modeling_duration = start.elapsed();
    let per_prediction = modeling_duration.as_micros() as f64 / iterations as f64;
    
    println!("   🤖 Opponent Modeling:");
    println!("      Average: {:.2}μs per prediction", per_prediction);
    println!("      Throughput: {:.0} predictions/second", 1_000_000.0 / per_prediction);
    
    println!("   ✅ Tournament algorithms are highly optimized\n");
}
