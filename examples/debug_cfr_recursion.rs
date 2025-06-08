// Debug script to identify infinite recursion in CFR algorithm

use nice_hand_core::cfr_core::{Game, GameState};
use nice_hand_core::HoldemState;
use std::collections::HashSet;

/// Enhanced debug CFR that tracks visited states to detect cycles
struct DebugCFRTrainer {
    visited_states: HashSet<String>,
    call_stack: Vec<String>,
    max_depth: usize,
}

impl DebugCFRTrainer {
    fn new() -> Self {
        Self {
            visited_states: HashSet::new(),
            call_stack: Vec::new(),
            max_depth: 0,
        }
    }

    fn debug_state_key(&self, state: &HoldemState) -> String {
        format!(
            "street:{},pot:{},to_call:{},to_act:{},alive:{:?},invested:{:?},actions:{}",
            state.street,
            state.pot,
            state.to_call,
            state.to_act,
            state.alive,
            state.invested,
            state.actions_taken
        )
    }

    fn debug_cfr_with_cycle_detection(&mut self, state: &HoldemState, hero: usize, depth: usize) -> f64 {
        let state_key = self.debug_state_key(state);
        self.max_depth = self.max_depth.max(depth);

        // Check for infinite recursion
        if depth > 25 {
            println!("⚠️  DEPTH LIMIT REACHED at depth {}", depth);
            println!("    Call stack size: {}", self.call_stack.len());
            for (i, frame) in self.call_stack.iter().rev().take(5).enumerate() {
                println!("    Frame {}: {}", i, frame);
            }
            return 0.0;
        }

        // Check for state cycle
        if self.call_stack.contains(&state_key) {
            println!("🔄 CYCLE DETECTED at depth {}", depth);
            println!("    State: {}", state_key);
            if let Some(first_occurrence) = self.call_stack.iter().position(|s| s == &state_key) {
                println!("    First seen at depth: {}", first_occurrence);
                println!("    Cycle length: {}", depth - first_occurrence);
            }
            return 0.0;
        }

        self.call_stack.push(state_key.clone());

        // Debug current state
        if depth <= 10 || depth % 5 == 0 {
            println!("🔍 CFR depth {} - State: {}", depth, state_key);
            
            if let Some(player) = HoldemState::current_player(state) {
                let actions = HoldemState::legal_actions(state);
                println!("    Player {}, {} actions: {:?}", player, actions.len(), actions);
            } else if state.is_terminal() {
                println!("    TERMINAL state");
            } else if state.is_chance_node() {
                println!("    CHANCE node");
            } else {
                println!("    UNKNOWN state type - this might be the bug!");
            }
        }

        let result = if let Some(_player) = HoldemState::current_player(state) {
            // Player node
            let actions = HoldemState::legal_actions(state);
            if actions.is_empty() {
                println!("    No legal actions - treating as terminal");
                HoldemState::util(state, hero)
            } else {
                let mut total_util = 0.0;
                for (i, &action) in actions.iter().enumerate() {
                    println!("      Exploring action {}/{}: {:?}", i + 1, actions.len(), action);
                    let next_state = HoldemState::next_state(state, action);
                    let action_util = self.debug_cfr_with_cycle_detection(&next_state, hero, depth + 1);
                    total_util += action_util;
                }
                total_util / actions.len() as f64
            }
        } else if state.is_terminal() {
            let util = HoldemState::util(state, hero);
            println!("    Terminal utility for hero {}: {}", hero, util);
            util
        } else if state.is_chance_node() {
            println!("    Applying chance...");
            let mut rng = rand::thread_rng();
            let chance_state = HoldemState::apply_chance(state, &mut rng);
            self.debug_cfr_with_cycle_detection(&chance_state, hero, depth + 1)
        } else {
            println!("    ⚠️  UNKNOWN STATE TYPE - neither player, terminal, nor chance!");
            println!("         current_player: {:?}", HoldemState::current_player(state));
            println!("         is_terminal: {}", state.is_terminal());
            println!("         is_chance_node: {}", state.is_chance_node());
            0.0
        };

        self.call_stack.pop();
        result
    }
}

fn main() {
    println!("🐛 CFR Infinite Recursion Debug Tool");
    println!("=====================================");

    // Test 1: Basic state classification
    println!("\n📋 Test 1: Basic State Classification");
    let initial_state = HoldemState::new();
    println!("Initial state analysis:");
    println!("  current_player: {:?}", HoldemState::current_player(&initial_state));
    println!("  is_terminal: {}", initial_state.is_terminal());
    println!("  is_chance_node: {}", initial_state.is_chance_node());
    println!("  legal_actions: {:?}", HoldemState::legal_actions(&initial_state));

    // Test 2: Follow a simple path and check for problems
    println!("\n🛣️  Test 2: Simple Path Analysis");
    let mut state = initial_state.clone();
    for step in 0..10 {
        println!("Step {}: ", step);
        println!("  current_player: {:?}", HoldemState::current_player(&state));
        println!("  is_terminal: {}", state.is_terminal());
        println!("  is_chance_node: {}", state.is_chance_node());
        
        if state.is_terminal() {
            println!("  Reached terminal state at step {}", step);
            break;
        }
        
        if state.is_chance_node() {
            println!("  Applying chance...");
            let mut rng = rand::thread_rng();
            state = HoldemState::apply_chance(&state, &mut rng);
            continue;
        }
        
        let actions = HoldemState::legal_actions(&state);
        if actions.is_empty() {
            println!("  No legal actions - this might be a bug!");
            break;
        }
        
        // Take the first action (usually fold)
        let action = actions[0];
        println!("  Taking action: {:?}", action);
        state = HoldemState::next_state(&state, action);
    }

    // Test 3: CFR with cycle detection
    println!("\n🔄 Test 3: CFR with Cycle Detection");
    let mut debug_trainer = DebugCFRTrainer::new();
    let result = debug_trainer.debug_cfr_with_cycle_detection(&initial_state, 0, 0);
    println!("CFR result: {}", result);
    println!("Max depth reached: {}", debug_trainer.max_depth);
    println!("States visited: {}", debug_trainer.visited_states.len());

    // Test 4: Check for problematic state transitions
    println!("\n🔍 Test 4: State Transition Analysis");
    let test_state = initial_state.clone();
    let actions = HoldemState::legal_actions(&test_state);
    
    for (i, &action) in actions.iter().enumerate() {
        println!("Action {} ({:?}):", i, action);
        let next_state = HoldemState::next_state(&test_state, action);
        
        println!("  Before: player={:?}, terminal={}, chance={}",
            HoldemState::current_player(&test_state),
            test_state.is_terminal(),
            test_state.is_chance_node()
        );
        
        println!("  After:  player={:?}, terminal={}, chance={}",
            HoldemState::current_player(&next_state),
            next_state.is_terminal(),
            next_state.is_chance_node()
        );
        
        // Check for suspicious transitions
        if !next_state.is_terminal() && !next_state.is_chance_node() && HoldemState::current_player(&next_state).is_none() {
            println!("    ⚠️  SUSPICIOUS: Not terminal, not chance, but no current player!");
        }
    }

    println!("\n✅ Debug analysis complete");
}
