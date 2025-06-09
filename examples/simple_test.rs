use nice_hand_core::{Trainer, holdem};

fn main() {
    println!("간단한 CFR 테스트 - 홀덤 테스팅");
    
    // 홀덤 CFR 테스트 (무한 재귀 수정 테스트)
    println!("\n=== 홀덤 CFR 테스트 (무한 재귀 수정 테스트) ===");
    let mut holdem_trainer = Trainer::<holdem::State>::new();
    
    // 올바른 구조를 사용하여 간단한 2플레이어 홀덤 상태 생성
    let initial_state = holdem::State {
        hole: [[0, 1], [2, 3], [4, 5], [6, 7], [8, 9], [10, 11]],
        board: vec![],
        to_act: 0,
        street: 0,
        pot: 100,
        stack: [1000; 6],
        alive: [true, true, false, false, false, false], // 2명의 플레이어만
        invested: [15, 30, 0, 0, 0, 0], // 블라인드 게시
        to_call: 30,
        actions_taken: 0,
    };
    
    println!("50회 반복으로 홀덤 훈련 (무한 재귀 테스트)...");
    let start_time = std::time::Instant::now();
    
    holdem_trainer.run(vec![initial_state], 50);
    
    let elapsed = start_time.elapsed();
    println!("홀덤 훈련이 {:?}에 완료되었습니다! 생성된 노드: {}", elapsed, holdem_trainer.nodes.len());
    
    if holdem_trainer.nodes.len() > 0 {
        println!("✅ 홀덤 CFR 훈련 성공 - 무한 재귀 감지되지 않음!");
        
        // 몇 가지 예제 전략 보기 (처음 몇 개 노드)
        for (info_key, node) in holdem_trainer.nodes.iter().take(3) {
            let avg_strategy = node.average();
            println!("InfoKey {}: 전략 {:?}", info_key, avg_strategy);
        }
    } else {
        println!("❌ 노드가 생성되지 않음 - 아직 문제가 있을 수 있음");
    }
}
