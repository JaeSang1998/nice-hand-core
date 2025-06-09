use nice_hand_core::{Trainer, holdem};

fn main() {
    println!("홀덤 CFR 테스트 - 무한 재귀 수정 테스트");
    
    let mut trainer = Trainer::<holdem::State>::new();
    
    // 간단한 2플레이어 홀덤 상태 생성
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
    
    println!("100회 반복으로 훈련 (무한 재귀 테스트)...");
    let start_time = std::time::Instant::now();
    
    trainer.run(vec![initial_state], 100);
    
    let elapsed = start_time.elapsed();
    println!("훈련이 {:?}에 완료되었습니다! 생성된 노드: {}", elapsed, trainer.nodes.len());
    
    if trainer.nodes.len() > 0 {
        println!("✅ 홀덤 CFR 훈련 성공 - 무한 재귀 감지되지 않음!");
        
        // Show some example strategies (first few nodes)
        for (info_key, node) in trainer.nodes.iter().take(5) {
            let avg_strategy = node.average();
            println!("정보키 {}: 전략 {:?}", info_key, avg_strategy);
        }
    } else {
        println!("❌ No nodes created - there may still be an issue");
    }
}
