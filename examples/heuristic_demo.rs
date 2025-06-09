// 고급 휴리스틱 전략 시연
// 상세한 분석을 통해 정교한 포커 로직 실행을 보여줍니다

use nice_hand_core::web_api_simple::{QuickPokerAPI, WebGameState};

fn main() {
    println!("🃏 고급 포커 휴리스틱 전략 데모");
    println!("=========================================");
    
    let api = QuickPokerAPI::new();
    
    // 테스트 시나리오 1: 프리미엄 프리플롭 핸드
    println!("\n📋 시나리오 1: 프리미엄 프리플롭 핸드 (AA)");
    println!("{}", "-".repeat(50));
    
    let premium_state = WebGameState {
        hole_cards: [0, 13], // AA (스페이드 에이스, 하트 에이스)
        board: vec![],
        street: 0,
        pot: 150,
        to_call: 100,
        my_stack: 1000,
        opponent_stack: 1000,
    };
    
    demonstrate_strategy(&api, premium_state, "포켓 에이스 프리플롭에서 레이즈에 직면");
    
    // 테스트 시나리오 2: 경계선 콜링 핸드
    println!("\n📋 시나리오 2: 경계선 콜링 핸드 (KQ 오프수트)");
    println!("{}", "-".repeat(50));
    
        let marginal_state = WebGameState {
        hole_cards: [11, 23], // KQ 오프수트
        board: vec![],
        street: 0,
        pot: 200,
        to_call: 150,
        my_stack: 800,
        opponent_stack: 800,
    };
    
    demonstrate_strategy(&api, marginal_state, "KQ 오프수트에서 큰 프리플롭 레이즈에 직면");
    
    // 테스트 시나리오 3: 강한 포스트플롭 핸드 (탑 페어)
    println!("\n📋 시나리오 3: 강한 포스트플롭 핸드 (탑 페어)");
    println!("{}", "-".repeat(50));
    
    let postflop_state = WebGameState {
        hole_cards: [0, 26], // A♠ K♠
        board: vec![1, 21, 34], // A♥ 9♠ J♥ - 훌륭한 키커를 가진 탑 페어
        street: 1,
        pot: 300,
        to_call: 0, // 우리에게 체크
        my_stack: 700,
        opponent_stack: 700,
    };
    
    demonstrate_strategy(&api, postflop_state, "플롭에서 킹 키커를 가진 에이스 탑 페어");
    
    // 테스트 시나리오 4: 플러시 드로우
    println!("\n📋 시나리오 4: 플러시 드로우 (세미 블러프 스팟)");
    println!("{}", "-".repeat(50));
    
    let flush_draw_state = WebGameState {
        hole_cards: [26, 39], // K♠ Q♠
        board: vec![7, 20, 33], // 8♠ 8♥ 9♠ - 플러시 드로우 + 스트레이트 드로우
        street: 1,
        pot: 400,
        to_call: 200,
        my_stack: 600,
        opponent_stack: 600,
    };
    
    demonstrate_strategy(&api, flush_draw_state, "연결된 보드에서 베팅에 직면한 플러시 드로우");
    
    // 테스트 시나리오 5: 블러프 스팟에서의 약한 핸드
    println!("\n📋 시나리오 5: 약한 핸드 블러프 스팟");
    println!("{}", "-".repeat(50));
    
    let bluff_state = WebGameState {
        hole_cards: [4, 17], // 5♠ 6♥
                board: vec![48, 49, 50], // K♠ Q♠ J♠ - 완전히 빗나감
        street: 1,
        pot: 250,
        to_call: 0,
        my_stack: 750,
        opponent_stack: 750,
    };
    
    demonstrate_strategy(&api, bluff_state, "높은 연결 보드에서의 완전한 에어");
    
    // 테스트 시나리오 6: 숏 스택 올인 상황
    println!("\n📋 시나리오 6: 숏 스택 올인 결정");
    println!("{}", "-".repeat(50));
    
    let short_stack_state = WebGameState {
        hole_cards: [32, 45], // 7♠ 7♥ 
        board: vec![],
        street: 0,
        pot: 400,
        to_call: 180, // 우리 스택의 거의 절반
        my_stack: 400,
        opponent_stack: 800,
    };
    
    demonstrate_strategy(&api, short_stack_state, "포켓 7s 숏 스택에서 큰 레이즈에 직면");
    
    // 성능 테스트
    println!("\n📊 성능 분석");
    println!("{}", "-".repeat(50));
    
    let start = std::time::Instant::now();
    let test_states: Vec<WebGameState> = (0u32..1000u32).map(|i| {
        WebGameState {
            hole_cards: [(i % 52) as u8, ((i + 13) % 52) as u8],
            board: if i % 3 == 0 { vec![] } else { vec![(i % 52) as u8, ((i + 1) % 52) as u8, ((i + 2) % 52) as u8] },
            street: if i % 3 == 0 { 0 } else { 1 },
            pot: 100 + (i % 500) as u32,
            to_call: (i % 200) as u32,
            my_stack: 1000,
            opponent_stack: 1000,
        }
    }).collect();
    
    let responses = api.get_strategies_batch(test_states);
    let duration = start.elapsed();
    
    println!("✅ 1,000개 결정을 {:?}에 처리", duration);
    println!("⚡ 평균: 결정당 {:.2}μs", duration.as_micros() as f64 / 1000.0);
    
    // 액션 분포 분석
    let mut action_counts = std::collections::HashMap::new();
    for response in &responses {
        *action_counts.entry(response.recommended_action.clone()).or_insert(0) += 1;
    }
    
    println!("\n📈 액션 분포 (1,000개 랜덤 시나리오):");
    for (action, count) in action_counts {
        println!("  {} {}: {}% ({}개 결정)", 
                 get_action_emoji(&action), action, 
                 (count as f64 / 10.0), count);
    }
    
    println!("\n🎯 휴리스틱 향상 완료!");
    println!("   ✓ 정교한 핸드 평가");
    println!("   ✓ 고급 베팅 전략");
    println!("   ✓ 맥락 인식 의사결정");
    println!("   ✓ 운영 준비 성능");
}

fn demonstrate_strategy(api: &QuickPokerAPI, state: WebGameState, description: &str) {
    println!("📝 상황: {}", description);
    
    let response = api.get_optimal_strategy(state.clone());
    
    println!("🎯 권장 액션: {} {}", 
             get_action_emoji(&response.recommended_action), 
             response.recommended_action);
    println!("💪 핸드 강도: {:.1}%", response.hand_strength * 100.0);
    println!("📊 기댓값: {:.1} 칩", response.expected_value);
    println!("🎲 신뢰도: {:.1}%", response.confidence * 100.0);
    
    if state.to_call > 0 {
        println!("💰 팟 오즈: {:.1}%", response.pot_odds * 100.0);
    }
    
    println!("🧠 추론: {}", response.reasoning);
    
    println!("📈 전략 분포:");
    let mut sorted_strategy: Vec<_> = response.strategy.iter().collect();
    sorted_strategy.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    
    for (action, probability) in sorted_strategy {
        if *probability > 0.01 { // 1% 이상 확률인 액션만 표시
            println!("   {} {}: {:.1}%", 
                     get_action_emoji(action), action, probability * 100.0);
        }
    }
}

fn get_action_emoji(action: &str) -> &'static str {
    match action {
        "fold" => "🛑",
        "check" => "✋",
        "call" => "📞",
        "bet_small" => "💰",
        "bet_large" => "💎",
        "raise" => "🚀",
        _ => "❓",
    }
}
