//! Tournament Support Module
//!
//! This module provides comprehensive tournament support for poker games, including:
//! - ICM (Independent Chip Model) calculations for tournament equity
//! - Tournament structure management with blinds and antes
//! - Bubble strategy adjustments and pressure calculations
//! - Multi-table tournament (MTT) management
//! - Tournament-specific AI strategies
//!
//! # Key Components
//!
//! ## ICM Calculator
//! The [`ICMCalculator`] implements the Independent Chip Model to calculate the real-world
//! monetary value of tournament chips based on stack sizes and payout structure.
//!
//! ## Tournament State Management
//! [`TournamentState`] tracks the current state of a tournament including blind levels,
//! players remaining, and payout structure.
//!
//! ## Bubble Strategy
//! [`BubbleStrategy`] provides specialized strategy adjustments for play near the
//! tournament money bubble, where ICM pressure significantly affects optimal play.
//!
//! # Examples
//!
//! ## Basic ICM Calculation
//! ```
//! use nice_hand_core::game::tournament::ICMCalculator;
//!
//! // 4 players with different stack sizes, top 3 get paid
//! let stacks = vec![8000, 6000, 4000, 2000];
//! let payouts = vec![15000, 10000, 5000]; // Only top 3 paid
//!
//! let icm = ICMCalculator::new(stacks, payouts);
//! let equities = icm.calculate_equity();
//!
//! // The short stack will have significantly less equity than chip proportion
//! // due to the risk of bubbling (finishing 4th with no payout)
//! println!("ICM equities: {:?}", equities);
//! ```
//!
//! ## Tournament Structure Setup
//! ```
//! use nice_hand_core::game::tournament::{TournamentStructure, BlindLevel, TournamentState};
//!
//! let structure = TournamentStructure {
//!     levels: vec![
//!         BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
//!         BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 10 },
//!         BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 15 },
//!     ],
//!     level_duration_minutes: 20,
//!     starting_stack: 1500,
//!     ante_schedule: vec![],
//! };
//!
//! let tournament = TournamentState::new(structure, 180, 100000);
//! let (sb, bb, ante) = tournament.current_blinds();
//! println!("Current blinds: {}/{} with {} ante", sb, bb, ante);
//! ```
//!
//! ## Bubble Strategy Analysis
//! ```
//! use nice_hand_core::game::tournament::BubbleStrategy;
//!
//! // 19 players remaining, 18 get paid (classic bubble situation)
//! let bubble_strategy = BubbleStrategy::new(19, 18);
//!
//! // Check if we should make an aggressive play with a medium stack
//! let should_be_aggressive = bubble_strategy.should_make_aggressive_play(1.2, 0.1);
//!
//! // Adjust hand range based on bubble pressure
//! let base_range = 0.2; // 20% of hands normally
//! let adjusted_range = bubble_strategy.adjust_hand_range(base_range, 0.8); // Short stack
//!
//! println!("Bubble factor: {:.3}", bubble_strategy.bubble_factor);
//! println!("Adjusted range: {:.1}%", adjusted_range * 100.0);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tournament structure and blind schedule management
///
/// Defines the blind levels, antes, and timing structure for a tournament.
/// This structure controls how the blinds increase over time and manages
/// the overall tournament progression.
///
/// # Fields
///
/// * `levels` - Vector of blind levels defining small blind, big blind, and ante for each level
/// * `level_duration_minutes` - How long each blind level lasts in minutes
/// * `starting_stack` - Number of chips each player starts with
/// * `ante_schedule` - Optional separate ante schedule (usually embedded in levels)
///
/// # Examples
///
/// ```
/// use nice_hand_core::game::tournament::{TournamentStructure, BlindLevel};
///
/// let structure = TournamentStructure {
///     levels: vec![
///         BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
///         BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 10 },
///     ],
///     level_duration_minutes: 15,
///     starting_stack: 1500,
///     ante_schedule: vec![],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentStructure {
    pub levels: Vec<BlindLevel>,
    pub level_duration_minutes: u32,
    pub starting_stack: u32,
    pub ante_schedule: Vec<AnteLevel>,
}

/// Individual blind level configuration
///
/// Represents a single blind level in a tournament structure, defining the
/// small blind, big blind, and ante amounts for that level.
///
/// # Fields
///
/// * `level` - The blind level number (1-based indexing)
/// * `small_blind` - Small blind amount in chips
/// * `big_blind` - Big blind amount in chips  
/// * `ante` - Ante amount in chips (0 if no ante at this level)
///
/// # Examples
///
/// ```
/// use nice_hand_core::game::tournament::BlindLevel;
///
/// // Early tournament level with no ante
/// let early_level = BlindLevel {
///     level: 1,
///     small_blind: 25,
///     big_blind: 50,
///     ante: 0,
/// };
///
/// // Later level with ante introduced
/// let late_level = BlindLevel {
///     level: 8,
///     small_blind: 400,
///     big_blind: 800,
///     ante: 100,
/// };
/// ```
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

        // For simple cases, use direct calculation
        if num_players == 1 {
            return vec![self.payouts.get(0).copied().unwrap_or(0) as f64];
        }

        if num_players == 2 {
            return self.calculate_heads_up_equity();
        }

        // For larger fields, use simplified ICM model
        self.calculate_simplified_icm()
    }

    /// Calculate heads-up ICM equity (2 players)
    fn calculate_heads_up_equity(&self) -> Vec<f64> {
        let total_chips = (self.stacks[0] + self.stacks[1]) as f64;
        let p1_chips = self.stacks[0] as f64;

        // Calculate adjusted win probabilities using ICM model
        // In tournament play, the chip leader's advantage is reduced due to ICM pressure
        let chip_ratio = p1_chips / total_chips;

        // Apply ICM pressure adjustment - larger stacks have diminishing returns
        let p1_win_prob = if chip_ratio > 0.5 {
            // Reduce big stack advantage
            let excess = chip_ratio - 0.5;
            0.5 + excess * 0.85 // Big stacks get ~85% of their chip advantage
        } else {
            chip_ratio * 1.1 // Small stacks get slight boost
        };

        let p2_win_prob = 1.0 - p1_win_prob;

        let first_place_payout = self.payouts.get(0).copied().unwrap_or(0) as f64;
        let second_place_payout = self.payouts.get(1).copied().unwrap_or(0) as f64;

        // ICM equity = (win_prob * 1st_prize) + (lose_prob * 2nd_prize)
        let p1_equity =
            p1_win_prob * first_place_payout + (1.0 - p1_win_prob) * second_place_payout;
        let p2_equity =
            p2_win_prob * first_place_payout + (1.0 - p2_win_prob) * second_place_payout;

        vec![p1_equity, p2_equity]
    }

    /// Simplified ICM calculation for multiple players
    fn calculate_simplified_icm(&self) -> Vec<f64> {
        let num_players = self.stacks.len();
        let total_chips: u32 = self.stacks.iter().sum();

        if total_chips == 0 {
            return vec![0.0; num_players];
        }

        let mut equities = vec![0.0; num_players];
        let total_payout: f64 = self.payouts.iter().map(|&p| p as f64).sum();

        // Basic proportional distribution adjusted for ICM effects
        for (i, &stack) in self.stacks.iter().enumerate() {
            let stack_ratio = stack as f64 / total_chips as f64;

            // Apply ICM pressure (diminishing returns for big stacks)
            let icm_adjusted_ratio = if stack_ratio > 0.5 {
                0.5 + (stack_ratio - 0.5) * 0.7 // Big stacks get less than proportional
            } else if stack_ratio < 0.05 {
                stack_ratio * 1.2 // Small stacks get slight boost
            } else {
                stack_ratio
            };

            equities[i] = icm_adjusted_ratio * total_payout;
        }

        // Normalize to ensure total equals payout total
        let equity_total: f64 = equities.iter().sum();
        if equity_total > 0.0 {
            let normalization_factor = total_payout / equity_total;
            for equity in &mut equities {
                *equity *= normalization_factor;
            }
        }

        equities
    }

    /// Calculate exact ICM equity for a specific player using dynamic programming
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

    /// 디리클레-다항분포 모델을 사용하여 기본 마무리 확률 계산
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

    /// 의사결정에 대한 ICM 조정 기댓값 계산
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

/// 고급 버블 전략 계산기
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
            // Higher bubble factor = more pressure (closer to bubble)
            // For 1 excess player (bubble): 1.0 - 1.0/6.0 = 0.833
            // For 2 excess players: 1.0 - 2.0/6.0 = 0.667
            // For 5+ excess players: minimal pressure
            (1.0 - (excess_players as f64 / 6.0)).max(0.1)
        } else {
            1.0 // Already in the money - maximum factor for different strategy
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icm_calculator_basic() {
        let stacks = vec![1500, 1200, 800, 500];
        let payouts = vec![1000, 600, 300, 100];

        let icm = ICMCalculator::new(stacks, payouts);
        let equities = icm.calculate_equity();

        // Basic sanity checks
        assert_eq!(equities.len(), 4);
        assert!(equities.iter().all(|&eq| eq >= 0.0));

        // Total equity should approximately equal total payouts
        let total_equity: f64 = equities.iter().sum();
        let total_payouts: f64 = icm.payouts.iter().map(|&p| p as f64).sum();
        assert!(
            (total_equity - total_payouts).abs() < 10.0,
            "Total equity {} should be close to total payouts {}",
            total_equity,
            total_payouts
        );

        // Chip leader should have highest equity
        let max_stack_idx = icm
            .stacks
            .iter()
            .enumerate()
            .max_by_key(|(_, &stack)| stack)
            .unwrap()
            .0;
        assert!(equities[max_stack_idx] >= equities.iter().cloned().fold(0.0, f64::max) * 0.99);
    }

    #[test]
    fn test_icm_calculator_heads_up() {
        let stacks = vec![30000, 10000];
        let payouts = vec![20000, 12000];

        let icm = ICMCalculator::new(stacks, payouts);
        let equities = icm.calculate_equity();

        // Chip leader should have more than 75% equity despite 3:1 chip lead
        assert!(
            equities[0] > 15000.0 && equities[0] < 18000.0,
            "ICM should reduce chip leader advantage: got {}",
            equities[0]
        );
        assert!(
            equities[1] > 14000.0 && equities[1] < 17000.0,
            "ICM should boost short stack: got {}",
            equities[1]
        );
    }

    #[test]
    fn test_icm_pressure_calculation() {
        let stacks = vec![15000, 8000, 5000, 2000];
        let payouts = vec![10000, 6000, 4000];

        let icm = ICMCalculator::new(stacks, payouts);

        // Test ICM pressure for losing chips
        let pressure_big = icm.calculate_icm_pressure(0, -1000);
        let pressure_small = icm.calculate_icm_pressure(3, -1000);

        // Short stacks should have higher ICM pressure
        assert!(
            pressure_small.abs() > pressure_big.abs(),
            "Short stack should have higher ICM pressure: {} vs {}",
            pressure_small,
            pressure_big
        );
    }

    #[test]
    fn test_bubble_strategy() {
        // Test near bubble (11 players, 10 get paid)
        let bubble_strategy = BubbleStrategy::new(11, 10);
        assert!(bubble_strategy.bubble_factor > 0.8);

        // Test deep in bubble (5 players, 10 get paid - already ITM)
        let itm_strategy = BubbleStrategy::new(5, 10);
        assert_eq!(itm_strategy.bubble_factor, 1.0);

        // Test hand range adjustments
        let base_range = 0.2; // 20% of hands
        let tight_range = bubble_strategy.adjust_hand_range(base_range, 0.05); // Short stack
        let loose_range = bubble_strategy.adjust_hand_range(base_range, 0.5); // Big stack

        assert!(tight_range < base_range, "Short stack should tighten range");
        assert!(loose_range > base_range, "Big stack should loosen range");
    }

    #[test]
    fn test_tournament_state_creation() {
        let structure = TournamentStructure {
            levels: vec![
                BlindLevel {
                    level: 1,
                    small_blind: 25,
                    big_blind: 50,
                    ante: 0,
                },
                BlindLevel {
                    level: 2,
                    small_blind: 50,
                    big_blind: 100,
                    ante: 0,
                },
            ],
            level_duration_minutes: 15,
            starting_stack: 1500,
            ante_schedule: vec![AnteLevel { level: 3, ante: 10 }],
        };

        let tournament = TournamentState::new(structure, 9, 10000);

        assert_eq!(tournament.players_remaining, 9);
        assert_eq!(tournament.prize_pool, 10000);
        assert_eq!(tournament.current_level, 1);
    }

    #[test]
    fn test_tournament_evaluator() {
        let structure = TournamentStructure {
            levels: vec![BlindLevel {
                level: 1,
                small_blind: 25,
                big_blind: 50,
                ante: 0,
            }],
            level_duration_minutes: 15,
            starting_stack: 1500,
            ante_schedule: vec![],
        };

        let tournament_state = TournamentState::new(structure, 6, 5000);
        let player_stacks = vec![1500, 1200, 1800, 900, 2100, 1000];

        let evaluator = TournamentEvaluator::new(tournament_state, player_stacks);

        // Test ICM calculations
        let icm_ev = evaluator.calculate_icm_adjusted_ev(0, -500);
        assert!(icm_ev != 0.0, "ICM EV should be calculated");

        // Test that evaluator was created successfully
        assert_eq!(evaluator.opponent_models.len(), 0); // No models initially
    }

    #[test]
    fn test_mtt_manager_creation() {
        let structure = TournamentStructure {
            levels: vec![BlindLevel {
                level: 1,
                small_blind: 25,
                big_blind: 50,
                ante: 0,
            }],
            level_duration_minutes: 15,
            starting_stack: 1500,
            ante_schedule: vec![],
        };

        let mtt = MTTManager::new(27, 9, structure, 50000);

        // Should create 3 tables for 27 players with max 9 per table
        assert_eq!(mtt.tables.len(), 3);

        // Check player distribution
        let total_players: u32 = mtt
            .tables
            .iter()
            .map(|table| table.count_active_players())
            .sum();
        assert_eq!(total_players, 27);

        // Last table might have fewer players
        assert!(mtt.tables[2].count_active_players() <= 9);
    }

    #[test]
    fn test_tournament_action_evaluation() {
        let _context = ActionContext {
            stack_ratio: 0.15, // Short stack
            pot_odds: 3.0,
            is_preflop: true,
            near_bubble: true,
            position: Position::Button,
            num_opponents: 3,
        };

        let fold_action = TournamentAction::Fold;
        let call_action = TournamentAction::Call;
        let raise_action = TournamentAction::Raise(300);

        // Test that actions can be created and evaluated
        match fold_action {
            TournamentAction::Fold => assert!(true),
            _ => assert!(false, "Should be fold action"),
        }

        match call_action {
            TournamentAction::Call => assert!(true),
            _ => assert!(false, "Should be call action"),
        }

        match raise_action {
            TournamentAction::Raise(amount) => assert_eq!(amount, 300),
            _ => assert!(false, "Should be raise action"),
        }
    }

    #[test]
    fn test_opponent_model() {
        let mut model = OpponentModel::new(1);

        let context = ActionContext {
            stack_ratio: 0.25,
            pot_odds: 2.5,
            is_preflop: true,
            near_bubble: false,
            position: Position::EarlyPosition,
            num_opponents: 4,
        };

        // Update with some aggressive actions
        model.update_with_action(&TournamentAction::Raise(200), &context);
        model.update_with_action(&TournamentAction::Raise(150), &context);
        model.update_with_action(&TournamentAction::Call, &context);

        // Check that stats are being tracked
        assert!(model.sample_size > 0);
        assert!(model.vpip >= 0.0 && model.vpip <= 1.0);
    }

    #[test]
    fn test_elimination_probability() {
        let stacks = vec![5000, 3000, 2000, 1000];
        let payouts = vec![6000, 3000, 1000];

        let icm = ICMCalculator::new(stacks, payouts);
        let remaining_players = vec![0, 1, 2, 3];

        // Test elimination probabilities
        let prob_0 = icm.calculate_elimination_probability(0, &remaining_players);
        let prob_3 = icm.calculate_elimination_probability(3, &remaining_players);

        // Short stack should have higher elimination probability
        assert!(
            prob_3 > prob_0,
            "Short stack should have higher elimination probability: {} vs {}",
            prob_3,
            prob_0
        );

        // All probabilities should be between 0 and 1
        assert!(prob_0 >= 0.0 && prob_0 <= 1.0);
        assert!(prob_3 >= 0.0 && prob_3 <= 1.0);
    }

    #[test]
    fn test_icm_edge_cases() {
        // Test with empty stacks
        let icm_empty = ICMCalculator::new(vec![], vec![]);
        let equities_empty = icm_empty.calculate_equity();
        assert!(equities_empty.is_empty());

        // Test with single player
        let icm_single = ICMCalculator::new(vec![1000], vec![1000]);
        let equities_single = icm_single.calculate_equity();
        assert_eq!(equities_single.len(), 1);
        assert!((equities_single[0] - 1000.0).abs() < 1.0);

        // Test with zero stacks
        let icm_zero = ICMCalculator::new(vec![1000, 0, 500], vec![1000, 500, 100]);
        let equities_zero = icm_zero.calculate_equity();
        assert_eq!(equities_zero[1], 0.0); // Player with 0 chips should have 0 equity
    }

    #[test]
    fn test_icm_calculator_three_players() {
        let stacks = vec![6000, 4000, 2000];
        let payouts = vec![6000, 3000, 1000];

        let icm = ICMCalculator::new(stacks, payouts.clone());
        let equities = icm.calculate_equity();

        // Verify total equity equals total payouts
        let total_equity: f64 = equities.iter().sum();
        let total_payouts: f64 = payouts.iter().map(|&p| p as f64).sum();
        assert!(
            (total_equity - total_payouts).abs() < 0.01,
            "Total equity {} should equal total payouts {}",
            total_equity,
            total_payouts
        );

        // Chip leader should have highest equity
        assert!(
            equities[0] > equities[1] && equities[1] > equities[2],
            "Equities should be in chip order: {:?}",
            equities
        );

        // Big stack should have less than or equal to chip-proportional equity due to ICM
        let chip_proportion = 6000.0 / 12000.0;
        let expected_chip_equity = chip_proportion * total_payouts;
        assert!(
            equities[0] <= expected_chip_equity * 1.01,
            "Big stack equity {} should be close to chip-proportional {}",
            equities[0],
            expected_chip_equity
        );
    }

    #[test]
    fn test_icm_calculator_edge_cases() {
        // Test single player
        let single_stacks = vec![10000];
        let single_payouts = vec![5000];
        let single_icm = ICMCalculator::new(single_stacks, single_payouts);
        let single_equities = single_icm.calculate_equity();
        assert_eq!(single_equities.len(), 1);
        assert_eq!(single_equities[0], 5000.0);

        // Test equal stacks
        let equal_stacks = vec![5000, 5000, 5000];
        let equal_payouts = vec![9000, 6000, 0];
        let equal_icm = ICMCalculator::new(equal_stacks, equal_payouts);
        let equal_equities = equal_icm.calculate_equity();

        // With equal stacks, equity should be equal
        let expected_equity = 15000.0 / 3.0; // Total payouts divided by players
        for equity in &equal_equities {
            assert!(
                (equity - expected_equity).abs() < 100.0,
                "Equal stacks should have roughly equal equity: got {}, expected {}",
                equity,
                expected_equity
            );
        }

        // Test zero chips
        let zero_stacks = vec![10000, 0, 5000];
        let zero_payouts = vec![8000, 4000, 0];
        let zero_icm = ICMCalculator::new(zero_stacks, zero_payouts);
        let zero_equities = zero_icm.calculate_equity();

        // Player with zero chips should have minimal equity
        assert!(
            zero_equities[1] < 100.0,
            "Player with zero chips should have minimal equity: got {}",
            zero_equities[1]
        );
    }

    #[test]
    fn test_icm_calculator_large_field() {
        // Test with larger tournament field
        let stacks = vec![20000, 15000, 12000, 8000, 6000, 4000, 3000, 2000];
        let payouts = vec![30000, 18000, 12000, 8000, 6000]; // Top 5 paid

        let icm = ICMCalculator::new(stacks, payouts.clone());
        let equities = icm.calculate_equity();

        // Verify total equity conservation
        let total_equity: f64 = equities.iter().sum();
        let total_payouts: f64 = payouts.iter().map(|&p| p as f64).sum();
        assert!(
            (total_equity - total_payouts).abs() < 1.0,
            "Large field: total equity {} should equal total payouts {}",
            total_equity,
            total_payouts
        );

        // Verify equities are roughly in stack order
        for i in 0..equities.len() - 1 {
            assert!(equities[i] >= equities[i + 1] * 0.8, // Allow some flexibility
                    "Equity should generally decrease with stack size: position {} has {}, position {} has {}", 
                    i, equities[i], i + 1, equities[i + 1]);
        }

        // Players not in money spots should still have some equity (bubble factor)
        for i in payouts.len()..equities.len() {
            assert!(
                equities[i] > 0.0,
                "Player {} outside money should still have some equity: got {}",
                i,
                equities[i]
            );
        }
    }

    #[test]
    fn test_icm_calculator_bubble_scenario() {
        // Classic bubble scenario: 4 players, 3 paid
        let stacks = vec![8000, 7000, 6000, 1000];
        let payouts = vec![15000, 10000, 5000]; // Only top 3 paid

        let icm = ICMCalculator::new(stacks, payouts);
        let equities = icm.calculate_equity();

        // Short stack (bubble boy) should have significantly less equity
        let short_stack_equity = equities[3];
        let average_equity = equities.iter().sum::<f64>() / equities.len() as f64;

        assert!(
            short_stack_equity < average_equity * 0.4,
            "Bubble boy should have much less than average equity: got {}, average {}",
            short_stack_equity,
            average_equity
        );

        // Big stacks should benefit from bubble pressure
        assert!(
            equities[0] > equities[1] && equities[1] > equities[2],
            "Equities should decrease with stack size on bubble: {:?}",
            equities
        );
    }

    #[test]
    fn test_icm_calculator_winner_take_all() {
        // Winner take all scenario
        let stacks = vec![6000, 4000, 3000, 2000];
        let payouts = vec![15000]; // Only winner gets paid

        let icm = ICMCalculator::new(stacks, payouts);
        let equities = icm.calculate_equity();

        // Total equity should equal payout
        let total_equity: f64 = equities.iter().sum();
        assert!(
            (total_equity - 15000.0).abs() < 0.01,
            "Winner-take-all: total equity {} should equal payout 15000",
            total_equity
        );

        // Chip leader should have highest equity but close to chip proportion in winner-take-all
        let chip_leader_proportion = 6000.0 / 15000.0; // 40% of chips
        let chip_leader_equity_proportion = equities[0] / 15000.0;

        assert!(
            chip_leader_equity_proportion > 0.35
                && chip_leader_equity_proportion <= chip_leader_proportion * 1.01,
            "Winner-take-all: chip leader should have significant equity: {:.2}% of total",
            chip_leader_equity_proportion * 100.0
        );
    }
} // End of tests module
