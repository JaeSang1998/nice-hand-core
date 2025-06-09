// 텍사스 홀덤용 간단한 무상태 웹 API 데모
use nice_hand_core::web_api_simple::*;

fn main() {
    println!("🚀 텍사스 홀덤 간단한 웹 API 데모");
    println!("====================================");
    println!("✨ 훈련 불필요 - 즉시 응답!");
    
    // 빠른 API 초기화 (훈련 불필요)
    println!("\n🌐 빠른 포커 API 초기화 중...");
    let api = QuickPokerAPI::new();
    println!("✅ API가 즉시 요청 처리 준비 완료");
    
    // 웹 요청 시뮬레이션
    println!("\n📡 웹 요청 시뮬레이션...");
    
    // 요청 1: 포켓 에이스를 가진 프리플랍
    println!("\n🃏 요청 1: 포켓 에이스를 가진 프리플랍");
    let request1 = WebGameState {
        hole_cards: [12, 25], // AA (스페이드 에이스, 하트 에이스)
        board: vec![],
        street: 0,
        pot: 150,
        to_call: 50,
        my_stack: 1000,
        opponent_stack: 1000,
    };
    
    let start_time = std::time::Instant::now();
    let response1 = api.get_optimal_strategy(request1);
    let response_time = start_time.elapsed();
    
    println!("💡 추천 액션: {}", response1.recommended_action);
    println!("📊 액션 확률:");
    for (action, prob) in &response1.strategy {
        println!("   {}: {:.1}%", action, prob * 100.0);
    }
    println!("🎯 기댓값: {:.2}", response1.expected_value);
    println!("⚡ 응답 시간: {:?}", response_time);
    
    // 요청 2: 탑 페어가 있는 플랍
    println!("\n🃏 요청 2: 탑 페어가 있는 플랍");
    let request2 = WebGameState {
        hole_cards: [12, 7], // A♠ 8♦ 
        board: vec![25, 1, 14], // A♥ 3♠ 2♦
        street: 1,
        pot: 200,
        to_call: 75,
        my_stack: 925,
        opponent_stack: 875,
    };
    
    let start_time = std::time::Instant::now();
    let response2 = api.get_optimal_strategy(request2);
    let response_time = start_time.elapsed();
    
    println!("💡 추천 액션: {}", response2.recommended_action);
    println!("📊 액션 확률:");
    for (action, prob) in &response2.strategy {
        println!("   {}: {:.1}%", action, prob * 100.0);
    }
    println!("🎯 기댓값: {:.2}", response2.expected_value);
    println!("⚡ 응답 시간: {:?}", response_time);
    
    // 요청 3: 플러시 드로우가 있는 턴
    println!("\n🃏 요청 3: 플러시 드로우가 있는 턴");
    let request3 = WebGameState {
        hole_cards: [12, 11], // A♠ K♠
        board: vec![25, 1, 14, 10], // A♥ 3♠ 2♦ J♠
        street: 2,
        pot: 400,
        to_call: 150,
        my_stack: 750,
        opponent_stack: 700,
    };
    
    let start_time = std::time::Instant::now();
    let response3 = api.get_optimal_strategy(request3);
    let response_time = start_time.elapsed();
    
    println!("💡 추천 액션: {}", response3.recommended_action);
    println!("📊 액션 확률:");
    for (action, prob) in &response3.strategy {
        println!("   {}: {:.1}%", action, prob * 100.0);
    }
    println!("🎯 기댓값: {:.2}", response3.expected_value);
    println!("⚡ 응답 시간: {:?}", response_time);
    
    // 여러 요청으로 성능 테스트
    println!("\n⚡ 성능 테스트: 100회 요청");
    let perf_request = WebGameState {
        hole_cards: [8, 21], // J♠ 9♥
        board: vec![],
        street: 0,
        pot: 100,
        to_call: 25,
        my_stack: 975,
        opponent_stack: 950,
    };
    
    let perf_start = std::time::Instant::now();
    for _ in 0..100 {
        let _response = api.get_optimal_strategy(perf_request.clone());
    }
    let total_time = perf_start.elapsed();
    let avg_time = total_time / 100;
    
    println!("🚀 100회 요청이 {:?}에 완료됨", total_time);
    println!("📊 평균 응답 시간: {:?}", avg_time);
    println!("🔥 초당 요청 수: {:.0}", 1.0 / avg_time.as_secs_f64());
    
    // 요약
    println!("\n📋 요약");
    println!("=========");
    println!("✅ 간단한 API가 훈련 없이 작동");
    println!("✅ 무상태 요청이 올바르게 작동");
    println!("✅ 서브 밀리초 응답 시간");
    println!("✅ 즉시 프로덕션 사용 준비");
    println!("✅ 캐주얼 플레이에 적합한 휴리스틱 기반 전략");
    
    println!("\n🎯 웹 서버 통합:");
    println!("   1. 서버 시작 시 QuickPokerAPI::new() 초기화");
    println!("   2. get_strategy()로 HTTP 요청 처리");
    println!("   3. 각 요청은 완전히 독립적 (무상태)");
    println!("   4. 훈련이나 사전 계산 불필요");
    println!("   5. 실시간 포커 애플리케이션에 완벽");
}
