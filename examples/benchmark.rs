use nice_hand_core::{Trainer, holdem};
use std::time::Instant;

fn main() {
    println!("🎯 Nice Hand Core - 성능 벤치마크");
    println!("==========================================");
    
    // 벤치마킹을 위한 홀덤 상태 생성
    let initial_state = holdem::State {
        hole: [[0, 1], [2, 3], [4, 5], [6, 7], [8, 9], [10, 11]],
        board: vec![],
        to_act: 0,
        street: 0,
        pot: 100,
        stack: [1000; 6],
        alive: [true, true, false, false, false, false], // 2명의 플레이어
        invested: [15, 30, 0, 0, 0, 0], // 블라인드 게시
        to_call: 30,
        actions_taken: 0,
    };
    
    let iterations = [10, 50, 100, 250];
    
    for &iters in &iterations {
        print!("{}회 반복으로 홀덤 훈련 중... ", iters);
        
        let mut trainer = Trainer::<holdem::State>::new();
        let start = Instant::now();
        
        trainer.run(vec![initial_state.clone()], iters);
        
        let duration = start.elapsed();
        let nodes = trainer.nodes.len();
        
        println!("✅ {}ms ({} 노드)", duration.as_millis(), nodes);
        
        if iters == 250 {
            println!("\n📊 전략 수렴 결과:");
            for (i, (info_key, node)) in trainer.nodes.iter().enumerate().take(3) {
                let avg_strategy = node.average();
                println!("  노드 {}: InfoKey {} → 전략 {:?}", 
                    i + 1, info_key, 
                    avg_strategy.iter().map(|x| format!("{:.3}", x)).collect::<Vec<_>>()
                );
            }
        }
    }
    
    println!("\n🚀 멀티스레드 성능:");
    println!("   - rayon을 사용한 병렬 CFR 탐색");
    println!("   - 확장성을 위한 스레드 로컬 탐색");
    
    println!("\n💡 Architecture Benefits:");
    println!("   ✓ Generic Game trait for multiple poker variants");
    println!("   ✓ Multi-platform support (WASM + Native)");
    println!("   ✓ Memory-efficient hash-based node storage");
    
    println!("\n🎮 Ready for web and desktop deployment!");
}
