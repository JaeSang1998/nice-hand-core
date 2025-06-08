// MCCFR 테스트
use nice_hand_core::{HoldemState, mccfr::MCCFRTrainer, cfr_core::Trainer};
use std::time::Instant;

fn main() {
    test_mccfr_basic();
    test_mccfr_performance();
    test_mccfr_sampling_rates();
}

fn test_mccfr_basic() {
    println!("🎲 Monte Carlo CFR 기본 테스트");
    
    let state = HoldemState::new();
    println!("테스트 상태: pot={}, to_call={}, alive={:?}", 
             state.pot, state.to_call, state.alive);
    
    // MCCFR 테스트 (50% 샘플링)
    println!("\n--- Monte Carlo CFR (50% 샘플링) ---");
    let start = Instant::now();
    let mut mccfr_trainer: MCCFRTrainer<HoldemState> = MCCFRTrainer::new(0.5);
    mccfr_trainer.run(vec![state.clone()], 200);
    let mccfr_time = start.elapsed();
    
    println!("MCCFR 결과:");
    println!("  학습 시간: {:.2?}", mccfr_time);
    println!("  생성된 노드: {}", mccfr_trainer.nodes.len());
    
    // 노드가 생성되었는지 확인
    assert!(!mccfr_trainer.nodes.is_empty(), "MCCFR should create nodes");
    
    // 전략 샘플링 테스트
    if !mccfr_trainer.nodes.is_empty() {
        println!("\n--- MCCFR 전략 샘플 ---");
        let sample_key = mccfr_trainer.nodes.keys().next().unwrap();
        let node = mccfr_trainer.nodes.get(sample_key).unwrap();
        let strategy = node.average();
        println!("  샘플 노드 전략: {:?}", strategy);
        
        // 전략 확률의 합이 1에 가까운지 확인
        let sum: f64 = strategy.iter().sum();
        println!("전략 확률의 합: {}", sum);
    }
}

fn test_mccfr_performance() {
    println!("🎯 MCCFR vs CFR 제한적 비교");
    
    let state = HoldemState::new();
    
    // MCCFR 테스트 (30% 샘플링, 적은 반복)
    println!("\n--- MCCFR (30% 샘플링, 50 반복) ---");
    let start = Instant::now();
    let mut mccfr_trainer: MCCFRTrainer<HoldemState> = MCCFRTrainer::new(0.3);
    mccfr_trainer.run(vec![state.clone()], 50);
    let mccfr_time = start.elapsed();
    
    println!("MCCFR: {:.2?}, {} 노드", mccfr_time, mccfr_trainer.nodes.len());
    
    // 기존 CFR (매우 적은 반복)
    println!("\n--- 기존 CFR (5 반복) ---");
    let start = Instant::now();
    let mut cfr_trainer: Trainer<HoldemState> = Trainer::new();
    cfr_trainer.run(vec![state.clone()], 5);
    let cfr_time = start.elapsed();
    
    println!("CFR: {:.2?}, {} 노드", cfr_time, cfr_trainer.nodes.len());
    
    // 결과 비교
    println!("\n--- 효율성 비교 ---");
    let mccfr_efficiency = mccfr_trainer.nodes.len() as f64 / mccfr_time.as_secs_f64();
    let cfr_efficiency = cfr_trainer.nodes.len() as f64 / cfr_time.as_secs_f64();
    
    println!("  MCCFR 효율성: {:.0} 노드/초", mccfr_efficiency);
    println!("  CFR 효율성: {:.0} 노드/초", cfr_efficiency);
    
    if mccfr_efficiency > cfr_efficiency {
        println!("  ✅ MCCFR이 {:.1}배 더 효율적", mccfr_efficiency / cfr_efficiency);
    } else {
        println!("  ⚠️  CFR이 {:.1}배 더 효율적", cfr_efficiency / mccfr_efficiency);
    }
}

fn test_mccfr_sampling_rates() {
    println!("📊 MCCFR 샘플링 비율 테스트");
    
    let state = HoldemState::new();
    let iterations = 30;
    
    for &sample_rate in &[0.1, 0.3, 0.5, 0.8, 1.0] {
        println!("\n--- {}% 샘플링 ---", (sample_rate * 100.0) as u32);
        
        let start = Instant::now();
        let mut trainer: MCCFRTrainer<HoldemState> = MCCFRTrainer::new(sample_rate);
        trainer.run(vec![state.clone()], iterations);
        let elapsed = start.elapsed();
        
        println!("  시간: {:.2?}, 노드: {}", elapsed, trainer.nodes.len());
        
        if !trainer.nodes.is_empty() {
            let sample_key = trainer.nodes.keys().next().unwrap();
            let node = trainer.nodes.get(sample_key).unwrap();
            let strategy = node.average();
            println!("  전략 샘플: [{:.3}, {:.3}, ...]", 
                     strategy.get(0).unwrap_or(&0.0), 
                     strategy.get(1).unwrap_or(&0.0));
        }
    }
}
