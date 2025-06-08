//! CFR Solver Module
//!
//! This module contains Counterfactual Regret Minimization algorithms:
//! - Core CFR implementation with Game trait
//! - Monte Carlo CFR for large game trees
//! - Training and strategy computation

pub mod cfr_core;
pub mod ev_calculator;
pub mod mccfr;

#[cfg(test)]
mod ev_calculator_tests;

// Re-export commonly used types
pub use cfr_core::*;
pub use mccfr::*;
