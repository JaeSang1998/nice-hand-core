//! Web API Module
//!
//! This module provides HTTP/WebSocket APIs for poker strategy evaluation:
//! - Simple stateless API for quick strategy queries
//! - Full-featured API with state tracking and batch processing

pub mod web_api;
pub mod web_api_simple;

// Re-export selected types to avoid conflicts
pub use web_api::{OfflineTrainer, PokerWebAPI, StrategyTable};
pub use web_api_simple::QuickPokerAPI;
