use nice_hand_core::game::holdem;
use nice_hand_core::solver::cfr_core::{Game, Trainer, GameState};

fn main() {
    println!("🔍 CFR 무한 루프 디버깅...");
    
    let state = holdem::State::new();
    println!("초기 상태:");
    println!("  to_act: {}", state.to_act);
    println!("  alive: {:?}", state.alive);
    println!("  invested: {:?}", state.invested);
    println!("  to_call: {}", state.to_call);
    println!("  actions_taken: {}", state.actions_taken);
    println!("  is_terminal: {}", state.is_terminal());
    println!("  is_chance_node: {}", state.is_chance_node());
    
    // 현재 플레이어 확인
    if let Some(player) = holdem::State::current_player(&state) {
        println!("  current_player: {}", player);
        
        // 가능한 액션 확인
        let actions = holdem::State::legal_actions(&state);
        println!("  legal_actions: {:?}", actions);
        
        // 하나의 액션 시도
        if !actions.is_empty() {
            let next_state = holdem::State::next_state(&state, actions[0]);
            println!("첫 번째 액션 {:?} 후:", actions[0]);
            println!("  to_act: {}", next_state.to_act);
            println!("  alive: {:?}", next_state.alive);
            println!("  invested: {:?}", next_state.invested);
            println!("  actions_taken: {}", next_state.actions_taken);
            println!("  is_terminal: {}", next_state.is_terminal());
            println!("  current_player: {:?}", holdem::State::current_player(&next_state));
        }
    } else {
        println!("  현재 플레이어가 없음! 이것이 문제일 수 있음.");
    }
    
    // 매우 제한적인 CFR 실행 시도
    println!("\n🧪 1회 반복으로 CFR 테스트...");
    let mut trainer = Trainer::<holdem::State>::new();
    trainer.run(vec![state], 1);
    println!("CFR이 성공적으로 완료됨!");
}
