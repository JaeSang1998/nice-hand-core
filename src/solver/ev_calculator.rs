// EV(Expected Value) 계산 모듈
// 특정 게임 상황에서 각 액션의 기댓값을 계산하여 최적 의사결정을 지원

use crate::game::card_abstraction::hand_strength;
use crate::game::holdem::{Act, State};
use crate::solver::cfr_core::{Game, GameState};

/// 액션별 EV 계산 결과
#[derive(Debug, Clone)]
pub struct ActionEV {
    pub action: Act,
    pub ev: f64,
    pub confidence: f64, // 계산의 신뢰도 (샘플 수 기반)
}

/// EV 계산 설정
#[derive(Debug, Clone)]
pub struct EVConfig {
    pub sample_count: usize,      // 시뮬레이션 샘플 수
    pub max_depth: u8,            // 최대 탐색 깊이
    pub use_opponent_model: bool, // 상대방 모델 사용 여부
}

impl Default for EVConfig {
    fn default() -> Self {
        Self {
            sample_count: 10000,
            max_depth: 10,
            use_opponent_model: true,
        }
    }
}

/// EV 계산기
pub struct EVCalculator {
    config: EVConfig,
}

impl EVCalculator {
    /// 새로운 EV 계산기 생성
    pub fn new(config: EVConfig) -> Self {
        Self { config }
    }

    /// 기본 설정으로 EV 계산기 생성
    pub fn default() -> Self {
        Self::new(EVConfig::default())
    }

    /// 현재 상태에서 모든 가능한 액션의 EV 계산
    pub fn calculate_action_evs(&self, state: &State) -> Vec<ActionEV> {
        let legal_actions = State::legal_actions(state);
        let mut action_evs = Vec::new();

        for action in legal_actions {
            let ev = self.calculate_single_action_ev(state, &action);
            let confidence = self.calculate_confidence(state);

            action_evs.push(ActionEV {
                action,
                ev,
                confidence,
            });
        }

        // EV 높은 순으로 정렬
        action_evs.sort_by(|a, b| b.ev.partial_cmp(&a.ev).unwrap());
        action_evs
    }

    /// 특정 액션의 EV 계산
    fn calculate_single_action_ev(&self, state: &State, action: &Act) -> f64 {
        // 액션 실행 후 상태 생성
        let next_state = State::next_state(state, action.clone());

        // 터미널 상태인 경우 즉시 평가
        if next_state.is_terminal() {
            return self.evaluate_terminal_state(&next_state, state.to_act);
        }

        // 몬테카를로 시뮬레이션으로 EV 계산
        let mut total_payoff = 0.0;
        for _ in 0..self.config.sample_count {
            let payoff = self.simulate_game(&next_state, state.to_act, 0);
            total_payoff += payoff;
        }

        total_payoff / self.config.sample_count as f64
    }

    /// 게임 시뮬레이션 (몬테카를로)
    fn simulate_game(&self, state: &State, original_player: usize, depth: u8) -> f64 {
        // 최대 깊이 도달 시 휴리스틱 평가
        if depth >= self.config.max_depth {
            return self.heuristic_evaluation(state, original_player);
        }

        // 터미널 상태 처리
        if state.is_terminal() {
            return self.evaluate_terminal_state(state, original_player);
        }

        // 찬스 노드 처리
        if state.is_chance_node() {
            let mut rng = rand::thread_rng();
            let chance_state = State::apply_chance(state, &mut rng);
            return self.simulate_game(&chance_state, original_player, depth + 1);
        }

        let current_player = State::current_player(state);
        let legal_actions = State::legal_actions(state);

        if legal_actions.is_empty() {
            return self.heuristic_evaluation(state, original_player);
        }

        // 액션 선택 (상대방 모델 또는 랜덤)
        let action =
            if self.config.use_opponent_model && current_player.unwrap_or(0) != original_player {
                self.select_opponent_action(state, &legal_actions)
            } else {
                self.select_random_action(&legal_actions)
            };

        // 다음 상태로 진행
        let next_state = State::next_state(state, action);
        self.simulate_game(&next_state, original_player, depth + 1)
    }

    /// 터미널 상태 평가
    fn evaluate_terminal_state(&self, state: &State, player: usize) -> f64 {
        // 정확한 payoff 계산
        let alive_count = state.alive.iter().filter(|&&alive| alive).count();

        if alive_count <= 1 {
            if state.alive[player] {
                state.pot as f64 - state.invested[player] as f64 // 팟에서 투자금 제외
            } else {
                -(state.invested[player] as f64) // 폴드했으면 투자금 손실
            }
        } else {
            // 쇼다운: 정확한 핸드 평가로 승률 계산
            let my_strength = self.estimate_hand_strength(state, player);
            let opponents_average_strength =
                self.estimate_opponents_average_strength(state, player);

            // 상대적 핸드 강도로 승률 계산 (Malmuth 모델 기반)
            let win_probability = self.calculate_showdown_probability(
                my_strength,
                opponents_average_strength,
                alive_count - 1,
            );

            // 정확한 EV 계산
            let total_pot = state.pot as f64;
            let my_investment = state.invested[player] as f64;

            win_probability * total_pot - my_investment
        }
    }

    /// 휴리스틱 평가 (게임 진행 중)
    fn heuristic_evaluation(&self, state: &State, player: usize) -> f64 {
        let hand_strength = self.estimate_hand_strength(state, player);
        let pot_size = state.pot as f64;
        let position_bonus = self.calculate_position_bonus(state, player);

        // 핸드 강도, 팟 크기, 포지션을 고려한 평가
        let base_ev = (hand_strength - 0.5) * pot_size * 0.3;
        base_ev + position_bonus
    }

    /// 핸드 강도 추정
    fn estimate_hand_strength(&self, state: &State, player: usize) -> f64 {
        if player < state.hole.len() {
            hand_strength(state.hole[player], &state.board)
        } else {
            0.5 // 정보 없음
        }
    }

    /// 상대방들의 평균 핸드 강도 추정
    fn estimate_opponents_average_strength(&self, state: &State, exclude_player: usize) -> f64 {
        let mut total_strength = 0.0;
        let mut count = 0;

        for i in 0..state.alive.len() {
            if i != exclude_player && state.alive[i] {
                // 상대방 정보가 없으므로 추정값 사용
                // 일반적으로 상대방은 평균적인 핸드를 가진다고 가정
                let estimated_strength = if state.board.is_empty() {
                    // 프리플랍에서는 더 보수적으로 추정
                    0.35 + (i as f64 * 0.05) // 포지션별 차이
                } else {
                    // 포스트플랍에서는 액션을 통해 추정
                    let aggression_factor = self.estimate_aggression_from_betting(state, i);
                    0.4 + aggression_factor * 0.3
                };
                total_strength += estimated_strength;
                count += 1;
            }
        }

        if count > 0 {
            total_strength / count as f64
        } else {
            0.5
        }
    }

    /// 베팅 패턴으로부터 공격성 추정
    fn estimate_aggression_from_betting(&self, state: &State, player: usize) -> f64 {
        let investment_ratio = state.invested[player] as f64 / state.pot.max(1) as f64;

        if investment_ratio > 0.3 {
            0.7 // 공격적인 베팅
        } else if investment_ratio > 0.1 {
            0.4 // 중간 베팅
        } else {
            0.2 // 패시브한 플레이
        }
    }

    /// 쇼다운에서의 승률 계산 (다중 상대 고려)
    fn calculate_showdown_probability(
        &self,
        my_strength: f64,
        avg_opponent_strength: f64,
        num_opponents: usize,
    ) -> f64 {
        // 각 상대방에 대한 개별 승률을 계산한 후 전체 승률 계산
        let individual_win_prob =
            self.calculate_heads_up_win_probability(my_strength, avg_opponent_strength);

        // 다중 상대에 대한 승률 (독립성 가정)
        let lose_prob_against_each = 1.0 - individual_win_prob;
        let lose_prob_against_all = lose_prob_against_each.powi(num_opponents as i32);

        1.0 - lose_prob_against_all
    }

    /// 헤즈업 승률 계산
    fn calculate_heads_up_win_probability(&self, my_strength: f64, opponent_strength: f64) -> f64 {
        // 로지스틱 함수를 사용한 승률 계산
        let strength_diff = my_strength - opponent_strength;
        let scaled_diff = strength_diff * 8.0; // 스케일링 팩터

        1.0 / (1.0 + (-scaled_diff).exp())
    }

    /// 포지션 보너스 계산
    fn calculate_position_bonus(&self, _state: &State, player: usize) -> f64 {
        // 간단한 포지션 보너스
        match player {
            0 => -5.0, // 얼리 포지션
            1 => 0.0,  // 미들 포지션
            _ => 5.0,  // 레이트 포지션
        }
    }

    /// 상대방 액션 선택 (정교한 모델)
    fn select_opponent_action(&self, state: &State, actions: &[Act]) -> Act {
        if let Some(current_player) = State::current_player(state) {
            let hand_strength = self.estimate_hand_strength(state, current_player);
            let pot_odds = self.calculate_pot_odds(state);
            let position_factor = self.get_position_factor(current_player, state);
            let stack_pressure = self.calculate_stack_pressure(state, current_player);

            // 포지션, 스택 크기, 팟 오즈를 종합적으로 고려
            let aggression_threshold = self.calculate_aggression_threshold(
                hand_strength,
                pot_odds,
                position_factor,
                stack_pressure,
            );

            // 액션 선택 로직
            if hand_strength > 0.75 || (hand_strength > 0.6 && position_factor > 0.7) {
                // 강한 핸드 또는 좋은 포지션에서 중간 핸드
                self.select_aggressive_action(actions, hand_strength, aggression_threshold)
            } else if hand_strength > 0.35 && pot_odds > 0.25 {
                // 중간 핸드에서 좋은 팟 오즈
                self.select_balanced_action(actions, hand_strength, pot_odds)
            } else if hand_strength < 0.3 || stack_pressure > 0.8 {
                // 약한 핸드 또는 스택 프레셔가 높은 상황
                self.select_defensive_action(actions)
            } else {
                // 기본적인 액션 선택
                self.select_default_action(actions, hand_strength)
            }
        } else {
            self.select_random_action(actions)
        }
    }

    /// 팟 오즈 계산
    fn calculate_pot_odds(&self, state: &State) -> f64 {
        if state.to_call == 0 {
            0.0
        } else {
            state.to_call as f64 / (state.pot + state.to_call) as f64
        }
    }

    /// 포지션 팩터 계산
    fn get_position_factor(&self, player: usize, state: &State) -> f64 {
        let active_players = state.alive.iter().filter(|&&alive| alive).count();
        let relative_position = player as f64 / active_players.max(1) as f64;

        // 레이트 포지션일수록 높은 값
        relative_position
    }

    /// 스택 프레셔 계산
    fn calculate_stack_pressure(&self, state: &State, player: usize) -> f64 {
        let big_blind = 50.0; // 기본 빅블라인드 값
        let effective_stack = state.stack[player] as f64;
        let bb_ratio = effective_stack / big_blind;

        if bb_ratio < 10.0 {
            1.0 // 매우 높은 프레셔
        } else if bb_ratio < 20.0 {
            0.7 // 높은 프레셔
        } else if bb_ratio < 50.0 {
            0.4 // 중간 프레셔
        } else {
            0.1 // 낮은 프레셔
        }
    }

    /// 공격성 임계값 계산
    fn calculate_aggression_threshold(
        &self,
        hand_strength: f64,
        pot_odds: f64,
        position_factor: f64,
        stack_pressure: f64,
    ) -> f64 {
        let base_threshold = 0.5;
        let hand_adjustment = (hand_strength - 0.5) * 0.4;
        let position_adjustment = (position_factor - 0.5) * 0.2;
        let pot_odds_adjustment = pot_odds * 0.3;
        let stack_adjustment = stack_pressure * 0.2;

        (base_threshold + hand_adjustment + position_adjustment + pot_odds_adjustment
            - stack_adjustment)
            .max(0.1)
            .min(0.9)
    }

    /// 공격적인 액션 선택
    fn select_aggressive_action(&self, actions: &[Act], hand_strength: f64, threshold: f64) -> Act {
        if hand_strength > threshold + 0.2 {
            // 매우 강한 핸드: 레이즈 우선
            actions
                .iter()
                .find(|a| matches!(a, Act::Raise(_)))
                .or_else(|| actions.iter().find(|a| matches!(a, Act::Call)))
                .unwrap_or(&actions[0])
                .clone()
        } else {
            // 강한 핸드: 콜 우선
            actions
                .iter()
                .find(|a| matches!(a, Act::Call))
                .or_else(|| actions.iter().find(|a| matches!(a, Act::Raise(_))))
                .unwrap_or(&actions[0])
                .clone()
        }
    }

    /// 균형잡힌 액션 선택
    fn select_balanced_action(&self, actions: &[Act], hand_strength: f64, pot_odds: f64) -> Act {
        let call_probability = hand_strength + pot_odds - 0.5;

        if call_probability > 0.6 {
            actions
                .iter()
                .find(|a| matches!(a, Act::Call))
                .unwrap_or(&actions[0])
                .clone()
        } else if call_probability > 0.3 {
            // 랜덤하게 콜 또는 폴드
            if rand::random::<f64>() < 0.6 {
                actions
                    .iter()
                    .find(|a| matches!(a, Act::Call))
                    .unwrap_or(&actions[0])
                    .clone()
            } else {
                actions
                    .iter()
                    .find(|a| matches!(a, Act::Fold))
                    .unwrap_or(&actions[0])
                    .clone()
            }
        } else {
            self.select_defensive_action(actions)
        }
    }

    /// 수비적인 액션 선택
    fn select_defensive_action(&self, actions: &[Act]) -> Act {
        actions
            .iter()
            .find(|a| matches!(a, Act::Fold))
            .or_else(|| actions.iter().find(|a| matches!(a, Act::Call)))
            .unwrap_or(&actions[0])
            .clone()
    }

    /// 기본 액션 선택
    fn select_default_action(&self, actions: &[Act], hand_strength: f64) -> Act {
        if hand_strength > 0.55 {
            actions
                .iter()
                .find(|a| matches!(a, Act::Call))
                .unwrap_or(&actions[0])
                .clone()
        } else {
            actions
                .iter()
                .find(|a| matches!(a, Act::Fold))
                .unwrap_or(&actions[0])
                .clone()
        }
    }

    /// 랜덤 액션 선택
    fn select_random_action(&self, actions: &[Act]) -> Act {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..actions.len());
        actions[index].clone()
    }

    /// 계산 신뢰도 추정
    fn calculate_confidence(&self, state: &State) -> f64 {
        // 샘플 수와 게임 단계를 고려한 신뢰도
        let sample_factor = (self.config.sample_count as f64 / 10000.0).min(1.0);
        let street_factor = match state.street {
            0 => 0.6, // 프리플랍: 낮은 신뢰도
            1 => 0.7, // 플랍: 중간 신뢰도
            2 => 0.8, // 턴: 높은 신뢰도
            3 => 0.9, // 리버: 매우 높은 신뢰도
            _ => 0.5,
        };

        sample_factor * street_factor
    }
}

/// 빠른 EV 계산을 위한 헬퍼 함수
pub fn quick_ev_analysis(state: &State, sample_count: Option<usize>) -> Vec<ActionEV> {
    let config = EVConfig {
        sample_count: sample_count.unwrap_or(1000),
        max_depth: 5,
        use_opponent_model: true,
    };

    let calculator = EVCalculator::new(config);
    calculator.calculate_action_evs(state)
}

/// 상세한 EV 분석을 위한 헬퍼 함수
pub fn detailed_ev_analysis(state: &State) -> Vec<ActionEV> {
    let config = EVConfig {
        sample_count: 50000,
        max_depth: 15,
        use_opponent_model: true,
    };

    let calculator = EVCalculator::new(config);
    calculator.calculate_action_evs(state)
}
