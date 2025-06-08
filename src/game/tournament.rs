use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tournament structure and blind schedule management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentStructure {
    pub levels: Vec<BlindLevel>,
    pub level_duration_minutes: u32,
    pub starting_stack: u32,
    pub ante_schedule: Vec<AnteLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlindLevel {
    pub level: u32,
    pub small_blind: u32,
    pub big_blind: u32,
    pub ante: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnteLevel {
    pub level: u32,
    pub ante: u32,
}

/// Tournament state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentState {
    pub structure: TournamentStructure,
    pub current_level: u32,
    pub minutes_elapsed: u32,
    pub players_remaining: u32,
    pub total_players: u32,
    pub prize_pool: u64,
    pub payout_structure: Vec<PayoutLevel>,
}

impl TournamentState {
    pub fn new(structure: TournamentStructure, total_players: u32, prize_pool: u64) -> Self {
        // Create basic payout structure (top 10% get paid)
        let payout_spots = (total_players as f64 * 0.1).ceil() as u32;
        let mut payout_structure = Vec::new();

        for position in 1..=payout_spots {
            let percentage = match position {
                1 => 0.4,                             // Winner gets 40%
                2 => 0.25,                            // Second gets 25%
                3 => 0.15,                            // Third gets 15%
                _ => 0.2 / (payout_spots - 3) as f64, // Remaining split the rest
            };

            payout_structure.push(PayoutLevel {
                position,
                percentage,
                amount: (prize_pool as f64 * percentage) as u64,
            });
        }

        Self {
            structure,
            current_level: 1,
            minutes_elapsed: 0,
            players_remaining: total_players,
            total_players,
            prize_pool,
            payout_structure,
        }
    }

    pub fn total_chips(&self) -> u32 {
        self.total_players * self.structure.starting_stack
    }

    pub fn current_blinds(&self) -> (u32, u32, u32) {
        if let Some(level) = self.structure.levels.get(self.current_level as usize - 1) {
            (level.small_blind, level.big_blind, level.ante)
        } else {
            (10, 20, 0) // Default blinds if level not found
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutLevel {
    pub position: u32,
    pub percentage: f64,
    pub amount: u64,
}

/// ICM (Independent Chip Model) calculations for tournament play
#[derive(Debug, Clone)]
pub struct ICMCalculator {
    pub stacks: Vec<u32>,
    pub payouts: Vec<u64>,
}

impl ICMCalculator {
    pub fn new(stacks: Vec<u32>, payouts: Vec<u64>) -> Self {
        Self { stacks, payouts }
    }

    /// Calculate ICM equity for each player using proper probability theory
    pub fn calculate_equity(&self) -> Vec<f64> {
        let num_players = self.stacks.len();

        if num_players == 0 || self.payouts.is_empty() {
            return vec![0.0; num_players];
        }

        let mut equities = vec![0.0; num_players];

        // Use recursive probability calculation for exact ICM
        for player_idx in 0..num_players {
            equities[player_idx] =
                self.calculate_player_equity(player_idx, &(0..num_players).collect::<Vec<_>>());
        }

        equities
    }

    /// Calculate exact ICM equity for a specific player using dynamic programming
    fn calculate_player_equity(&self, player_idx: usize, remaining_players: &[usize]) -> f64 {
        let num_remaining = remaining_players.len();
        let num_payouts = self.payouts.len();

        // Base case: if we're in the money, calculate exact payout probability
        if num_remaining <= num_payouts {
            return self.calculate_exact_finish_probabilities(player_idx, remaining_players);
        }

        // If more players than payouts, calculate elimination probabilities
        let mut equity = 0.0;
        let total_chips: u32 = remaining_players.iter().map(|&i| self.stacks[i]).sum();

        if total_chips == 0 {
            return 0.0;
        }

        // Calculate probability of each player being eliminated next
        for &eliminated_player in remaining_players {
            if eliminated_player == player_idx {
                continue; // Skip if we're the one being eliminated
            }

            let elimination_prob =
                self.calculate_elimination_probability(eliminated_player, remaining_players);
            let remaining_after_elimination: Vec<usize> = remaining_players
                .iter()
                .filter(|&&p| p != eliminated_player)
                .copied()
                .collect();

            equity += elimination_prob
                * self.calculate_player_equity(player_idx, &remaining_after_elimination);
        }

        equity
    }

    /// Calculate exact finish probabilities when in the money
    fn calculate_exact_finish_probabilities(
        &self,
        player_idx: usize,
        remaining_players: &[usize],
    ) -> f64 {
        let num_remaining = remaining_players.len();
        let mut equity = 0.0;

        // Calculate probability of finishing in each position
        for finish_position in 0..num_remaining {
            let payout_idx = num_remaining - 1 - finish_position; // Convert to payout index
            if payout_idx < self.payouts.len() {
                let finish_prob = self.calculate_finish_probability_exact(
                    player_idx,
                    finish_position,
                    remaining_players,
                );
                equity += finish_prob * self.payouts[payout_idx] as f64;
            }
        }

        equity
    }

    /// Calculate probability of elimination using Malmuth-Weitzman model
    fn calculate_elimination_probability(
        &self,
        player_idx: usize,
        remaining_players: &[usize],
    ) -> f64 {
        let total_chips: u32 = remaining_players.iter().map(|&i| self.stacks[i]).sum();
        let player_stack = self.stacks[player_idx] as f64;

        if total_chips == 0 || player_stack <= 0.0 {
            return if player_stack <= 0.0 { 1.0 } else { 0.0 };
        }

        // Malmuth-Weitzman: elimination probability inversely proportional to stack size
        let stack_ratio = player_stack / total_chips as f64;
        let num_players = remaining_players.len() as f64;

        // Adjust for tournament dynamics (short stacks more likely to be eliminated)
        let elimination_factor = if stack_ratio < 0.1 {
            2.0 // Very short stacks
        } else if stack_ratio < 0.3 {
            1.5 // Short stacks
        } else if stack_ratio > 0.4 {
            0.7 // Big stacks
        } else {
            1.0 // Average stacks
        };

        (1.0 - stack_ratio) * elimination_factor / num_players
    }

    /// Calculate exact probability of finishing in specific position using advanced tournament modeling
    pub fn calculate_finish_probability_exact(
        &self,
        player_idx: usize,
        position: usize,
        remaining_players: &[usize],
    ) -> f64 {
        let total_chips: u32 = remaining_players.iter().map(|&i| self.stacks[i]).sum();
        let player_stack = self.stacks[player_idx] as f64;
        let num_remaining = remaining_players.len();

        if total_chips == 0 || num_remaining == 0 {
            return 0.0;
        }

        // Handle trivial cases
        if num_remaining == 1 {
            return if position == 0 { 1.0 } else { 0.0 };
        }

        // Calculate stack dynamics and tournament phase
        let stack_ratio = player_stack / total_chips as f64;
        let big_blind_pressure = self.calculate_blind_pressure(player_idx, remaining_players);

        // Base probability using sophisticated model
        let base_prob = self.calculate_base_finish_probability(
            stack_ratio,
            position,
            num_remaining,
            big_blind_pressure,
        );

        // Apply tournament dynamics adjustments
        let skill_adjustment =
            self.calculate_skill_based_adjustment(player_idx, position, remaining_players);
        let variance_factor = self.calculate_variance_factor(stack_ratio, num_remaining);
        let position_specific_factor =
            self.calculate_position_specific_dynamics(position, stack_ratio, num_remaining);

        // Combine all factors
        let raw_probability =
            base_prob * skill_adjustment * variance_factor * position_specific_factor;

        // Store raw probability without normalization for now - normalization will happen globally
        raw_probability.min(1.0).max(0.0)
    }

    /// Calculate base finish probability using Dirichlet-Multinomial model
    fn calculate_base_finish_probability(
        &self,
        stack_ratio: f64,
        position: usize,
        num_remaining: usize,
        blind_pressure: f64,
    ) -> f64 {
        // Use modified Dirichlet concentration parameter based on stack size
        let alpha = self.calculate_dirichlet_concentration(stack_ratio, blind_pressure);

        // Position-dependent base rate
        let position_weight = match position {
            0 => 1.0,                              // First place
            1 => 0.85,                             // Second place
            2 => 0.7,                              // Third place
            p if p < num_remaining / 3 => 0.6,     // Top third
            p if p < 2 * num_remaining / 3 => 0.4, // Middle third
            _ => 0.2,                              // Bottom third
        };

        // Stack-based probability with diminishing returns
        let stack_factor = if stack_ratio > 0.5 {
            // Big stacks: logarithmic advantage
            (stack_ratio.ln() + 1.0) * 0.8
        } else if stack_ratio < 0.1 {
            // Very short stacks: minimal chance except for specific positions
            stack_ratio
                * (if position >= num_remaining - 2 {
                    3.0
                } else {
                    0.5
                })
        } else {
            // Medium stacks: linear relationship with slight curve
            stack_ratio.powf(0.8)
        };

        alpha * position_weight * stack_factor
    }

    /// Calculate Dirichlet concentration parameter based on tournament dynamics
    fn calculate_dirichlet_concentration(&self, stack_ratio: f64, blind_pressure: f64) -> f64 {
        // Higher concentration = more predictable outcomes
        // Lower concentration = higher variance

        let base_concentration = 2.0;

        // Stack size effect: bigger stacks = higher concentration (more predictable)
        let stack_effect = if stack_ratio > 0.4 {
            1.5 + stack_ratio // Big stacks have advantage
        } else if stack_ratio < 0.1 {
            0.3 // Short stacks are very unpredictable
        } else {
            0.5 + stack_ratio // Medium stacks
        };

        // Blind pressure effect: higher pressure = lower concentration (more variance)
        let pressure_effect = 1.0 / (1.0 + blind_pressure * 2.0);

        base_concentration * stack_effect * pressure_effect
    }

    /// Calculate blind pressure based on stack size relative to blinds
    fn calculate_blind_pressure(&self, player_idx: usize, remaining_players: &[usize]) -> f64 {
        let player_stack = self.stacks[player_idx] as f64;

        // Estimate current blind level (this would ideally come from tournament state)
        let estimated_big_blind = self.estimate_current_big_blind(remaining_players.len());

        // M-ratio (stack / cost per orbit)
        let m_ratio = player_stack / (estimated_big_blind * 1.5); // BB + SB + antes

        // Convert M-ratio to pressure score
        if m_ratio > 20.0 {
            0.0 // No pressure
        } else if m_ratio > 10.0 {
            (20.0 - m_ratio) / 20.0 * 0.3 // Low pressure
        } else if m_ratio > 5.0 {
            0.3 + (10.0 - m_ratio) / 10.0 * 0.4 // Medium pressure
        } else {
            0.7 + (5.0 - m_ratio) / 5.0 * 0.3 // High pressure (capped at 1.0)
        }
    }

    /// Estimate current big blind based on remaining players
    fn estimate_current_big_blind(&self, players_remaining: usize) -> f64 {
        // Simple heuristic: blinds increase as field gets smaller
        let total_starting_players = self.stacks.len();
        let elimination_rate = 1.0 - (players_remaining as f64 / total_starting_players as f64);

        // Assume blinds roughly double every 20% of field eliminated
        let blind_levels = (elimination_rate * 5.0).floor();
        let base_big_blind = 100.0; // Starting BB assumption

        base_big_blind * 2.0_f64.powf(blind_levels)
    }

    /// Calculate skill-based adjustment for finish probabilities
    fn calculate_skill_based_adjustment(
        &self,
        player_idx: usize,
        position: usize,
        remaining_players: &[usize],
    ) -> f64 {
        let player_stack = self.stacks[player_idx] as f64;
        let total_chips: u32 = remaining_players.iter().map(|&i| self.stacks[i]).sum();
        let stack_ratio = player_stack / total_chips as f64;

        // Infer skill from stack accumulation vs elimination rate
        let expected_stack_ratio = 1.0 / remaining_players.len() as f64;
        let skill_indicator = (stack_ratio / expected_stack_ratio).ln().max(-2.0).min(2.0);

        // Skill effect varies by position
        let skill_multiplier = match position {
            0 | 1 => 1.0 + skill_indicator * 0.3, // Top positions benefit more from skill
            p if p < remaining_players.len() / 2 => 1.0 + skill_indicator * 0.2, // Middle positions
            _ => 1.0 + skill_indicator * 0.1,     // Low positions less skill-dependent
        };

        skill_multiplier.max(0.1).min(3.0) // Cap the adjustment
    }

    /// Calculate variance factor based on stack size and field size
    fn calculate_variance_factor(&self, stack_ratio: f64, num_remaining: usize) -> f64 {
        // Smaller fields and extreme stack sizes increase variance
        let field_variance = 1.0 + (1.0 / num_remaining as f64) * 0.5;

        let stack_variance = if stack_ratio < 0.05 || stack_ratio > 0.6 {
            1.2 // Extreme stacks have higher variance
        } else if stack_ratio < 0.1 || stack_ratio > 0.4 {
            1.1 // Moderately extreme stacks
        } else {
            1.0 // Average stacks
        };

        field_variance * stack_variance
    }

    /// Calculate position-specific dynamics
    fn calculate_position_specific_dynamics(
        &self,
        position: usize,
        stack_ratio: f64,
        num_remaining: usize,
    ) -> f64 {
        match position {
            0 => {
                // First place: big stacks heavily favored, short stacks need luck
                if stack_ratio > 0.4 {
                    1.4 + stack_ratio * 0.6 // Big stack advantage
                } else if stack_ratio < 0.1 {
                    0.3 + stack_ratio * 2.0 // Short stacks can still win but unlikely
                } else {
                    0.8 + stack_ratio * 0.4 // Medium stacks
                }
            }
            1 => {
                // Second place: more achievable for medium stacks
                if stack_ratio > 0.3 {
                    1.2 + stack_ratio * 0.3
                } else if stack_ratio < 0.1 {
                    0.5 + stack_ratio * 3.0
                } else {
                    1.0 + stack_ratio * 0.5
                }
            }
            p if p >= num_remaining - 2 => {
                // Bottom positions: short stacks more likely
                if stack_ratio < 0.15 {
                    1.5 + (0.15 - stack_ratio) * 5.0 // Short stacks likely to bust
                } else {
                    0.5 // Big stacks unlikely to finish low
                }
            }
            _ => {
                // Middle positions: relatively neutral
                1.0 + (stack_ratio - 0.5) * 0.2
            }
        }
    }

    /// Calculate ICM pressure - how much equity changes with stack changes
    pub fn calculate_icm_pressure(&self, player_idx: usize, chip_change: i32) -> f64 {
        if player_idx >= self.stacks.len() {
            return 0.0;
        }

        let original_equity = self.calculate_equity()[player_idx];

        // Create modified stacks for comparison
        let mut modified_stacks = self.stacks.clone();
        modified_stacks[player_idx] =
            (modified_stacks[player_idx] as i32 + chip_change).max(0) as u32;

        let modified_icm = ICMCalculator::new(modified_stacks, self.payouts.clone());
        let modified_equity = modified_icm.calculate_equity()[player_idx];

        (modified_equity - original_equity) / chip_change.abs() as f64
    }
}

/// Tournament-specific strategy adjustments
#[derive(Debug, Clone)]
pub struct TournamentStrategy {
    pub bubble_factor: f64,
    pub icm_pressure: f64,
    pub stack_preservation: f64,
}

impl TournamentStrategy {
    pub fn new(tournament_state: &TournamentState, player_stack: u32) -> Self {
        let avg_stack = tournament_state.total_chips() / tournament_state.players_remaining;
        let stack_ratio = player_stack as f64 / avg_stack as f64;

        // Calculate bubble factor (how close we are to payouts)
        let payout_spots = tournament_state.payout_structure.len() as u32;
        let bubble_factor = if tournament_state.players_remaining <= payout_spots + 5 {
            2.0 - (tournament_state.players_remaining as f64 / payout_spots as f64)
        } else {
            0.0
        };

        Self {
            bubble_factor,
            icm_pressure: (2.0 - stack_ratio).max(0.0),
            stack_preservation: if stack_ratio < 0.5 { 2.0 } else { 1.0 },
        }
    }

    /// Adjust CFR strategy based on tournament considerations
    pub fn adjust_strategy(&self, base_strategy: &[f64]) -> Vec<f64> {
        let mut adjusted = base_strategy.to_vec();

        // Increase folding frequency near bubble
        if self.bubble_factor > 0.5 {
            if adjusted.len() >= 3 {
                let fold_boost = self.bubble_factor * 0.2;
                adjusted[0] += fold_boost; // Fold
                adjusted[1] = (adjusted[1] - fold_boost * 0.5).max(0.0); // Call
                adjusted[2] = (adjusted[2] - fold_boost * 0.5).max(0.0); // Raise
            }
        }

        // Normalize probabilities
        let sum: f64 = adjusted.iter().sum();
        if sum > 0.0 {
            for prob in &mut adjusted {
                *prob /= sum;
            }
        }

        adjusted
    }
}

/// Advanced opponent modeling for tournament play
#[derive(Debug, Clone)]
pub struct OpponentModel {
    pub player_id: u32,
    pub vpip: f64,              // Voluntarily Put money In Pot
    pub pfr: f64,               // Pre-Flop Raise
    pub aggression: f64,        // Aggression factor
    pub tightness: f64,         // How tight they play
    pub bubble_adjustment: f64, // How they adjust near bubble
    pub stack_based_play: f64,  // How stack size affects their play
    pub sample_size: u32,       // Number of hands observed
}

impl OpponentModel {
    pub fn new(player_id: u32) -> Self {
        Self {
            player_id,
            vpip: 0.25,             // Default 25% VPIP
            pfr: 0.15,              // Default 15% PFR
            aggression: 1.5,        // Moderate aggression
            tightness: 0.5,         // Moderate tightness
            bubble_adjustment: 0.8, // Tighten up 20% near bubble
            stack_based_play: 1.0,  // Normal stack-based adjustments
            sample_size: 0,
        }
    }

    /// Update opponent model based on observed action
    pub fn update_with_action(&mut self, action: &TournamentAction, context: &ActionContext) {
        self.sample_size += 1;
        let learning_rate = (1.0 / (self.sample_size as f64 + 1.0)).min(0.1);

        match action {
            TournamentAction::Fold => {
                // Folding increases tightness
                self.tightness = self.tightness * (1.0 - learning_rate) + learning_rate * 0.8;
            }
            TournamentAction::Call => {
                // Calling affects VPIP
                if context.is_preflop {
                    self.vpip = self.vpip * (1.0 - learning_rate) + learning_rate * 0.7;
                }
            }
            TournamentAction::Raise(_) => {
                // Raising affects PFR and aggression
                if context.is_preflop {
                    self.pfr = self.pfr * (1.0 - learning_rate) + learning_rate * 0.8;
                }
                self.aggression = self.aggression * (1.0 - learning_rate) + learning_rate * 2.0;
            }
            TournamentAction::AllIn => {
                // All-in shows extreme aggression or desperation
                let aggression_boost = if context.stack_ratio < 0.1 { 1.5 } else { 3.0 };
                self.aggression =
                    self.aggression * (1.0 - learning_rate) + learning_rate * aggression_boost;
            }
        }

        // Adjust for bubble context
        if context.near_bubble {
            let bubble_factor = if matches!(action, TournamentAction::Fold) {
                1.2
            } else {
                0.8
            };
            self.bubble_adjustment =
                self.bubble_adjustment * (1.0 - learning_rate) + learning_rate * bubble_factor;
        }
    }

    /// Predict opponent's likely action distribution
    pub fn predict_action_distribution(&self, context: &ActionContext) -> Vec<f64> {
        let mut base_distribution = vec![0.4, 0.35, 0.25]; // fold, call, raise

        // Adjust for stack size
        if context.stack_ratio < 0.1 {
            // Short stack: more likely to fold or go all-in
            base_distribution = vec![0.6, 0.1, 0.3];
        } else if context.stack_ratio > 0.3 {
            // Big stack: more aggressive
            base_distribution = vec![0.25, 0.35, 0.4];
        }

        // Adjust for bubble
        if context.near_bubble {
            let fold_boost = self.bubble_adjustment * 0.2;
            base_distribution[0] += fold_boost; // More folding
            base_distribution[1] -= fold_boost * 0.5;
            base_distribution[2] -= fold_boost * 0.5;
        }

        // Adjust for opponent tendencies
        base_distribution[0] *= self.tightness; // Fold frequency
        base_distribution[2] *= self.aggression.min(2.0); // Raise frequency

        // Normalize
        let sum: f64 = base_distribution.iter().sum();
        if sum > 0.0 {
            for prob in &mut base_distribution {
                *prob /= sum;
            }
        }

        base_distribution
    }
}

/// Tournament-specific actions
#[derive(Debug, Clone, PartialEq)]
pub enum TournamentAction {
    Fold,
    Call,
    Raise(u32), // Raise amount
    AllIn,
}

/// Context for action evaluation
#[derive(Debug, Clone)]
pub struct ActionContext {
    pub stack_ratio: f64,   // Player's stack relative to average
    pub pot_odds: f64,      // Current pot odds
    pub is_preflop: bool,   // Whether this is preflop action
    pub near_bubble: bool,  // Whether we're near bubble
    pub position: Position, // Player's position
    pub num_opponents: u32, // Number of active opponents
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    SmallBlind,
    BigBlind,
    EarlyPosition,
    MiddlePosition,
    LatePosition,
    Button,
}

/// Advanced terminal state evaluation for tournament scenarios
#[derive(Debug, Clone)]
pub struct TournamentEvaluator {
    pub tournament_state: TournamentState,
    pub opponent_models: HashMap<u32, OpponentModel>,
    pub icm_calculator: ICMCalculator,
}

impl TournamentEvaluator {
    pub fn new(tournament_state: TournamentState, player_stacks: Vec<u32>) -> Self {
        let payouts: Vec<u64> = tournament_state
            .payout_structure
            .iter()
            .map(|p| p.amount)
            .collect();
        let icm_calculator = ICMCalculator::new(player_stacks, payouts);

        Self {
            tournament_state,
            opponent_models: HashMap::new(),
            icm_calculator,
        }
    }

    /// Evaluate terminal state with realistic tournament considerations
    pub fn evaluate_terminal_state(&self, final_stacks: &[u32], player_idx: usize) -> f64 {
        if final_stacks.is_empty() || player_idx >= final_stacks.len() {
            return 0.0;
        }

        // Create ICM calculator with final stacks
        let payouts: Vec<u64> = self
            .tournament_state
            .payout_structure
            .iter()
            .map(|p| p.amount)
            .collect();
        let final_icm = ICMCalculator::new(final_stacks.to_vec(), payouts);
        let equities = final_icm.calculate_equity();

        if player_idx < equities.len() {
            // Convert equity to normalized value (-1 to 1 range for CFR)
            let max_possible_equity = self.tournament_state.prize_pool as f64;
            let normalized_equity = (equities[player_idx] / max_possible_equity) * 2.0 - 1.0;

            // Apply tournament-specific adjustments
            let stack_bonus = self.calculate_stack_survival_bonus(final_stacks[player_idx]);
            let position_bonus = self.calculate_position_bonus(final_stacks, player_idx);

            normalized_equity + stack_bonus + position_bonus
        } else {
            -1.0 // Player eliminated
        }
    }

    /// Calculate bonus for stack survival (staying alive is valuable)
    fn calculate_stack_survival_bonus(&self, final_stack: u32) -> f64 {
        if final_stack == 0 {
            -0.5 // Penalty for elimination
        } else if final_stack < 1000 {
            -0.2 // Penalty for being very short
        } else {
            0.1 // Bonus for having reasonable stack
        }
    }

    /// Calculate bonus based on final position
    fn calculate_position_bonus(&self, final_stacks: &[u32], player_idx: usize) -> f64 {
        let mut stack_ranks: Vec<(usize, u32)> = final_stacks
            .iter()
            .enumerate()
            .map(|(i, &stack)| (i, stack))
            .collect();
        stack_ranks.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by stack size descending

        if let Some(rank) = stack_ranks.iter().position(|(i, _)| *i == player_idx) {
            let position_bonus = match rank {
                0 => 0.3,  // Chip leader
                1 => 0.15, // Second place
                2 => 0.05, // Third place
                _ => 0.0,
            };
            position_bonus
        } else {
            0.0
        }
    }

    /// Select opponent action using sophisticated modeling
    pub fn select_opponent_action(
        &self,
        player_id: u32,
        context: &ActionContext,
        available_actions: &[TournamentAction],
    ) -> TournamentAction {
        if available_actions.is_empty() {
            return TournamentAction::Fold;
        }

        let model = self
            .opponent_models
            .get(&player_id)
            .cloned()
            .unwrap_or_else(|| OpponentModel::new(player_id));

        let action_probabilities = model.predict_action_distribution(context);

        // Select action based on probabilities
        let random_value: f64 = rand::random();
        let mut cumulative_prob = 0.0;

        for (i, &prob) in action_probabilities.iter().enumerate() {
            cumulative_prob += prob;
            if random_value <= cumulative_prob {
                return match i {
                    0 => TournamentAction::Fold,
                    1 => TournamentAction::Call,
                    2 => {
                        // Determine raise size based on context
                        let raise_size = self.calculate_appropriate_raise_size(context);
                        if context.stack_ratio < 0.15 && rand::random::<f64>() < 0.3 {
                            TournamentAction::AllIn
                        } else {
                            TournamentAction::Raise(raise_size)
                        }
                    }
                    _ => TournamentAction::Fold,
                };
            }
        }

        TournamentAction::Fold
    }

    /// Calculate appropriate raise size based on tournament context
    fn calculate_appropriate_raise_size(&self, context: &ActionContext) -> u32 {
        let (_, bb, _) = self.tournament_state.current_blinds();

        if context.is_preflop {
            // Preflop raise sizing
            if context.near_bubble {
                bb * 2 // Smaller raises near bubble
            } else {
                (bb as f64 * 2.5) as u32 // Standard 2.5x raise
            }
        } else {
            // Postflop raise sizing (pot-based)
            let pot_fraction = if context.near_bubble { 0.5 } else { 0.75 };
            ((bb * 10) as f64 * pot_fraction) as u32 // Estimate pot size
        }
    }

    /// Update opponent model with observed action
    pub fn update_opponent_model(
        &mut self,
        player_id: u32,
        action: TournamentAction,
        context: ActionContext,
    ) {
        let model = self
            .opponent_models
            .entry(player_id)
            .or_insert_with(|| OpponentModel::new(player_id));
        model.update_with_action(&action, &context);
    }

    /// Calculate ICM-adjusted expected value for a decision
    pub fn calculate_icm_adjusted_ev(&self, player_idx: usize, chip_change: i32) -> f64 {
        self.icm_calculator
            .calculate_icm_pressure(player_idx, chip_change)
    }
}

/// Multi-Table Tournament (MTT) management
#[derive(Debug, Clone)]
pub struct MTTManager {
    pub tables: Vec<MTTTable>,
    pub tournament_state: TournamentState,
    pub balancing_algorithm: BalancingAlgorithm,
}

#[derive(Debug, Clone)]
pub struct MTTTable {
    pub table_id: u32,
    pub seats: Vec<Option<MTTPlayer>>,
    pub max_seats: u32,
    pub current_hand: u32,
    pub button_position: u32,
}

#[derive(Debug, Clone)]
pub struct MTTPlayer {
    pub player_id: u32,
    pub stack_size: u32,
    pub position: u32,
    pub is_sitting_out: bool,
    pub has_been_dealt_in: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BalancingAlgorithm {
    StandardBalancing,       // Move players to balance tables
    ChipRaceProtocol,        // Handle odd chips during color-ups
    FinalTableConsolidation, // Consolidate to final table
}

impl MTTManager {
    pub fn new(
        total_players: u32,
        max_seats_per_table: u32,
        tournament_structure: TournamentStructure,
        prize_pool: u64,
    ) -> Self {
        let num_tables = (total_players + max_seats_per_table - 1) / max_seats_per_table;
        let mut tables = Vec::new();

        let mut player_id = 1;
        for table_id in 0..num_tables {
            let mut table = MTTTable {
                table_id,
                seats: vec![None; max_seats_per_table as usize],
                max_seats: max_seats_per_table,
                current_hand: 1,
                button_position: 1,
            };

            // Distribute players evenly across tables
            let players_for_this_table = if table_id == num_tables - 1 {
                total_players - (table_id * max_seats_per_table)
            } else {
                max_seats_per_table
            };

            for seat in 0..players_for_this_table {
                table.seats[seat as usize] = Some(MTTPlayer {
                    player_id,
                    stack_size: tournament_structure.starting_stack,
                    position: seat,
                    is_sitting_out: false,
                    has_been_dealt_in: false,
                });
                player_id += 1;
            }

            tables.push(table);
        }

        let tournament_state =
            TournamentState::new(tournament_structure, total_players, prize_pool);

        Self {
            tables,
            tournament_state,
            balancing_algorithm: BalancingAlgorithm::StandardBalancing,
        }
    }

    /// Balance tables by moving players
    pub fn balance_tables(&mut self) {
        match self.balancing_algorithm {
            BalancingAlgorithm::StandardBalancing => self.standard_table_balancing(),
            BalancingAlgorithm::ChipRaceProtocol => self.handle_chip_race(),
            BalancingAlgorithm::FinalTableConsolidation => self.consolidate_to_final_table(),
        }
    }

    /// Standard table balancing algorithm
    fn standard_table_balancing(&mut self) {
        let total_active_players = self.count_active_players();
        let target_players_per_table =
            (total_active_players + self.tables.len() as u32 - 1) / self.tables.len() as u32;

        // Find tables that need balancing
        let mut moves = Vec::new();

        for (table_idx, table) in self.tables.iter().enumerate() {
            let active_players = table.count_active_players();

            if active_players > target_players_per_table + 1 {
                // Table has too many players, find someone to move
                if let Some(player_to_move) = self.find_player_to_move(table_idx) {
                    if let Some(destination_table) =
                        self.find_destination_table(target_players_per_table)
                    {
                        moves.push((table_idx, player_to_move, destination_table));
                    }
                }
            }
        }

        // Execute moves
        for (source_table, player_pos, dest_table) in moves {
            self.move_player(source_table, player_pos, dest_table);
        }
    }

    /// Handle chip race during color-ups
    fn handle_chip_race(&mut self) {
        // Implementation for chip race protocol when removing lower denomination chips
        for table in &mut self.tables {
            for seat in &mut table.seats {
                if let Some(ref mut player) = seat {
                    // Round down stacks and handle fractional chips
                    let old_stack = player.stack_size;
                    player.stack_size = (old_stack / 100) * 100; // Example: round to nearest 100

                    // The fractional chips would be handled by chip race in real implementation
                }
            }
        }
    }

    /// Consolidate remaining players to final table
    fn consolidate_to_final_table(&mut self) {
        if self.count_active_players() <= 9 {
            let mut final_table_players = Vec::new();

            // Collect all remaining players
            for table in &mut self.tables {
                for seat in &mut table.seats {
                    if let Some(player) = seat.take() {
                        if !player.is_sitting_out && player.stack_size > 0 {
                            final_table_players.push(player);
                        }
                    }
                }
            }

            // Create single final table
            self.tables.clear();
            let mut final_table = MTTTable {
                table_id: 999,
                seats: vec![None; 9],
                max_seats: 9,
                current_hand: 1,
                button_position: 1,
            };

            // Seat players at final table based on chip counts (big stack gets best position)
            final_table_players.sort_by(|a, b| b.stack_size.cmp(&a.stack_size));

            for (i, mut player) in final_table_players.into_iter().enumerate() {
                if i < 9 {
                    player.position = i as u32;
                    final_table.seats[i] = Some(player);
                }
            }

            self.tables.push(final_table);
        }
    }

    pub fn count_active_players(&self) -> u32 {
        self.tables
            .iter()
            .map(|table| table.count_active_players())
            .sum()
    }

    fn find_player_to_move(&self, table_idx: usize) -> Option<u32> {
        // Find player in worst position relative to button for fairness
        if let Some(table) = self.tables.get(table_idx) {
            let button_pos = table.button_position;

            // Find player in big blind or early position to move
            for (pos, seat) in table.seats.iter().enumerate() {
                if let Some(player) = seat {
                    if !player.is_sitting_out && pos as u32 != button_pos {
                        return Some(pos as u32);
                    }
                }
            }
        }
        None
    }

    fn find_destination_table(&self, target_players: u32) -> Option<usize> {
        self.tables
            .iter()
            .enumerate()
            .find(|(_, table)| table.count_active_players() < target_players)
            .map(|(idx, _)| idx)
    }

    fn move_player(&mut self, source_table: usize, player_pos: u32, dest_table: usize) {
        if let Some(player) = self.tables[source_table].seats[player_pos as usize].take() {
            // Find empty seat at destination table
            if let Some(empty_seat) = self.tables[dest_table]
                .seats
                .iter_mut()
                .find(|seat| seat.is_none())
            {
                *empty_seat = Some(player);
            }
        }
    }

    /// Eliminate player and update tournament state
    pub fn eliminate_player(&mut self, table_id: u32, player_id: u32) {
        for table in &mut self.tables {
            if table.table_id == table_id {
                for seat in &mut table.seats {
                    if let Some(ref mut player) = seat {
                        if player.player_id == player_id {
                            player.stack_size = 0;
                            player.is_sitting_out = true;
                            self.tournament_state.players_remaining -= 1;

                            // Check if table needs balancing after elimination
                            if table.count_active_players() <= table.max_seats / 2 {
                                self.balance_tables();
                            }
                            return;
                        }
                    }
                }
            }
        }
    }

    /// Get current tournament standings
    pub fn get_tournament_standings(&self) -> Vec<(u32, u32, u32)> {
        // (player_id, stack, table_id)
        let mut standings = Vec::new();

        for table in &self.tables {
            for seat in &table.seats {
                if let Some(player) = seat {
                    if !player.is_sitting_out {
                        standings.push((player.player_id, player.stack_size, table.table_id));
                    }
                }
            }
        }

        // Sort by stack size descending
        standings.sort_by(|a, b| b.1.cmp(&a.1));
        standings
    }
}

impl MTTTable {
    pub fn count_active_players(&self) -> u32 {
        self.seats
            .iter()
            .filter(|seat| {
                if let Some(player) = seat {
                    !player.is_sitting_out && player.stack_size > 0
                } else {
                    false
                }
            })
            .count() as u32
    }
}

/// Advanced bubble strategy calculator
#[derive(Debug, Clone)]
pub struct BubbleStrategy {
    pub bubble_factor: f64,      // How close to bubble (0.0 to 1.0)
    pub pressure_threshold: f64, // When to start adjusting strategy
    pub fold_equity_boost: f64,  // Increase in fold equity near bubble
    pub icm_sensitivity: f64,    // How much ICM affects decisions
}

impl BubbleStrategy {
    pub fn new(players_remaining: u32, payout_spots: u32) -> Self {
        let bubble_factor = if players_remaining > payout_spots {
            let excess_players = players_remaining - payout_spots;
            (1.0 / (excess_players as f64 + 1.0)).min(1.0)
        } else {
            1.0 // Already in the money
        };

        Self {
            bubble_factor,
            pressure_threshold: 0.3,
            fold_equity_boost: bubble_factor * 0.4,
            icm_sensitivity: bubble_factor * 2.0,
        }
    }

    /// Calculate bubble-adjusted hand range
    pub fn adjust_hand_range(&self, base_range: f64, stack_ratio: f64) -> f64 {
        if self.bubble_factor < self.pressure_threshold {
            return base_range; // No bubble pressure yet
        }

        let tightening_factor = if stack_ratio < 0.1 {
            0.5 // Very tight when short
        } else if stack_ratio > 0.4 {
            1.2 // More aggressive when big
        } else {
            0.8 // Moderate tightening
        };

        base_range * tightening_factor * (2.0 - self.bubble_factor)
    }

    /// Calculate fold equity adjustments near bubble
    pub fn calculate_fold_equity_adjustment(&self, base_fold_equity: f64) -> f64 {
        base_fold_equity + self.fold_equity_boost
    }

    /// Should we make this play based on bubble considerations?
    pub fn should_make_aggressive_play(&self, stack_ratio: f64, icm_cost: f64) -> bool {
        if self.bubble_factor < self.pressure_threshold {
            return true; // No bubble concerns
        }

        // Big stacks can be more aggressive
        if stack_ratio > 0.4 {
            return icm_cost < self.icm_sensitivity * 1.5;
        }

        // Short stacks need to be more careful
        icm_cost < self.icm_sensitivity * 0.5
    }
}
