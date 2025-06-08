use nice_hand_core::{HoldemState};
use nice_hand_core::cfr_core::{Game, GameState};
use std::collections::HashSet;

fn main() {
    println!("=== CFR 무한 재귀 분석 ===");
    
    // 초기 게임 상태 생성
    let initial_state = HoldemState::new();
    println!("초기 상태:");
    print_state(&initial_state);
    
    // 상태 변화 추적을 위한 Set
    let mut seen_states = HashSet::new();
    let mut current_state = initial_state;
    let mut depth = 0;
    
    // 상태 변화를 추적하며 순환 탐지
    loop {
        depth += 1;
        if depth > 50 {
            println!("\n❌ 깊이 50 도달 - 종료");
            break;
        }
        
        // 상태 해시 생성 (간단한 방법)
        let state_hash = format!("{:?}", current_state);
        
        if seen_states.contains(&state_hash) {
            println!("\n🔄 상태 순환 탐지! 깊이: {}", depth);
            println!("중복된 상태:");
            print_state(&current_state);
            break;
        }
        
        seen_states.insert(state_hash);
        
        println!("\n--- 깊이 {} ---", depth);
        print_state(&current_state);
        
        // 터미널 상태 체크
        if current_state.is_terminal() {
            println!("✅ 터미널 상태 도달");
            break;
        }
        
        // 찬스 노드 체크
        if current_state.is_chance_node() {
            println!("🎲 찬스 노드 - 카드 딜링");
            let mut rng = rand::thread_rng();
            current_state = HoldemState::apply_chance(&current_state, &mut rng);
            continue;
        }
        
        // 플레이어 액션 노드
        let legal_actions = HoldemState::legal_actions(&current_state);
        println!("합법적 액션들: {:?}", legal_actions);
        
        if legal_actions.is_empty() {
            println!("❌ 합법적 액션이 없음!");
            break;
        }
        
        // 첫 번째 액션 선택 (항상 같은 패턴으로 테스트)
        let action = legal_actions[0];
        println!("선택한 액션: {:?}", action);
        
        // 액션 적용
        current_state = HoldemState::next_state(&current_state, action);
    }
    
    println!("\n=== 분석 완료 ===");
}

fn print_state(state: &HoldemState) {
    println!("  베팅 라운드: {}", state.street);
    println!("  행동할 플레이어: {:?}", HoldemState::current_player(state));
    println!("  터미널: {}", state.is_terminal());
    println!("  찬스 노드: {}", state.is_chance_node());
    println!("  팟 크기: {}", state.pot);
    println!("  보드: {:?}", state.board);
    println!("  to_act: {}", state.to_act);
    println!("  액션 수: {}", state.actions_taken);
    
    // 상세한 베팅 상태
    if !state.is_terminal() && !state.is_chance_node() {
        let legal_actions = HoldemState::legal_actions(state);
        println!("  가능한 액션 수: {}", legal_actions.len());
        if legal_actions.len() <= 5 {
            println!("  가능한 액션들: {:?}", legal_actions);
        }
    }
}
