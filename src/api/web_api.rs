// 포커 전략 평가를 위한 웹 API - 무상태 방식
// 각 요청마다 현재 게임 상태를 제공하면 최적 전략을 반환합니다

use crate::game::holdem;
use crate::solver::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 웹 API용 게임 상태 - 직렬화 가능
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebGameState {
    /// 홀카드 (요청하는 플레이어만)
    pub hole_cards: [u8; 2],
    /// 보드 카드 (0=preflop, 3=flop, 4=turn, 5=river)
    pub board: Vec<u8>,
    /// 현재 스트리트 (0=preflop, 1=flop, 2=turn, 3=river)
    pub street: u8,
    /// 팟 크기
    pub pot: u32,
    /// 각 플레이어의 스택
    pub stacks: Vec<u32>,
    /// 생존한 플레이어들
    pub alive_players: Vec<usize>,
    /// 현재 스트리트에서 각 플레이어가 투자한 금액
    pub street_investments: Vec<u32>,
    /// 콜하기 위해 필요한 금액
    pub to_call: u32,
    /// 액션을 취해야 할 플레이어
    pub player_to_act: usize,
    /// 요청하는 플레이어의 포지션
    pub hero_position: usize,
    /// 베팅 히스토리 (각 스트리트별)
    pub betting_history: Vec<Vec<Action>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    Fold,
    Call,
    Raise(u32), // 실제 레이즈 금액
}

/// 웹 API 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyResponse {
    /// 각 액션에 대한 확률
    pub strategy: HashMap<String, f64>,
    /// 예상 EV
    pub expected_value: f64,
    /// 권장 액션
    pub recommended_action: String,
    /// 신뢰도 (0-1, 학습된 데이터의 충분함 정도)
    pub confidence: f64,
}

/// Pre-computed strategy lookup table
pub struct StrategyTable {
    /// 미리 계산된 전략들 (InfoKey -> Strategy)
    strategies: HashMap<u64, Vec<f64>>,
    /// 액션 매핑
    action_names: Vec<String>,
}

impl StrategyTable {
    /// 미리 학습된 CFR 결과로부터 lookup table 생성
    pub fn from_trained_cfr(trainer: &Trainer<holdem::State>) -> Self {
        let mut strategies = HashMap::new();
        let action_names = vec![
            "fold".to_string(),
            "call".to_string(),
            "raise_small".to_string(),
            "raise_medium".to_string(),
            "raise_large".to_string(),
            "all_in".to_string(),
        ];

        // CFR 노드들을 lookup table로 변환
        for (key, node) in &trainer.nodes {
            strategies.insert(*key, node.average());
        }

        Self {
            strategies,
            action_names,
        }
    }

    /// 웹 상태로부터 전략 계산
    pub fn get_strategy(&self, state: &WebGameState) -> StrategyResponse {
        // 1. 현재 상태를 internal state로 변환
        let internal_state = self.web_to_internal_state(state);

        // 2. Info key 계산
        let info_key = holdem::State::info_key(&internal_state, state.hero_position);

        // 3. 미리 계산된 전략 조회
        if let Some(strategy_vec) = self.strategies.get(&info_key) {
            let mut strategy_map = HashMap::new();
            let mut max_prob = 0.0;
            let mut recommended = "fold".to_string();

            // 유효한 액션들만 필터링
            let legal_actions = holdem::State::legal_actions(&internal_state);

            for (i, &prob) in strategy_vec.iter().enumerate() {
                if i < self.action_names.len() && i < legal_actions.len() {
                    let action_name = &self.action_names[i];
                    strategy_map.insert(action_name.clone(), prob);

                    if prob > max_prob {
                        max_prob = prob;
                        recommended = action_name.clone();
                    }
                }
            }

            // EV는 간단한 휴리스틱으로 추정 (실제로는 더 정교한 계산 필요)
            let ev = self.estimate_ev(state, &strategy_map);

            StrategyResponse {
                strategy: strategy_map,
                expected_value: ev,
                recommended_action: recommended,
                confidence: 0.8, // 고정값, 실제로는 샘플 수 기반으로 계산
            }
        } else {
            // 학습되지 않은 상황 - 기본 전략 사용
            self.default_strategy(state)
        }
    }

    /// 웹 상태를 내부 상태로 변환
    fn web_to_internal_state(&self, web_state: &WebGameState) -> holdem::State {
        let mut state = holdem::State {
            hole: [[0; 2]; 6],
            board: web_state.board.clone(),
            to_act: web_state.player_to_act,
            street: web_state.street,
            pot: web_state.pot,
            stack: [0; 6],
            alive: [false; 6],
            invested: [0; 6],
            to_call: web_state.to_call,
            actions_taken: 0,
        };

        // 히어로의 홀카드 설정
        state.hole[web_state.hero_position] = web_state.hole_cards;

        // 스택과 생존 상태 설정
        for (i, &player_idx) in web_state.alive_players.iter().enumerate() {
            if player_idx < 6 && i < web_state.stacks.len() {
                state.stack[player_idx] = web_state.stacks[i];
                state.alive[player_idx] = true;
            }
        }

        // 현재 스트리트 투자 금액 설정
        for (i, &investment) in web_state.street_investments.iter().enumerate() {
            if i < 6 {
                state.invested[i] = investment;
            }
        }

        state
    }

    /// EV 추정 (간단한 휴리스틱)
    fn estimate_ev(&self, state: &WebGameState, strategy: &HashMap<String, f64>) -> f64 {
        // 간단한 예시 - 실제로는 더 정교한 계산 필요
        let fold_prob = strategy.get("fold").unwrap_or(&0.0);
        let call_prob = strategy.get("call").unwrap_or(&0.0);
        let raise_prob =
            strategy.values().filter(|&&p| p > 0.0).sum::<f64>() - fold_prob - call_prob;

        // 기본적인 EV 추정
        let pot_odds = if state.to_call > 0 {
            state.pot as f64 / (state.pot + state.to_call) as f64
        } else {
            1.0
        };

        // 매우 단순한 추정식
        call_prob * pot_odds * 0.5 + raise_prob * pot_odds * 0.7
            - fold_prob * (state.to_call as f64)
    }

    /// 기본 전략 (학습되지 않은 상황용)
    fn default_strategy(&self, state: &WebGameState) -> StrategyResponse {
        let mut strategy = HashMap::new();

        // 매우 기본적인 룰 기반 전략
        if state.to_call == 0 {
            // 체크 가능한 상황
            strategy.insert("call".to_string(), 0.7);
            strategy.insert("raise_small".to_string(), 0.2);
            strategy.insert("fold".to_string(), 0.1);
        } else if state.to_call > state.pot / 2 {
            // 큰 베팅에 직면
            strategy.insert("fold".to_string(), 0.7);
            strategy.insert("call".to_string(), 0.2);
            strategy.insert("raise_large".to_string(), 0.1);
        } else {
            // 보통 상황
            strategy.insert("fold".to_string(), 0.3);
            strategy.insert("call".to_string(), 0.5);
            strategy.insert("raise_medium".to_string(), 0.2);
        }

        StrategyResponse {
            strategy,
            expected_value: 0.0,
            recommended_action: "call".to_string(),
            confidence: 0.3, // 낮은 신뢰도
        }
    }
}

/// 웹 API 메인 핸들러
pub struct PokerWebAPI {
    strategy_table: StrategyTable,
}

impl PokerWebAPI {
    /// 미리 학습된 모델로부터 API 생성
    pub fn new(trainer: &Trainer<holdem::State>) -> Self {
        Self {
            strategy_table: StrategyTable::from_trained_cfr(trainer),
        }
    }

    /// 단일 요청 처리 - stateless
    pub fn get_optimal_strategy(&self, game_state: WebGameState) -> StrategyResponse {
        self.strategy_table.get_strategy(&game_state)
    }

    /// 배치 요청 처리 - 여러 상황을 한 번에
    pub fn get_strategies_batch(&self, states: Vec<WebGameState>) -> Vec<StrategyResponse> {
        states
            .into_iter()
            .map(|state| self.get_optimal_strategy(state))
            .collect()
    }

    /// 특정 스트리트에서의 권장 액션만 빠르게 조회
    pub fn get_quick_recommendation(&self, game_state: WebGameState) -> String {
        let response = self.get_optimal_strategy(game_state);
        response.recommended_action
    }
}

/// 오프라인 학습용 헬퍼
pub struct OfflineTrainer;

impl OfflineTrainer {
    /// Train with a simple single scenario (fast for testing)
    pub fn train_simple_strategy(iterations: usize) -> Trainer<holdem::State> {
        let mut trainer = Trainer::new();

        // Use single scenario for fast testing
        let scenarios = vec![holdem::State::new()];

        // Run CFR training
        trainer.run(scenarios, iterations);

        trainer
    }

    /// Train with comprehensive game scenarios (slower but more thorough)
    pub fn train_comprehensive_strategy(iterations: usize) -> Trainer<holdem::State> {
        let mut trainer = Trainer::new();

        // Use comprehensive training scenarios
        let scenarios = Self::generate_training_scenarios();

        // Run CFR training
        trainer.run(scenarios, iterations);

        trainer
    }

    /// Generate comprehensive training scenarios
    fn generate_training_scenarios() -> Vec<holdem::State> {
        let mut scenarios = Vec::new();

        // 1. Preflop scenarios
        scenarios.extend(Self::generate_preflop_scenarios());

        // 2. Postflop scenarios
        scenarios.extend(Self::generate_postflop_scenarios());

        // 3. Special situations (short stack, bubble, etc.)
        scenarios.extend(Self::generate_special_scenarios());

        scenarios
    }

    fn generate_preflop_scenarios() -> Vec<holdem::State> {
        // Generate various preflop situations
        vec![
            // UTG open scenarios
            holdem::State {
                hole: [[0, 13], [26, 39], [0, 0], [0, 0], [0, 0], [0, 0]], // AA vs random
                board: vec![],
                to_act: 0,
                street: 0,
                pot: 75,
                stack: [2000; 6],
                alive: [true; 6],
                invested: [0, 0, 0, 0, 25, 50],
                to_call: 50,
                actions_taken: 0,
            },
            // Add 3-bet scenarios, call scenarios, etc...
        ]
    }

    fn generate_postflop_scenarios() -> Vec<holdem::State> {
        // Generate various flop/turn/river situations
        vec![
            // Dry board scenarios
            holdem::State {
                hole: [[0, 1], [26, 39], [0, 0], [0, 0], [0, 0], [0, 0]],
                board: vec![48, 21, 6], // A-9-3 rainbow
                to_act: 0,
                street: 1,
                pot: 200,
                stack: [1000, 1000, 0, 0, 0, 0],
                alive: [true, true, false, false, false, false],
                invested: [0, 0, 0, 0, 0, 0],
                to_call: 0,
                actions_taken: 0,
            },
            // Add wet board scenarios, etc...
        ]
    }

    fn generate_special_scenarios() -> Vec<holdem::State> {
        // Generate ICM, short stack, bubble situations
        vec![
            // Bubble play
            holdem::State {
                hole: [[40, 41], [26, 39], [0, 0], [0, 0], [0, 0], [0, 0]], // KK vs random
                board: vec![],
                to_act: 0,
                street: 0,
                pot: 150,
                stack: [800, 3000, 0, 0, 0, 0], // Short vs big stack
                alive: [true, true, false, false, false, false],
                invested: [25, 50, 0, 0, 0, 0],
                to_call: 50,
                actions_taken: 0,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_api_basic() {
        // 기본 오프라인 학습 (5회로 축소)
        let trainer = OfflineTrainer::train_comprehensive_strategy(5);

        // Web API 생성
        let api = PokerWebAPI::new(&trainer);

        // 테스트 요청
        let game_state = WebGameState {
            hole_cards: [0, 1], // Ace of spades, deuce of hearts
            board: vec![],
            street: 0,
            pot: 150,
            stacks: vec![1000, 1000],
            alive_players: vec![0, 1],
            street_investments: vec![50, 100],
            to_call: 100,
            player_to_act: 0,
            hero_position: 0,
            betting_history: vec![],
        };

        let response = api.get_optimal_strategy(game_state);

        // 응답 검증
        assert!(!response.strategy.is_empty());
        assert!(!response.recommended_action.is_empty());
        println!("Strategy response: {:?}", response);
    }

    #[test]
    fn test_stateless_multiple_requests() {
        let trainer = OfflineTrainer::train_comprehensive_strategy(5);
        let api = PokerWebAPI::new(&trainer);

        // 여러 독립적인 요청들
        let states = vec![
            WebGameState {
                hole_cards: [0, 13], // AA
                board: vec![],
                street: 0,
                pot: 100,
                stacks: vec![1000, 1000],
                alive_players: vec![0, 1],
                street_investments: vec![25, 50],
                to_call: 50,
                player_to_act: 0,
                hero_position: 0,
                betting_history: vec![],
            },
            WebGameState {
                hole_cards: [26, 39],    // KQ suited
                board: vec![47, 21, 34], // K-9-J
                street: 1,
                pot: 200,
                stacks: vec![900, 900],
                alive_players: vec![0, 1],
                street_investments: vec![0, 0],
                to_call: 0,
                player_to_act: 0,
                hero_position: 0,
                betting_history: vec![],
            },
        ];

        let responses = api.get_strategies_batch(states);
        assert_eq!(responses.len(), 2);

        for (i, response) in responses.iter().enumerate() {
            println!(
                "Response {}: recommended={}, confidence={}",
                i, response.recommended_action, response.confidence
            );
        }
    }
}
