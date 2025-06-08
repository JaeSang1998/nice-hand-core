// Tournament-specific Texas Hold'em implementation
// Integrates tournament context with CFR learning for realistic tournament play

use crate::solver::cfr_core::{Game, GameState, Trainer};
use crate::game::holdem::{State as HoldemState, Act as HoldemAction};
use crate::game::tournament::{TournamentState, TournamentEvaluator, ICMCalculator};
use rand::rngs::ThreadRng;

/// Tournament Texas Hold'em state that combines regular Hold'em with tournament context
#[derive(Clone, Debug)]
pub struct TournamentHoldemState {
    /// Base Hold'em game state
    pub holdem_state: HoldemState,
    
    /// Tournament context
    pub tournament_state: TournamentState,
    
    /// Player positions in tournament (stack sizes, blind levels, etc.)
    pub tournament_positions: Vec<TournamentPlayerPosition>,
    
    /// ICM values for current situation
    pub icm_values: Vec<f64>,
    
    /// Bubble pressure indicator
    pub bubble_pressure: f64,
}

#[derive(Clone, Debug)]
pub struct TournamentPlayerPosition {
    pub player_id: u32,
    pub stack_size: u32,
    pub position_rank: u32,  // 1 = chip leader, higher = shorter stack
    pub pay_jump_equity: f64, // Equity difference between current and next payout
}

impl TournamentHoldemState {
    /// Create new tournament hand
    pub fn new_tournament_hand(
        holdem_state: HoldemState,
        tournament_state: TournamentState,
        player_stacks: Vec<u32>,
    ) -> Self {
        let mut tournament_positions = Vec::new();
        let active_players = tournament_state.players_remaining as usize;
        
        // Create player positions with tournament context
        for (i, &stack) in player_stacks.iter().take(active_players).enumerate() {
            tournament_positions.push(TournamentPlayerPosition {
                player_id: i as u32,
                stack_size: stack,
                position_rank: 0, // Will be calculated
                pay_jump_equity: 0.0, // Will be calculated
            });
        }
        
        // Calculate ICM values
        let default_payouts: Vec<u64> = vec![100, 60, 40, 25, 15, 10]; // Default payout structure
        let payouts = if tournament_state.payout_structure.is_empty() {
            default_payouts
        } else {
            tournament_state.payout_structure.iter().map(|p| p.amount).collect()
        };
        let icm_calculator = ICMCalculator::new(player_stacks.clone(), payouts);
        let icm_values = icm_calculator.calculate_equity();
        
        // Calculate bubble pressure
        let bubble_pressure = Self::calculate_bubble_pressure(&tournament_state, &player_stacks);
        
        TournamentHoldemState {
            holdem_state,
            tournament_state,
            tournament_positions,
            icm_values,
            bubble_pressure,
        }
    }
    
    /// Calculate bubble pressure based on tournament stage
    fn calculate_bubble_pressure(tournament_state: &TournamentState, _stacks: &[u32]) -> f64 {
        let payout_spots = tournament_state.payout_structure.len() as u32;
        let players_remaining = tournament_state.players_remaining;
        
        if players_remaining <= payout_spots {
            0.0 // Already in the money
        } else if players_remaining <= payout_spots + 3 {
            // High bubble pressure
            let bubble_distance = (players_remaining - payout_spots) as f64;
            1.0 - (bubble_distance / 4.0) // Linear decrease from 1.0 to 0.25
        } else {
            // Low bubble pressure
            0.1
        }
    }
    
    /// Update ICM values after action
    pub fn update_icm_after_action(&mut self, _action: &HoldemAction, _player: usize) {
        // Recalculate ICM values based on new stack distributions
        let current_stacks: Vec<u32> = self.holdem_state.stack.iter()
            .take(self.tournament_state.players_remaining as usize)
            .cloned()
            .collect();
        
        let payouts: Vec<u64> = self.tournament_state.payout_structure.iter()
            .map(|p| p.amount)
            .collect();
        
        let icm_calculator = ICMCalculator::new(current_stacks, payouts);
        self.icm_values = icm_calculator.calculate_equity();
    }
}

impl GameState for TournamentHoldemState {
    fn is_terminal(&self) -> bool {
        self.holdem_state.is_terminal()
    }
    
    fn is_chance_node(&self) -> bool {
        self.holdem_state.is_chance_node()
    }
}

/// Tournament-specific Texas Hold'em game that uses ICM-adjusted utilities
#[derive(Clone)]
pub struct TournamentHoldem {
    pub evaluator: TournamentEvaluator,
}

impl TournamentHoldem {
    pub fn new(tournament_state: TournamentState, player_stacks: Vec<u32>) -> Self {
        let evaluator = TournamentEvaluator::new(tournament_state, player_stacks);
        
        TournamentHoldem {
            evaluator,
        }
    }
}

impl Game for TournamentHoldem {
    type State = TournamentHoldemState;
    type Action = HoldemAction;
    type InfoKey = u64; // Tournament-aware information set key
    
    const N_PLAYERS: usize = 6; // Support up to 6 players per table
    
    fn current_player(state: &Self::State) -> Option<usize> {
        if state.holdem_state.is_terminal() || state.holdem_state.is_chance_node() {
            None
        } else {
            Some(state.holdem_state.to_act)
        }
    }
    
    fn legal_actions(state: &Self::State) -> Vec<Self::Action> {
        // Get base Hold'em actions
        let base_actions = crate::game::holdem::State::legal_actions(&state.holdem_state);
        
        // Filter actions based on tournament context
        let mut tournament_actions = Vec::new();
        
        for action in base_actions {
            if Self::is_action_allowed_in_tournament(&action, state) {
                tournament_actions.push(action);
            }
        }
        
        if tournament_actions.is_empty() {
            // Always allow fold as last resort
            vec![HoldemAction::Fold]
        } else {
            tournament_actions
        }
    }
    
    fn next_state(state: &Self::State, action: Self::Action) -> Self::State {
        let mut new_state = state.clone();
        
        // Apply Hold'em action
        new_state.holdem_state = crate::game::holdem::State::next_state(&state.holdem_state, action);
        
        // Update tournament context
        if let Some(current_player) = Self::current_player(state) {
            new_state.update_icm_after_action(&action, current_player);
        }
        
        new_state
    }
    
    fn apply_chance(state: &Self::State, rng: &mut ThreadRng) -> Self::State {
        let mut new_state = state.clone();
        new_state.holdem_state = crate::game::holdem::State::apply_chance(&state.holdem_state, rng);
        new_state
    }
    
    fn util(state: &Self::State, hero: usize) -> f64 {
        if !state.holdem_state.is_terminal() {
            return 0.0;
        }
        
        // Get base Hold'em utility (chip change)
        let chip_change = crate::game::holdem::State::util(&state.holdem_state, hero) as i32;
        
        // Convert to ICM-adjusted utility
        let current_stacks: Vec<u32> = state.holdem_state.stack.iter()
            .take(state.tournament_state.players_remaining as usize)
            .cloned()
            .collect();
        
        let payouts: Vec<u64> = state.tournament_state.payout_structure.iter()
            .map(|p| p.amount)
            .collect();
        
        let icm_evaluator = ICMCalculator::new(current_stacks, payouts);
        let icm_adjustment = icm_evaluator.calculate_icm_pressure(hero, chip_change);
        
        // Apply bubble pressure adjustment
        let bubble_adjustment = if state.bubble_pressure > 0.5 {
            // High bubble pressure - be more risk averse
            if chip_change < 0 { 
                chip_change as f64 * (1.0 + state.bubble_pressure) 
            } else { 
                chip_change as f64 * (1.0 - state.bubble_pressure * 0.3) 
            }
        } else {
            chip_change as f64
        };
        
        // Combine ICM and bubble adjustments
        icm_adjustment + bubble_adjustment * 0.1
    }
    
    fn info_key(state: &Self::State, player: usize) -> Self::InfoKey {
        // Create tournament-aware information set key
        let base_key = crate::game::holdem::State::info_key(&state.holdem_state, player);
        
        // Add tournament context to key
        let tournament_context = (
            (state.bubble_pressure * 100.0) as u64,
            state.tournament_positions[player].position_rank as u64,
            (state.icm_values[player] * 1000.0) as u64,
        );
        
        // Combine base key with tournament context
        base_key.wrapping_add(
            tournament_context.0.wrapping_mul(1000003)
                .wrapping_add(tournament_context.1.wrapping_mul(1000033))
                .wrapping_add(tournament_context.2.wrapping_mul(1000037))
        )
    }
}

impl TournamentHoldem {
    /// Check if action is allowed in tournament context
    fn is_action_allowed_in_tournament(action: &HoldemAction, state: &TournamentHoldemState) -> bool {
        match action {
            HoldemAction::Fold => true, // Always allowed
            HoldemAction::Call => true, // Always allowed if legal
            HoldemAction::Raise(size) => {
                // Check if raise size is reasonable for tournament
                let current_player = state.holdem_state.to_act;
                let _player_stack = state.holdem_state.stack[current_player];
                
                // Don't allow aggressive raises near bubble for medium stacks
                if state.bubble_pressure > 0.7 && *size > 1 {
                    false
                } else {
                    true
                }
            }
        }
    }
}

/// Tournament CFR trainer that incorporates ICM calculations
pub struct TournamentCFRTrainer {
    pub base_trainer: Trainer<TournamentHoldem>,
    pub tournament_game: TournamentHoldem,
}

impl TournamentCFRTrainer {
    /// Create new tournament CFR trainer
    pub fn new(tournament_state: TournamentState, player_stacks: Vec<u32>) -> Self {
        let tournament_game = TournamentHoldem::new(tournament_state, player_stacks);
        let base_trainer = Trainer::new();
        
        TournamentCFRTrainer {
            base_trainer,
            tournament_game,
        }
    }
    
    /// Train tournament strategy with ICM considerations
    pub fn train_tournament_strategy(&mut self, iterations: usize, roots: &[TournamentHoldemState]) {
        println!("🏆 Training tournament strategy with ICM calculations...");
        println!("📊 Iterations: {}, Roots: {}", iterations, roots.len());
        
        let start_time = std::time::Instant::now();
        self.base_trainer.run(roots.to_vec(), iterations);
        let elapsed = start_time.elapsed();
        
        println!("✅ Tournament training completed in {:.2?}", elapsed);
        println!("📈 Nodes created: {}", self.base_trainer.nodes.len());
    }
    
    /// Get strategy for tournament situation
    pub fn get_tournament_strategy(&self, state: &TournamentHoldemState, player: usize) -> Vec<f64> {
        let info_key = TournamentHoldem::info_key(state, player);
        
        if let Some(node) = self.base_trainer.nodes.get(&info_key) {
            node.average()
        } else {
            // Default uniform strategy if no training data
            let actions = TournamentHoldem::legal_actions(state);
            let uniform_prob = 1.0 / actions.len() as f64;
            vec![uniform_prob; actions.len()]
        }
    }
    
    /// Evaluate tournament decision with ICM considerations
    pub fn evaluate_tournament_decision(
        &self, 
        state: &TournamentHoldemState, 
        action: HoldemAction, 
        player: usize
    ) -> f64 {
        let next_state = TournamentHoldem::next_state(state, action);
        
        if next_state.holdem_state.is_terminal() {
            TournamentHoldem::util(&next_state, player)
        } else {
            // Use current strategy to estimate value
            let strategy = self.get_tournament_strategy(&next_state, player);
            let actions = TournamentHoldem::legal_actions(&next_state);
            
            let mut expected_value = 0.0;
            for (i, &action) in actions.iter().enumerate() {
                let prob = strategy.get(i).unwrap_or(&0.0);
                let action_state = TournamentHoldem::next_state(&next_state, action);
                let value = if action_state.holdem_state.is_terminal() {
                    TournamentHoldem::util(&action_state, player)
                } else {
                    0.0 // Simplified - could recurse deeper
                };
                expected_value += prob * value;
            }
            
            expected_value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tournament_holdem_creation() {
        let holdem_state = crate::game::holdem::State::new();
        let tournament_state = TournamentState::new(
            crate::game::tournament::TournamentStructure {
                levels: vec![],
                level_duration_minutes: 15,
                starting_stack: 1500,
                ante_schedule: vec![],
            },
            100,
            10000
        );
        
        let player_stacks = vec![1500, 1200, 1800, 900, 2100, 1000];
        let tournament_holdem_state = TournamentHoldemState::new_tournament_hand(
            holdem_state,
            tournament_state,
            player_stacks,
        );
        
        assert_eq!(tournament_holdem_state.icm_values.len(), 6);
        assert!(tournament_holdem_state.bubble_pressure >= 0.0);
        assert!(tournament_holdem_state.bubble_pressure <= 1.0);
    }
    
    #[test]
    fn test_tournament_cfr_trainer() {
        let tournament_state = TournamentState::new(
            crate::game::tournament::TournamentStructure {
                levels: vec![],
                level_duration_minutes: 15,
                starting_stack: 1500,
                ante_schedule: vec![],
            },
            6,  // Use 6 players to match holdem state
            5000
        );
        
        let player_stacks_early = vec![1500, 1200, 1800, 900, 2100, 1000];
        let mut trainer = TournamentCFRTrainer::new(tournament_state, player_stacks_early.clone());
        
        // Create sample training scenarios - use a heads-up game to match holdem::State::new()
        let holdem_state = crate::game::holdem::State::new(); // This creates 2-player game
        let player_stacks = vec![1000, 1000]; // Match the 2-player setup from holdem::new()
        let tournament_holdem_state = TournamentHoldemState::new_tournament_hand(
            holdem_state,
            trainer.tournament_game.evaluator.tournament_state.clone(),
            player_stacks,
        );
        
        let roots = vec![tournament_holdem_state];
        
        // Use just 1 iteration for basic functionality test
        trainer.train_tournament_strategy(1, &roots);
        
        // Verify training completed
        println!("🧪 Test completed - nodes created: {}", trainer.base_trainer.nodes.len());
    }
    
    #[test]
    fn test_tournament_action_filtering() {
        let tournament_state = TournamentState::new(
            crate::game::tournament::TournamentStructure {
                levels: vec![],
                level_duration_minutes: 15,
                starting_stack: 1500,
                ante_schedule: vec![],
            },
            6,
            5000
        );
        
        let holdem_state = crate::game::holdem::State::new();
        let player_stacks = vec![1000, 1000]; // Match 2-player setup
        let tournament_holdem_state = TournamentHoldemState::new_tournament_hand(
            holdem_state,
            tournament_state,
            player_stacks,
        );
        
        // Test legal actions work
        let actions = TournamentHoldem::legal_actions(&tournament_holdem_state);
        assert!(actions.len() > 0);
        println!("🎯 Legal actions: {:?}", actions);
        
        // Test next state works
        if let Some(action) = actions.first() {
            let next_state = TournamentHoldem::next_state(&tournament_holdem_state, *action);
            println!("✅ Next state transition successful");
        }
    }
}
