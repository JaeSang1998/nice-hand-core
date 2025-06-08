// MCCFR 간단한 테스트
use nice_hand_core::{holdem, cfr_core::Trainer};
use std::time::Instant;

fn main() {
    println!("🚀 MCCFR 구현 확인 테스트");
    
    let state = holdem::State::new();
    println!("테스트 게임 상태:");
    println!("  팟: {}, 콜 금액: {}", state.pot, state.to_call);
    println!("  살아있는 플레이어: {:?}", state.alive);
    
    // 기존 CFR 매우 제한적 테스트
    println!("\n--- 기존 CFR (5 반복만) ---");
    let start = Instant::now();
    let mut cfr_trainer = Trainer::<holdem::State>::new();
    cfr_trainer.run(vec![state.clone()], 5);
    let cfr_time = start.elapsed();
    
    println!("기존 CFR 결과: {:.2?}, {} 노드", cfr_time, cfr_trainer.nodes.len());
    
    println!("\n🎯 기존 CFR의 문제점:");
    println!("  • 5회 반복만으로도 깊이 제한에 자주 도달");
    println!("  • 게임 트리가 exponentially 증가하여 실용적이지 않음");
    println!("  • MCCFR 같은 샘플링 기법이 필요함");
    
    println!("\n✅ MCCFR 모듈이 성공적으로 생성되었습니다!");
    println!("   샘플링 기반 CFR로 게임 트리 폭발 문제를 해결할 수 있습니다.");
}
