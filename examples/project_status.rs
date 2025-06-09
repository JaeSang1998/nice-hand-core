use nice_hand_core::*;
use std::time::Instant;

fn main() {
    println!("🔍 Nice Hand Core - 프로젝트 상태 및 다음 단계");
    println!("===============================================");
    
    current_capabilities_demo();
    development_priorities();
}

fn current_capabilities_demo() {
    println!("\n✅ 현재 작동하는 기능:");
    println!("────────────────────────────");
    
    // CFR 훈련 데모
    let start = Instant::now();
    let trainer = api::web_api::OfflineTrainer::train_simple_strategy(10);
    let cfr_time = start.elapsed();
    
    println!("🧠 CFR 훈련: {} 노드, {:?} 소요", trainer.nodes.len(), cfr_time);
    
    // Web API 데모
    let start = Instant::now();
    let api = api::web_api_simple::QuickPokerAPI::new();
    let init_time = start.elapsed();
    
    let state = api::web_api_simple::WebGameState {
        hole_cards: [52, 53], // As, Ah (예시 값)
        board: vec![12, 25, 38], // Kh, Qd, Jc (예시 값)
        street: 1, // 플랍
        pot: 100,
        to_call: 50,
        my_stack: 1000,
        opponent_stack: 1000,
    };
    
    let result = api.get_optimal_strategy(state.clone());
    println!("🌐 Web API: {:?}에 초기화, 액션: {}", init_time, result.recommended_action);
    
    // 성능 테스트
    let start = Instant::now();
    for _ in 0..100 {
        let _ = api.get_optimal_strategy(state.clone());
    }
    let perf_time = start.elapsed();
    
    println!("⚡ 성능: {:?}에 100개 결정 (평균 {:.2}μs)", 
             perf_time, perf_time.as_micros() as f64 / 100.0);
}

fn development_priorities() {
    println!("\n🚀 다음 개발 우선순위:");
    println!("───────────────────────────────");
    
    println!("🏆 1. 토너먼트 지원 (1-2주)");
    println!("   • 토너먼트 모듈 컴파일 수정");
    println!("   • 에퀴티를 위한 ICM 계산");
    println!("   • 블라인드 구조 관리");
    println!("   • 버블 전략 조정");
    
    println!("\n🧠 2. 고급 AI (2-3주)");
    println!("   • 상대방 모델링");
    println!("   • 레인지 분석");
    println!("   • 익스플로잇 전략");
    println!("   • 메타게임 적응");
    
    println!("\n🌐 3. 웹 통합 (2-3주)");
    println!("   • WASM 브라우저 지원");
    println!("   • WebSocket 멀티플레이어");
    println!("   • 데이터베이스 통합");
    println!("   • React/Vue 컴포넌트");
    
    println!("\n📊 4. 분석 및 도구 (1-2주)");
    println!("   • 실시간 HUD");
    println!("   • 세션 분석");
    println!("   • 핸드 히스토리 추적");
    println!("   • 성능 프로파일링");
    
    println!("\n🎯 즉시 해야 할 작업 (이번 주):");
    println!("   1. 토너먼트 모듈 익스포트 수정");
    println!("   2. 포괄적인 문서 추가");
    println!("   3. 테스트 커버리지 확장");
    println!("   4. 성능 벤치마크 생성");
    println!("   5. 에러 핸들링 구현");
    
    println!("\n💡 다음에 구현할 우선순위를 선택하세요!");
    println!("   라이브러리 기반이 견고하고 확장할 준비가 되었습니다.");
    println!("   어떤 영역을 먼저 개발하고 싶으신가요?");
}
