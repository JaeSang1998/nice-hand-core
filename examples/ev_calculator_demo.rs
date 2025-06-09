// EV 계산기 데모 - 특정 스트리트에서 액션들의 Expected Value 계산 예제
use nice_hand_core::game::holdem::State;
use nice_hand_core::solver::ev_calculator::{EVCalculator, EVConfig};
use nice_hand_core::game::card_abstraction::hand_strength;

fn main() {
    println!("🎯 기댓값 계산기 데모");
    println!("=================================");
    
    // 테스트 시나리오 1: 프리플랍에서 강한 핸드 (Pocket Aces)
    println!("\n📋 시나리오 1: 프리플랍 - Pocket Aces");
    test_preflop_pocket_aces();
    
    // 테스트 시나리오 2: 플랍에서 탑 페어
    println!("\n📋 시나리오 2: 플랍 - 탑 페어");
    test_flop_top_pair();
    
    // 테스트 시나리오 3: 턴에서 드로우 상황
    println!("\n📋 시나리오 3: 턴 - 드로우 상황");
    test_turn_draw();
    
    // 테스트 시나리오 4: 리버에서 블러프 캐처
    println!("\n📋 시나리오 4: 리버 - 블러프 캐처");
    test_river_bluff_catcher();
}

/// 프리플랍에서 Pocket Aces의 EV 계산
fn test_preflop_pocket_aces() {
    // Pocket Aces 설정 (A♠ A♥)
    let mut state = State::new();
    state.hole[0] = [0, 13]; // A♠, A♥
    state.street = 0; // 프리플랍
    state.pot = 30; // Small blind + Big blind
    state.to_call = 20; // Big blind
    
    println!("홀카드: A♠ A♥ (Pocket Aces)");
    println!("스트리트: 프리플랍");
    println!("팟: {}칩, 콜: {}칩", state.pot, state.to_call);
    
    // 핸드 스트렝스 계산
    let hand_strength = hand_strength(state.hole[0], &[]);
    println!("핸드 스트렝스: {:.3}", hand_strength);
    
    // EV 계산 설정
    let config = EVConfig {
        sample_count: 5000,  // 빠른 데모를 위해 샘플 수 감소
        max_depth: 8,
        use_opponent_model: true,
    };
    
// EV 계산 실행
    let calculator = EVCalculator::new(config);
    let ev_results = calculator.calculate_action_evs(&state);
    
    println!("\n🎯 액션별 기댓값:");
    for action_ev in &ev_results {
        println!("  {:?}: EV = {:.2}칩 (신뢰도: {:.1}%)", 
                 action_ev.action, action_ev.ev, action_ev.confidence * 100.0);
    }
    
    // 최적 액션 추천
    if let Some(best_action) = ev_results.iter().max_by(|a, b| a.ev.partial_cmp(&b.ev).unwrap()) {
        println!("\n✅ 추천 액션: {:?} (EV: {:.2}칩)", best_action.action, best_action.ev);
    }
}

/// 플랍에서 탑 페어의 EV 계산
fn test_flop_top_pair() {
    let mut state = State::new();
    state.hole[0] = [0, 26]; // A♠, K♦
    state.board = vec![39, 15, 28]; // A♦, 3♥, 3♦ - 탑 페어
    state.street = 1; // 플랍
    state.pot = 60;
    state.to_call = 0; // 체크 상황
    
    println!("홀카드: A♠ K♦");
    println!("보드: A♦ 3♥ 3♦ (탑 페어)");
    println!("스트리트: 플랍");
    
    let hand_strength = hand_strength(state.hole[0], &state.board);
    println!("핸드 스트렝스: {:.3}", hand_strength);
    
    let config = EVConfig {
        sample_count: 3000,
        max_depth: 6,
        use_opponent_model: true,
    };
    
    let calculator = EVCalculator::new(config);
    let ev_results = calculator.calculate_action_evs(&state);
    
    println!("\n🎯 액션별 기댓값:");
    for action_ev in &ev_results {
        println!("  {:?}: EV = {:.2}칩 (신뢰도: {:.1}%)", 
                 action_ev.action, action_ev.ev, action_ev.confidence * 100.0);
    }
}

/// 턴에서 드로우 상황의 EV 계산
fn test_turn_draw() {
    let mut state = State::new();
    state.hole[0] = [1, 14]; // 2♠, 2♥ - 플러시 드로우
    state.board = vec![5, 18, 31, 44]; // 6♠, 6♦, 6♣, 7♠ - 스트레이트 드로우
    state.street = 2; // 턴
    state.pot = 200;
    state.to_call = 50;
    
    println!("홀카드: 2♠ 2♥");
    println!("보드: 6♠ 6♦ 6♣ 7♠ (스트레이트 드로우)");
    println!("스트리트: 턴");
    
    let hand_strength = hand_strength(state.hole[0], &state.board);
    println!("핸드 스트렝스: {:.3}", hand_strength);
    
    let config = EVConfig {
        sample_count: 2000,
        max_depth: 4,
        use_opponent_model: true,
    };
    
    let calculator = EVCalculator::new(config);
    let ev_results = calculator.calculate_action_evs(&state);
    
    println!("\n🎯 액션별 기댓값:");
    for action_ev in &ev_results {
        println!("  {:?}: EV = {:.2}칩 (신뢰도: {:.1}%)", 
                 action_ev.action, action_ev.ev, action_ev.confidence * 100.0);
    }
}

/// 리버에서 블러프 캐처 상황의 EV 계산
fn test_river_bluff_catcher() {
    let mut state = State::new();
    state.hole[0] = [0, 26]; // A♠, K♦
    state.board = vec![39, 15, 28, 42, 7]; // A♦, 3♥, 3♦, 9♣, 8♠ - 탑 페어
    state.street = 3; // 리버
    state.pot = 400;
    state.to_call = 100; // 상대방 베팅
    
    println!("홀카드: A♠ K♦");
    println!("보드: A♦ 3♥ 3♦ 9♣ 8♠ (탑 페어)");
    println!("스트리트: 리버");
    println!("상대방이 100칩 베팅");
    
    let hand_strength = hand_strength(state.hole[0], &state.board);
    println!("핸드 스트렝스: {:.3}", hand_strength);
    
    let config = EVConfig {
        sample_count: 1500,
        max_depth: 3,
        use_opponent_model: true,
    };
    
    let calculator = EVCalculator::new(config);
    let ev_results = calculator.calculate_action_evs(&state);
    
    println!("\n🎯 액션별 기댓값:");
    for action_ev in &ev_results {
        println!("  {:?}: EV = {:.2}칩 (신뢰도: {:.1}%)", 
                 action_ev.action, action_ev.ev, action_ev.confidence * 100.0);
    }
    
    // 콜의 팟 오즈 분석
    let pot_odds = state.to_call as f64 / (state.pot + state.to_call) as f64;
    println!("\n📊 팟 오즈 분석:");
    println!("  콜해야 할 금액: {}칩", state.to_call);
    println!("  총 팟: {}칩", state.pot + state.to_call);
    println!("  팟 오즈: {:.1}% (승률 필요: {:.1}%)", pot_odds * 100.0, pot_odds * 100.0);
}
