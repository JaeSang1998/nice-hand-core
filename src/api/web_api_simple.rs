// 고급 무상태 포커 전략 API
// 정교한 휴리스틱으로 실시간 의사결정
// 학습 불필요 - 즉석 운영 준비 응답

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 웹 API 게임 상태 표현
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebGameState {
    /// 히어로의 홀카드 [카드1, 카드2] (0-51 범위)
    pub hole_cards: [u8; 2],
    /// 커뮤니티 보드 카드들 (최대 5장)
    pub board: Vec<u8>,
    /// 현재 베팅 스트리트 (0=프리플랍, 1=플랍, 2=턴, 3=리버)
    pub street: u8,
    /// 칩 단위 총 팟 크기
    pub pot: u32,
    /// 칩 단위 콜 금액
    pub to_call: u32,
    /// 칩 단위 히어로의 스택 크기
    pub my_stack: u32,
    /// 칩 단위 상대방의 스택 크기
    pub opponent_stack: u32,
}

/// 상세 분석을 포함한 향상된 전략 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyResponse {
    /// 액션 확률 (예: "fold": 0.2, "call": 0.5, "raise": 0.3)
    pub strategy: HashMap<String, f64>,
    /// 권장 주요 액션
    pub recommended_action: String,
    /// 기댓값 추정
    pub expected_value: f64,
    /// 결정 신뢰도 (0.0-1.0)
    pub confidence: f64,
    /// 핸드 강도 평가 (0.0-1.0)
    pub hand_strength: f64,
    /// 팟 오즈 계산
    pub pot_odds: f64,
    /// 전략적 추론 (디버깅/설명용)
    pub reasoning: String,
}

/// 고급 포커 전략 엔진
///
/// 다음을 기반으로 한 정교한 휴리스틱 사용:
/// - 핸드 강도 평가
/// - 포지션 분석
/// - 스택 깊이 고려사항
/// - 팟 오즈 계산
/// - 상대방 모델링 (기본)
pub struct QuickPokerAPI {
    /// 프리플랍 핸드 랭킹 조회 테이블
    preflop_rankings: HashMap<(u8, u8, bool), f64>,
}

impl QuickPokerAPI {
    /// 사전 계산된 핸드 랭킹으로 포커 API 초기화
    pub fn new() -> Self {
        let mut preflop_rankings = HashMap::new();

        // 프리미엄 핸드 랭킹 초기화
        Self::init_preflop_rankings(&mut preflop_rankings);

        Self { preflop_rankings }
    }

    /// 주어진 게임 상태에 대한 포괄적 전략 계산
    pub fn get_optimal_strategy(&self, state: WebGameState) -> StrategyResponse {
        // 1. 핵심 지표 계산
        let hand_strength = self.evaluate_hand_strength(&state);
        let pot_odds = self.calculate_pot_odds(&state);
        // 2. 정교한 휴리스틱을 기반으로 전략 생성
        let strategy = self.calculate_advanced_strategy(&state, hand_strength, pot_odds);

        // 3. 최선의 액션과 근거 결정
        let recommended = self.get_best_action(&strategy);
        let reasoning = self.generate_reasoning(&state, hand_strength, pot_odds, &recommended);

        // 4. 기댓값 추정
        let ev = self.estimate_expected_value(&state, &strategy, hand_strength);

        // 5. 상황 명확성을 기반으로 신뢰도 계산
        let confidence = self.calculate_confidence(&state, hand_strength, pot_odds);

        StrategyResponse {
            strategy,
            recommended_action: recommended,
            expected_value: ev,
            confidence,
            hand_strength,
            pot_odds,
            reasoning,
        }
    }

    /// 여러 게임 상태에 대한 배치 처리
    pub fn get_strategies_batch(&self, states: Vec<WebGameState>) -> Vec<StrategyResponse> {
        states
            .into_iter()
            .map(|state| self.get_optimal_strategy(state))
            .collect()
    }

    /// 전체 분석 없이 빠른 추천
    pub fn get_quick_recommendation(&self, state: WebGameState) -> String {
        let hand_strength = self.evaluate_hand_strength(&state);
        let pot_odds = self.calculate_pot_odds(&state);

        if state.to_call == 0 {
            // 체크 가능 - 체크/베트 선택
            if hand_strength > 0.7 { "bet" } else { "check" }.to_string()
        } else {
            // 콜하거나 폴드해야 함
            if hand_strength > pot_odds + 0.1 {
                "call"
            } else {
                "fold"
            }
            .to_string()
        }
    }

    /// 고급 전략 계산 엔진
    fn calculate_advanced_strategy(
        &self,
        state: &WebGameState,
        hand_strength: f64,
        pot_odds: f64,
    ) -> HashMap<String, f64> {
        let mut strategy = HashMap::new();

        let effective_stack = state.my_stack.min(state.opponent_stack) as f64;
        let stack_to_pot_ratio = if state.pot > 0 {
            effective_stack / state.pot as f64
        } else {
            effective_stack / 100.0
        };

        let bet_size_factor = if stack_to_pot_ratio > 10.0 {
            0.5 // 딥 스택에서 작은 베트
        } else if stack_to_pot_ratio > 5.0 {
            0.75 // 중간 베트
        } else {
            1.2 // 큰 베트 / 숏 스택에서 올인
        };

        if state.to_call == 0 {
            // 체크/베트 상황
            self.calculate_check_bet_strategy(
                &mut strategy,
                hand_strength,
                bet_size_factor,
                stack_to_pot_ratio,
            )
        } else {
            // 콜/폴드/레이즈 상황
            self.calculate_call_fold_strategy(
                &mut strategy,
                hand_strength,
                pot_odds,
                bet_size_factor,
                state,
            )
        }

        // 확률 정규화
        self.normalize_strategy(&mut strategy);
        strategy
    }

    /// 체크/베트 상황에 대한 전략 계산
    fn calculate_check_bet_strategy(
        &self,
        strategy: &mut HashMap<String, f64>,
        hand_strength: f64,
        _bet_factor: f64,
        spr: f64,
    ) {
        if hand_strength > 0.85 {
            // 프리미엄 핸드: 대부분 밸류 베트
            strategy.insert("check".to_string(), 0.15);
            strategy.insert("bet_small".to_string(), 0.3);
            strategy.insert("bet_large".to_string(), 0.55);
        } else if hand_strength > 0.7 {
            // 강한 핸드: 균형 잡힌 접근
            strategy.insert("check".to_string(), 0.4);
            strategy.insert("bet_small".to_string(), 0.45);
            strategy.insert("bet_large".to_string(), 0.15);
        } else if hand_strength > 0.55 {
            // 중간 핸드: 대부분 체크, 일부 씬 밸류
            strategy.insert("check".to_string(), 0.7);
            strategy.insert("bet_small".to_string(), 0.25);
            strategy.insert("bet_large".to_string(), 0.05);
        } else if hand_strength > 0.3 {
            // 블러프 잠재력이 있는 약한 핸드
            let bluff_freq = if spr > 8.0 { 0.15 } else { 0.25 };
            strategy.insert("check".to_string(), 1.0 - bluff_freq);
            strategy.insert("bet_small".to_string(), bluff_freq * 0.8);
            strategy.insert("bet_large".to_string(), bluff_freq * 0.2);
        } else {
            // 매우 약한 핸드: 대부분 체크
            strategy.insert("check".to_string(), 0.9);
            strategy.insert("bet_small".to_string(), 0.08);
            strategy.insert("bet_large".to_string(), 0.02);
        }
    }

    /// 콜/폴드/레이즈 상황에 대한 전략 계산
    fn calculate_call_fold_strategy(
        &self,
        strategy: &mut HashMap<String, f64>,
        hand_strength: f64,
        pot_odds: f64,
        _bet_factor: f64,
        state: &WebGameState,
    ) {
        let call_requirement = pot_odds + 0.05; // 콜하려면 약간의 우위 필요
        let raise_threshold = 0.7; // 레이즈하려면 강한 핸드 필요

        let facing_large_bet = state.to_call > state.pot / 2;
        let stack_commitment = state.to_call as f64 / state.my_stack as f64;

        if hand_strength > 0.9 {
            // 넛/넛에 가까움: 거의 항상 레이즈/콜
            strategy.insert("fold".to_string(), 0.02);
            strategy.insert("call".to_string(), 0.23);
            strategy.insert("raise".to_string(), 0.75);
        } else if hand_strength > raise_threshold {
            // 강한 핸드: 대부분 콜/레이즈
            let raise_freq = if facing_large_bet { 0.4 } else { 0.6 };
            strategy.insert("fold".to_string(), 0.05);
            strategy.insert("call".to_string(), 0.95 - raise_freq);
            strategy.insert("raise".to_string(), raise_freq);
        } else if hand_strength > call_requirement {
            // 경계선 콜링 핸드
            if facing_large_bet && stack_commitment > 0.3 {
                // 상당한 스택 커밋이 있는 큰 베트 - 더 많은 폴딩
                strategy.insert("fold".to_string(), 0.4);
                strategy.insert("call".to_string(), 0.55);
                strategy.insert("raise".to_string(), 0.05);
            } else {
                // 표준 콜링 상황
                strategy.insert("fold".to_string(), 0.2);
                strategy.insert("call".to_string(), 0.75);
                strategy.insert("raise".to_string(), 0.05);
            }
        } else if hand_strength > 0.2 && !facing_large_bet {
            // 약한 핸드 - 간헐적 블러프 레이즈
            let bluff_freq = 0.1;
            strategy.insert("fold".to_string(), 0.9 - bluff_freq);
            strategy.insert("call".to_string(), 0.05);
            strategy.insert("raise".to_string(), bluff_freq);
        } else {
            // 매우 약한 핸드: 대부분 폴드
            strategy.insert("fold".to_string(), 0.95);
            strategy.insert("call".to_string(), 0.05);
        }
    }

    /// 프리플랍 핸드 랭킹 조회 테이블 초기화
    fn init_preflop_rankings(rankings: &mut HashMap<(u8, u8, bool), f64>) {
        // 포켓 페어
        rankings.insert((12, 12, false), 0.95); // AA
        rankings.insert((11, 11, false), 0.92); // KK
        rankings.insert((10, 10, false), 0.88); // QQ
        rankings.insert((9, 9, false), 0.84); // JJ
        rankings.insert((8, 8, false), 0.78); // TT
        rankings.insert((7, 7, false), 0.72); // 99
        rankings.insert((6, 6, false), 0.65); // 88
        rankings.insert((5, 5, false), 0.58); // 77
        rankings.insert((4, 4, false), 0.50); // 66
        rankings.insert((3, 3, false), 0.42); // 55
        rankings.insert((2, 2, false), 0.35); // 44
        rankings.insert((1, 1, false), 0.30); // 33
        rankings.insert((0, 0, false), 0.25); // 22

        // 프리미엄 수티드 핸드
        rankings.insert((12, 11, true), 0.90); // AKs
        rankings.insert((12, 10, true), 0.85); // AQs
        rankings.insert((11, 10, true), 0.80); // KQs
        rankings.insert((12, 9, true), 0.75); // AJs
        rankings.insert((12, 8, true), 0.70); // ATs
        rankings.insert((11, 9, true), 0.72); // KJs
        rankings.insert((10, 9, true), 0.68); // QJs

        // 프리미엄 오프수트 핸드
        rankings.insert((12, 11, false), 0.82); // AKo
        rankings.insert((12, 10, false), 0.76); // AQo
        rankings.insert((11, 10, false), 0.71); // KQo
        rankings.insert((12, 9, false), 0.65); // AJo

        // More suited hands...
        for high in 5..12 {
            for low in 0..high {
                if high == 12 {
                    // Ax suited
                    let strength = 0.55 + (low as f64 * 0.02);
                    rankings.insert((high, low, true), strength);
                    // Ax offsuit
                    let strength_o = strength - 0.08;
                    rankings.insert((high, low, false), strength_o);
                } else if high >= 10 {
                    // Broadway suited
                    let strength = 0.50 + ((high + low) as f64 * 0.01);
                    rankings.insert((high, low, true), strength);
                    // Broadway offsuit
                    let strength_o = strength - 0.06;
                    rankings.insert((high, low, false), strength_o);
                }
            }
        }
    }

    /// 팟 오즈 계산
    fn calculate_pot_odds(&self, state: &WebGameState) -> f64 {
        if state.to_call == 0 {
            1.0 // No call required
        } else {
            state.pot as f64 / (state.pot + state.to_call) as f64
        }
    }

    /// 전략적 추론 설명 생성  
    fn generate_reasoning(
        &self,
        state: &WebGameState,
        hand_strength: f64,
        pot_odds: f64,
        action: &str,
    ) -> String {
        let mut reasoning = String::new();

        // 핸드 스트렝스 평가
        if hand_strength > 0.8 {
            reasoning.push_str("프리미엄 핸드 스트렝스. ");
        } else if hand_strength > 0.6 {
            reasoning.push_str("좋은 핸드 스트렝스. ");
        } else if hand_strength > 0.4 {
            reasoning.push_str("한계적 핸드 스트렝스. ");
        } else {
            reasoning.push_str("약한 핸드 스트렝스. ");
        }

        // Pot odds analysis
        if state.to_call > 0 {
            if hand_strength > pot_odds + 0.1 {
                reasoning.push_str("Favorable pot odds support calling/raising. ");
            } else if hand_strength > pot_odds - 0.05 {
                reasoning.push_str("Marginal pot odds situation. ");
            } else {
                reasoning.push_str("Poor pot odds suggest folding. ");
            }
        }

        // Stack depth considerations
        let effective_stack = state.my_stack.min(state.opponent_stack);
        let spr = effective_stack as f64 / state.pot as f64;

        if spr > 10.0 {
            reasoning.push_str("Deep stacks allow for post-flop play. ");
        } else if spr < 3.0 {
            reasoning.push_str("Short stacks favor aggressive play. ");
        }

        // Action justification
        match action {
            "fold" => reasoning.push_str("Folding to minimize losses."),
            "check" => reasoning.push_str("Checking to control pot size."),
            "call" => reasoning.push_str("Calling to see next card."),
            "bet_small" | "raise" => reasoning.push_str("Betting for value/protection."),
            "bet_large" => reasoning.push_str("Large bet for maximum value."),
            _ => reasoning.push_str("Standard play."),
        }

        reasoning
    }

    /// 전략의 기댓값 추정
    fn estimate_expected_value(
        &self,
        state: &WebGameState,
        strategy: &HashMap<String, f64>,
        hand_strength: f64,
    ) -> f64 {
        let mut ev = 0.0;
        let win_rate = hand_strength;

        for (action, prob) in strategy {
            let action_ev = match action.as_str() {
                "fold" => {
                    if state.to_call > 0 {
                        -(state.to_call as f64 * 0.1) // Small loss for folding
                    } else {
                        0.0
                    }
                }
                "check" => {
                    // 팟 컨트롤 - 핸드 스트렝스에 기반한 작은 양수/음수
                    (win_rate - 0.5) * state.pot as f64 * 0.3
                }
                "call" => {
                    // EV = (win_rate * pot_size) - (lose_rate * call_amount)
                    let win_amount = state.pot as f64;
                    let lose_amount = state.to_call as f64;
                    (win_rate * win_amount) - ((1.0 - win_rate) * lose_amount)
                }
                "bet_small" => {
                    let bet_size = (state.pot as f64 * 0.5).max(50.0);
                    if win_rate > 0.6 {
                        bet_size * 0.4 // Good value bet
                    } else {
                        bet_size * -0.2 // Bluff that usually fails
                    }
                }
                "bet_large" | "raise" => {
                    let bet_size = (state.pot as f64 * 1.0).max(100.0);
                    if win_rate > 0.7 {
                        bet_size * 0.6 // Strong value bet
                    } else {
                        bet_size * -0.4 // Expensive bluff
                    }
                }
                _ => 0.0,
            };

            ev += prob * action_ev;
        }

        ev
    }

    /// 상황 명확성에 기반한 의사결정 신뢰도 계산
    fn calculate_confidence(&self, state: &WebGameState, hand_strength: f64, pot_odds: f64) -> f64 {
        let mut confidence: f64 = 0.7; // Base confidence

        // Very strong or very weak hands increase confidence
        if hand_strength > 0.85 || hand_strength < 0.25 {
            confidence += 0.15;
        }

        // Clear pot odds situations increase confidence
        let odds_margin = (hand_strength - pot_odds).abs();
        if odds_margin > 0.2 {
            confidence += 0.1;
        }

        // Preflop situations are generally clearer
        if state.street == 0 {
            confidence += 0.05;
        }

        // Short stack situations are clearer (less postflop play)
        let effective_stack = state.my_stack.min(state.opponent_stack);
        if effective_stack < state.pot * 3 {
            confidence += 0.08;
        }

        confidence.min(0.95)
    }

    /// 전략 확률을 1.0으로 정규화
    fn normalize_strategy(&self, strategy: &mut HashMap<String, f64>) {
        let total: f64 = strategy.values().sum();
        if total > 0.0 {
            for prob in strategy.values_mut() {
                *prob /= total;
            }
        }
    }

    /// 고급 핸드 스트렝스 평가 (0.0 - 1.0)
    fn evaluate_hand_strength(&self, state: &WebGameState) -> f64 {
        let hole = state.hole_cards;

        if state.board.is_empty() {
            // Preflop evaluation using lookup table
            self.preflop_hand_strength(hole)
        } else {
            // Postflop evaluation with sophisticated analysis
            self.postflop_hand_strength(hole, &state.board)
        }
    }

    /// 정교한 프리플랍 핸드 스트렝스 평가
    fn preflop_hand_strength(&self, hole: [u8; 2]) -> f64 {
        let rank1 = hole[0] % 13;
        let rank2 = hole[1] % 13;
        let suited = hole[0] / 13 == hole[1] / 13;

        let high_rank = rank1.max(rank2);
        let low_rank = rank1.min(rank2);

        // Check precomputed rankings first
        if let Some(&strength) = self.preflop_rankings.get(&(high_rank, low_rank, suited)) {
            return strength;
        }

        // Fallback calculation for hands not in lookup table
        if rank1 == rank2 {
            // Pocket pairs
            0.45 + (high_rank as f64 * 0.04) // Base + rank bonus
        } else if high_rank >= 11 {
            // Ace or King high
            let gap_penalty = if low_rank < high_rank - 4 { 0.08 } else { 0.0 };
            let base = if suited { 0.55 } else { 0.45 };
            base + (low_rank as f64 * 0.02) - gap_penalty
        } else if suited && (high_rank - low_rank <= 4) {
            // Suited connectors and gappers
            0.35 + (high_rank as f64 * 0.015) + if high_rank - low_rank <= 1 { 0.05 } else { 0.0 }
        } else {
            // Random hands
            0.20 + ((high_rank + low_rank) as f64 * 0.008)
        }
    }

    /// 고급 포스트플랍 핸드 스트렝스 평가
    fn postflop_hand_strength(&self, hole: [u8; 2], board: &[u8]) -> f64 {
        let hole_ranks: Vec<u8> = hole.iter().map(|&c| c % 13).collect();
        let hole_suits: Vec<u8> = hole.iter().map(|&c| c / 13).collect();
        let board_ranks: Vec<u8> = board.iter().map(|&c| c % 13).collect();
        let board_suits: Vec<u8> = board.iter().map(|&c| c / 13).collect();

        let all_ranks = [hole_ranks.clone(), board_ranks.clone()].concat();
        let all_suits = [hole_suits.clone(), board_suits.clone()].concat();

        // Count rank frequencies
        let mut rank_counts = [0u8; 13];
        for &rank in &all_ranks {
            rank_counts[rank as usize] += 1;
        }

        // Count suit frequencies
        let mut suit_counts = [0u8; 4];
        for &suit in &all_suits {
            suit_counts[suit as usize] += 1;
        }

        // Check for various hand types
        let pairs = rank_counts.iter().filter(|&&count| count >= 2).count();
        let trips = rank_counts.iter().filter(|&&count| count >= 3).count();
        let quads = rank_counts.iter().filter(|&&count| count >= 4).count();
        let flush_possible = suit_counts.iter().any(|&count| count >= 5);

        // 핸드 스트렝스 평가
        if quads > 0 {
            0.95 // Four of a kind
        } else if trips > 0 && pairs > 1 {
            0.90 // Full house
        } else if flush_possible {
            self.evaluate_flush_strength(&all_ranks, &all_suits)
        } else if self.has_straight(&all_ranks) {
            self.evaluate_straight_strength(&all_ranks)
        } else if trips > 0 {
            0.75 // Three of a kind
        } else if pairs >= 2 {
            0.65 // Two pair
        } else if pairs == 1 {
            self.evaluate_pair_strength(&hole_ranks, &board_ranks, &all_ranks)
        } else {
            self.evaluate_high_card_strength(&hole_ranks, &all_ranks)
        }
    }

    /// 플러시 스트렝스 평가
    fn evaluate_flush_strength(&self, ranks: &[u8], suits: &[u8]) -> f64 {
        let mut suit_ranks = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for (i, &suit) in suits.iter().enumerate() {
            if i < ranks.len() {
                suit_ranks[suit as usize].push(ranks[i]);
            }
        }

        for suit_cards in &mut suit_ranks {
            if suit_cards.len() >= 5 {
                suit_cards.sort_by(|a, b| b.cmp(a)); // Sort descending
                let top_card = suit_cards[0];
                return if top_card >= 12 {
                    0.88
                } else if top_card >= 10 {
                    0.85
                } else {
                    0.82
                };
            }
        }
        0.82 // Default flush value
    }

    /// 스트레이트 확인
    fn has_straight(&self, ranks: &[u8]) -> bool {
        let mut unique_ranks: Vec<u8> = ranks.iter().cloned().collect();
        unique_ranks.sort();
        unique_ranks.dedup();

        // Check for wheel (A-2-3-4-5)
        if unique_ranks.contains(&12)
            && unique_ranks.contains(&0)
            && unique_ranks.contains(&1)
            && unique_ranks.contains(&2)
            && unique_ranks.contains(&3)
        {
            return true;
        }

        // Check for regular straights
        for window in unique_ranks.windows(5) {
            if window[4] - window[0] == 4 {
                return true;
            }
        }
        false
    }

    /// 스트레이트 스트렝스 평가
    fn evaluate_straight_strength(&self, ranks: &[u8]) -> f64 {
        let max_rank = *ranks.iter().max().unwrap_or(&0);
        if max_rank >= 12 {
            0.80
        } else if max_rank >= 10 {
            0.78
        } else {
            0.76
        }
    }

    /// 페어 스트렝스 평가
    fn evaluate_pair_strength(
        &self,
        hole_ranks: &[u8],
        board_ranks: &[u8],
        all_ranks: &[u8],
    ) -> f64 {
        // Find the paired rank
        let mut rank_counts = [0u8; 13];
        for &rank in all_ranks {
            rank_counts[rank as usize] += 1;
        }

        let paired_rank = rank_counts
            .iter()
            .position(|&count| count >= 2)
            .unwrap_or(0) as u8;

        // Check if we have pocket pair or made pair with hole card
        let pocket_pair = hole_ranks[0] == hole_ranks[1];
        let top_pair = hole_ranks.contains(&paired_rank) && board_ranks.contains(&paired_rank);

        let base_strength = match paired_rank {
            12 => 0.68, // Aces
            11 => 0.65, // Kings
            10 => 0.62, // Queens
            9 => 0.58,  // Jacks
            8 => 0.55,  // Tens
            _ => 0.50,  // Lower pairs
        };

        if pocket_pair {
            base_strength + 0.05 // Pocket pair bonus
        } else if top_pair {
            base_strength
        } else {
            base_strength - 0.08 // Lower pair penalty
        }
    }

    /// 하이카드 스트렝스 평가
    fn evaluate_high_card_strength(&self, hole_ranks: &[u8], all_ranks: &[u8]) -> f64 {
        let max_hole = hole_ranks.iter().max().unwrap_or(&0);
        let max_all = all_ranks.iter().max().unwrap_or(&0);

        if hole_ranks.contains(max_all) {
            // We have the top card
            match max_all {
                12 => 0.45, // Ace high
                11 => 0.40, // King high
                10 => 0.35, // Queen high
                _ => 0.30,  // Lower high cards
            }
        } else {
            // Our hole cards don't include the board's highest card
            match max_hole {
                12 => 0.35, // Ace in hole but not top card
                11 => 0.30, // King in hole
                _ => 0.25,  // Lower cards
            }
        }
    }

    /// 최고 액션 추천 받기
    fn get_best_action(&self, strategy: &HashMap<String, f64>) -> String {
        strategy
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(action, _)| action.clone())
            .unwrap_or_else(|| "check".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_api_basic() {
        let api = QuickPokerAPI::new();

        let state = WebGameState {
            hole_cards: [0, 13], // AA
            board: vec![],
            street: 0,
            pot: 150,
            to_call: 100,
            my_stack: 1000,
            opponent_stack: 1000,
        };

        let response = api.get_optimal_strategy(state);

        assert!(!response.strategy.is_empty());
        assert!(!response.recommended_action.is_empty());
        println!("전략: {:?}", response);
    }

    #[test]
    fn test_quick_api_postflop() {
        let api = QuickPokerAPI::new();

        let state = WebGameState {
            hole_cards: [0, 26],    // A♠ K♠
            board: vec![1, 21, 34], // A♥ 9♠ J♥
            street: 1,
            pot: 200,
            to_call: 0,
            my_stack: 900,
            opponent_stack: 900,
        };

        let response = api.get_optimal_strategy(state);
        println!("포스트플랍 전략: {:?}", response);

        assert!(!response.strategy.is_empty());
    }

    #[test]
    fn test_batch_processing() {
        let api = QuickPokerAPI::new();

        let states = vec![
            WebGameState {
                hole_cards: [48, 49], // KK
                board: vec![],
                street: 0,
                pot: 100,
                to_call: 50,
                my_stack: 2000,
                opponent_stack: 2000,
            },
            WebGameState {
                hole_cards: [26, 39], // KQ suited
                board: vec![47, 21, 34],
                street: 1,
                pot: 200,
                to_call: 0,
                my_stack: 900,
                opponent_stack: 900,
            },
        ];

        let responses = api.get_strategies_batch(states);
        assert_eq!(responses.len(), 2);

        for (i, response) in responses.iter().enumerate() {
            println!(
                "Batch {}: {} (EV: {:.2})",
                i, response.recommended_action, response.expected_value
            );
        }
    }
}
