//! 웹 API 모듈
//!
//! 이 모듈은 포커 전략 평가를 위한 HTTP/WebSocket API들을 제공합니다:
//! - 빠른 전략 쿼리를 위한 간단한 무상태 API
//! - 상태 추적 및 배치 처리가 가능한 완전 기능 API
//! - 고급 분석 및 EV 계산 API

pub mod web_api;
pub mod web_api_simple;
pub mod analysis;

// 충돌을 피하기 위해 선택된 타입들을 재수출
pub use web_api::{OfflineTrainer, PokerWebAPI, StrategyTable};
pub use analysis::{analyze_poker_state, get_on_demand_ev_analysis, AnalysisRequest, PokerAnalysisResponse};
pub use web_api_simple::QuickPokerAPI;
