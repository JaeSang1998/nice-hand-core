// 고급 휴리스틱 전략 데모
use nice_hand_core::web_api_simple::{QuickPokerAPI, WebGameState};

fn main() {
    println!("🃏 고급 포커 휴리스틱 전략 데모");
    println!("=========================================");
    
    let api = QuickPokerAPI::new();
    
    // 테스트 시나리오 1: 프리미엄 프리플롭 핸드
    println!("\n📋 시나리오 1: 프리미엄 프리플롭 핸드 (AA)");
    println!("-{}", "-".repeat(49));
    
    let premium_state = WebGameState {
        hole_cards: [0, 13], // AA (스페이드 에이스, 하트 에이스)
        board: vec![],
        street: 0,
        pot: 150,
        to_call: 100,
        my_stack: 1000,
        opponent_stack: 1000,
    };
    
    let response = api.get_optimal_strategy(premium_state);
    println!("🎯 권장 액션: {}", response.recommended_action);
    println!("💪 핸드 강도: {:.1}%", response.hand_strength * 100.0);
    println!("📊 기댓값: {:.1} 칩", response.expected_value);
    println!("🧠 추론: {}", response.reasoning);
    
    // 테스트 시나리오 2: 경계선 콜링 핸드
    println!("\n📋 시나리오 2: 경계선 콜링 핸드 (KQ 오프수트)");
    println!("-{}", "-".repeat(49));
    
    let marginal_state = WebGameState {
        hole_cards: [11, 23], // KQ 오프수트
        board: vec![],
        street: 0,
        pot: 200,
        to_call: 150,
        my_stack: 800,
        opponent_stack: 800,
    };
    
    let response2 = api.get_optimal_strategy(marginal_state);
    println!("🎯 권장 액션: {}", response2.recommended_action);
    println!("💪 핸드 강도: {:.1}%", response2.hand_strength * 100.0);
    println!("📊 기댓값: {:.1} 칩", response2.expected_value);
    println!("🧠 추론: {}", response2.reasoning);
    
    // 테스트 시나리오 3: 강한 포스트플롭 핸드 (탑 페어)
    println!("\n📋 시나리오 3: 강한 포스트플롭 핸드 (탑 페어)");
    println!("-{}", "-".repeat(49));
    
    let postflop_state = WebGameState {
        hole_cards: [0, 26], // A♠ K♠
        board: vec![1, 21, 34], // A♥ 9♠ J♥ - 훌륭한 키커를 가진 탑 페어
        street: 1,
        pot: 300,
        to_call: 0, // 우리에게 체크
        my_stack: 700,
        opponent_stack: 700,
    };
    
    let response3 = api.get_optimal_strategy(postflop_state);
    println!("🎯 권장 액션: {}", response3.recommended_action);
    println!("💪 핸드 강도: {:.1}%", response3.hand_strength * 100.0);
    println!("📊 기댓값: {:.1} 칩", response3.expected_value);
    println!("🧠 추론: {}", response3.reasoning);
    
    // 성능 테스트
    println!("\n📊 성능 분석");
    println!("-{}", "-".repeat(49));
    
    let start = std::time::Instant::now();
    let test_states: Vec<WebGameState> = (0u32..1000u32).map(|i| {
        WebGameState {
            hole_cards: [(i % 52) as u8, ((i + 13) % 52) as u8],
            board: if i % 3 == 0 { vec![] } else { vec![(i % 52) as u8, ((i + 1) % 52) as u8, ((i + 2) % 52) as u8] },
            street: if i % 3 == 0 { 0 } else { 1 },
            pot: 100 + (i % 500),
            to_call: i % 200,
            my_stack: 1000,
            opponent_stack: 1000,
        }
    }).collect();
    
    let _responses = api.get_strategies_batch(test_states);
    let duration = start.elapsed();
    
    println!("✅ 1,000개 결정을 {:?}에 처리", duration);
    println!("⚡ 평균: 결정당 {:.2}μs", duration.as_micros() as f64 / 1000.0);
    
    println!("\n🎯 휴리스틱 향상 완료!");
    println!("   ✓ 정교한 핸드 평가");
    println!("   ✓ 고급 베팅 전략");
    println!("   ✓ 맥락 인식 의사결정");
    println!("   ✓ 운영 준비 성능");
}
