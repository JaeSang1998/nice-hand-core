use nice_hand_core::game::holdem;
use nice_hand_core::solver::cfr_core::{Game, Trainer, GameState};

fn main() {
    println!("🔍 Debugging CFR infinite loop...");
    
    let state = holdem::State::new();
    println!("Initial state:");
    println!("  to_act: {}", state.to_act);
    println!("  alive: {:?}", state.alive);
    println!("  invested: {:?}", state.invested);
    println!("  to_call: {}", state.to_call);
    println!("  actions_taken: {}", state.actions_taken);
    println!("  is_terminal: {}", state.is_terminal());
    println!("  is_chance_node: {}", state.is_chance_node());
    
    // Check current player
    if let Some(player) = holdem::State::current_player(&state) {
        println!("  current_player: {}", player);
        
        // Check legal actions
        let actions = holdem::State::legal_actions(&state);
        println!("  legal_actions: {:?}", actions);
        
        // Try one action
        if !actions.is_empty() {
            let next_state = holdem::State::next_state(&state, actions[0]);
            println!("After first action {:?}:", actions[0]);
            println!("  to_act: {}", next_state.to_act);
            println!("  alive: {:?}", next_state.alive);
            println!("  invested: {:?}", next_state.invested);
            println!("  actions_taken: {}", next_state.actions_taken);
            println!("  is_terminal: {}", next_state.is_terminal());
            println!("  current_player: {:?}", holdem::State::current_player(&next_state));
        }
    } else {
        println!("  No current player! This might be the issue.");
    }
    
    // Try a very limited CFR run
    println!("\n🧪 Testing CFR with 1 iteration...");
    let mut trainer = Trainer::<holdem::State>::new();
    trainer.run(vec![state], 1);
    println!("CFR completed successfully!");
}
