// Monte Carlo CFR (MCCFR) 구현
// 기존 CFR의 게임 트리 폭발 문제를 해결하기 위해 샘플링 기반 CFR 사용

use fxhash::FxHashMap as HashMap;
use rand::rngs::ThreadRng;
use crate::cfr_core::{Game, Node, GameState};

/// Monte Carlo CFR 학습기
/// 
/// 전체 게임 트리를 탐색하는 대신 액션을 샘플링하여 탐색합니다.
/// 이를 통해 포커와 같은 대형 게임에서도 실용적인 학습이 가능합니다.
pub struct MCCFRTrainer<G: Game> {
    pub nodes: HashMap<G::InfoKey, Node>,
    sample_rate: f64,  // 액션 샘플링 비율 (0.0~1.0)
}

impl<G: Game> MCCFRTrainer<G> {
    /// 새 MCCFR 학습기 생성
    /// 
    /// # 매개변수
    /// - sample_rate: 각 노드에서 탐색할 액션의 비율 (예: 0.3 = 30% 액션만 탐색)
    pub fn new(sample_rate: f64) -> Self {
        Self {
            nodes: HashMap::default(),
            sample_rate: sample_rate.clamp(0.1, 1.0),
        }
    }
    
    /// MCCFR 학습 실행
    pub fn run(&mut self, roots: Vec<G::State>, iterations: usize) {
        println!("🎲 Monte Carlo CFR 학습 시작 - {} 시나리오, {} 반복, {:.1}% 샘플링", 
                 roots.len(), iterations, self.sample_rate * 100.0);
        
        for iteration in 0..iterations {
            if iteration % 100 == 0 {
                println!("  반복 {}/{} (노드: {})", iteration + 1, iterations, self.nodes.len());
            }
            
            for root in &roots {
                for hero in 0..G::N_PLAYERS {
                    let mut rng = rand::thread_rng();
                    self.mccfr(root, hero, 1.0, &mut rng, 0);
                }
            }
            
            // 주기적으로 진행 상황 출력
            if iteration % 1000 == 999 {
                println!("    진행률: {:.1}%, 탐색된 노드: {}", 
                         (iteration as f64 / iterations as f64) * 100.0, 
                         self.nodes.len());
            }
        }
        
        println!("✅ MCCFR 학습 완료 - {} 개 노드 생성", self.nodes.len());
    }
    
    /// Monte Carlo CFR 재귀 함수
    /// 
    /// 각 플레이어 노드에서 모든 액션을 탐색하는 대신 일부만 샘플링합니다.
    fn mccfr(&mut self, state: &G::State, hero: usize, prob: f64, rng: &mut ThreadRng, depth: usize) -> f64 {
        // 깊이 제한 (MCCFR은 일반 CFR보다 더 깊이 탐색 가능)
        if depth > 50 {
            return 0.0;
        }
        
        if let Some(player) = G::current_player(state) {
            // 플레이어 노드
            let actions = G::legal_actions(state);
            if actions.is_empty() {
                return G::util(state, hero);
            }
            
            let info_key = G::info_key(state, player);
            
            // 노드가 없으면 생성
            if !self.nodes.contains_key(&info_key) {
                let delta_prefs = vec![1.0; actions.len()];
                self.nodes.insert(info_key, Node::new(actions.len(), delta_prefs));
            }
            
            let strategy = {
                let node = self.nodes.get(&info_key).unwrap();
                node.strategy()
            };
            
            // 액션 샘플링: 모든 액션 대신 일부만 탐색
            let sample_size = ((actions.len() as f64 * self.sample_rate).ceil() as usize).max(1);
            let mut sampled_indices: Vec<usize> = (0..actions.len()).collect();
            
            // 전략 확률이 높은 액션을 우선적으로 샘플링
            sampled_indices.sort_by(|&a, &b| strategy[b].partial_cmp(&strategy[a]).unwrap_or(std::cmp::Ordering::Equal));
            sampled_indices.truncate(sample_size);
            
            let mut utilities = vec![0.0; actions.len()];
            let mut node_util = 0.0;
            
            // 샘플링된 액션들만 탐색
            for &i in &sampled_indices {
                let action = actions[i];
                let next_state = G::next_state(state, action);
                utilities[i] = self.mccfr(&next_state, hero, prob * strategy[i], rng, depth + 1);
                node_util += strategy[i] * utilities[i];
            }
            
            // 히어로 플레이어만 리그렛 업데이트
            if player == hero {
                let node = self.nodes.get_mut(&info_key).unwrap();
                for &i in &sampled_indices {
                    let regret = utilities[i] - node_util;
                    node.update_regret(i, prob * regret);
                    node.update_strategy(i, prob * strategy[i]);
                }
            }
            
            node_util
        } else {
            // 터미널 또는 찬스 노드
            if state.is_terminal() {
                G::util(state, hero)
            } else {
                let chance_state = G::apply_chance(state, rng);
                self.mccfr(&chance_state, hero, prob, rng, depth + 1)
            }
        }
    }
}
