use nice_hand_core::{holdem, Trainer};

fn main() {
    println!("Nice Hand Core - 텍사스 홀덤용 선호도 CFR 구현체");

    // 텍사스 홀덤 CFR 테스트
    println!("\n=== 텍사스 홀덤 CFR 테스트 ===");
    let mut holdem_trainer = Trainer::<holdem::State>::new();

    // 테스트용 홀덤 상태 생성
    let initial_state = holdem::State {
        hole: [[0, 1], [2, 3], [4, 5], [6, 7], [8, 9], [10, 11]],
        board: vec![],
        to_act: 0,
        street: 0,
        pot: 100,
        stack: [1000; 6],
        alive: [true, true, false, false, false, false], // 2명의 플레이어
        invested: [15, 30, 0, 0, 0, 0],                  // 블라인드 투입됨
        to_call: 30,
        actions_taken: 0,
    };

    println!("{}번 반복으로 텍사스 홀덤 학습 중...", 100);
    let start_time = std::time::Instant::now();

    holdem_trainer.run(vec![initial_state], 100);

    let elapsed = start_time.elapsed();
    println!(
        "홀덤 학습이 {:?}에 완료되었습니다! 노드 수: {}",
        elapsed,
        holdem_trainer.nodes.len()
    );

    // 일부 전략 결과 표시
    for (info_key, node) in holdem_trainer.nodes.iter().take(3) {
        let avg_strategy = node.average();
        println!("정보키 {}: 전략 {:?}", info_key, avg_strategy);
    }

    println!("\n=== CFR 구현이 텍사스 홀덤에 성공적으로 적용되었습니다! ===");
}
