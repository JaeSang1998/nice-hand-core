//! CFR 솔버 모듈
//!
//! 이 모듈은 반사실적 후회 최소화 알고리즘들을 포함합니다:
//! - Game 트레잇과 함께하는 핵심 CFR 구현
//! - 대규모 게임 트리를 위한 몬테카를로 CFR
//! - 학습 및 전략 계산

pub mod cfr_core;
pub mod ev_calculator;
pub mod mccfr;

#[cfg(test)]
mod ev_calculator_tests;

// 자주 사용되는 타입들을 재수출
pub use cfr_core::*;
pub use mccfr::*;
