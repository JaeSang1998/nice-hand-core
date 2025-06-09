// CFR+ 알고리즘 핵심 구현
// CFR+ (Counterfactual Regret Minimization Plus) 를 사용한 포커 전략 학습
//
// CFR+는 기존 CFR의 개선된 버전으로, 누적 후회값(regret sum)이
// 음수가 되지 않도록 보장하여 더 빠른 수렴과 안정적인 학습을 제공합니다.
//
// 주요 개선 사항:
// - 음수 후회값을 0으로 클램핑하여 전략의 안정성 향상
// - 더 빠른 수렴 속도
// - 메모리 사용량 최적화 (음수 값 저장 불필요)

use fxhash::FxHashMap as HashMap;
use rand::rngs::ThreadRng;

/// 게임 공통 트레잇 - 모든 포커 게임이 구현해야 하는 기본 인터페이스
///
/// 이 트레잇을 구현하면 CFR 알고리즘을 자동으로 적용할 수 있습니다.
/// 예: 텍사스 홀덤, 쿠언 포커, 오마하 등
pub trait Game: Sync {
    type State: Clone + Sync + GameState; // 게임 상태 (보드, 스택, 카드 등)
    type Action: Copy + Clone + PartialEq + Eq + std::hash::Hash + Sync + std::fmt::Debug; // 가능한 액션들
    type InfoKey: Copy + Clone + PartialEq + Eq + std::hash::Hash + Sync; // 정보 집합 식별자

    const N_PLAYERS: usize; // 플레이어 수

    /// 현재 액션할 플레이어 반환 (None이면 터미널 또는 찬스 노드)
    fn current_player(s: &Self::State) -> Option<usize>;

    /// 현재 상태에서 가능한 모든 액션들 반환
    fn legal_actions(s: &Self::State) -> Vec<Self::Action>;

    /// 액션을 적용한 다음 상태 반환
    fn next_state(s: &Self::State, a: Self::Action) -> Self::State;

    /// 찬스 노드에서 랜덤 이벤트 적용 (카드 딜링 등)
    fn apply_chance(s: &Self::State, r: &mut ThreadRng) -> Self::State;

    /// 터미널 노드에서 히어로의 유틸리티 값 계산
    fn util(s: &Self::State, hero: usize) -> f64;

    /// 플레이어의 정보 집합 키 생성 (같은 키 = 같은 정보)
    fn info_key(s: &Self::State, v: usize) -> Self::InfoKey;
}

/// CFR 노드 - 각 정보 집합에서의 전략과 리그렛 저장
///
/// 노드는 다음을 추적합니다:
/// - regret_sum: 각 액션에 대한 누적 리그렛
/// - strat_sum: 각 액션의 누적 전략 확률  
/// - delta_prefs: δ-uniform 믹싱을 위한 선호도 값
#[derive(Clone)]
pub struct Node {
    regret_sum: Vec<f64>,  // 누적 리그렛 합계
    strat_sum: Vec<f64>,   // 누적 전략 합계
    delta_prefs: Vec<f64>, // δ 선호도 (균일 분포 방지)
}

impl Node {
    /// 새 노드 생성
    ///
    /// # 매개변수
    /// - n_acts: 가능한 액션 수
    /// - delta_prefs: 각 액션의 초기 선호도
    pub fn new(n_acts: usize, delta_prefs: Vec<f64>) -> Self {
        Self {
            regret_sum: vec![0.0; n_acts],
            strat_sum: vec![0.0; n_acts],
            delta_prefs,
        }
    }

    /// 현재 전략 계산 (regret matching+ 알고리즘)
    ///
    /// 리그렛이 양수인 액션에 더 높은 확률을 부여합니다.
    /// δ-uniform 믹싱을 적용하여 전략 붕괴를 방지합니다.
    pub fn strategy(&self) -> Vec<f64> {
        let n = self.regret_sum.len();
        let mut s = vec![0.0; n];

        // 양수 리그렛의 합계 계산
        let mut sum_pos = 0.0;
        for i in 0..n {
            if self.regret_sum[i] > 0.0 {
                sum_pos += self.regret_sum[i];
            }
        }

        // 전략 계산: 양수 리그렛 비례 + δ-uniform 믹싱
        if sum_pos > 0.0 {
            for i in 0..n {
                let regret_part = if self.regret_sum[i] > 0.0 {
                    self.regret_sum[i] / sum_pos
                } else {
                    0.0
                };

                let delta_part = self.delta_prefs[i] / n as f64;
                let eps = 0.1; // 믹싱 비율
                s[i] = (1.0 - eps) * regret_part + eps * delta_part;
            }
        } else {
            // 리그렛이 모두 음수면 δ 선호도 기반 균일 분포
            for i in 0..n {
                s[i] = self.delta_prefs[i] / n as f64;
            }
        }

        s
    }

    /// 평균 전략 계산 (수렴된 최종 전략)
    ///
    /// 학습 과정에서 누적된 전략의 평균을 반환합니다.
    /// 이것이 실제 게임에서 사용할 최종 전략입니다.
    pub fn average(&self) -> Vec<f64> {
        let sum: f64 = self.strat_sum.iter().sum();
        if sum > 0.0 {
            self.strat_sum.iter().map(|&x| x / sum).collect()
        } else {
            // 학습이 충분하지 않으면 균일 분포
            let n = self.strat_sum.len();
            vec![1.0 / n as f64; n]
        }
    }

    /// average()의 별칭 - lib.rs와의 호환성을 위함
    pub fn avg_strategy(&self) -> Vec<f64> {
        self.average()
    }

    /// 다른 노드와 병합 (서브게임 리솔빙에서 사용)
    ///
    /// 서브게임에서 학습한 전략을 메인 전략에 통합할 때 사용합니다.
    pub fn merge(&mut self, other: &Node) {
        for i in 0..self.strat_sum.len() {
            self.strat_sum[i] += other.strat_sum[i];
        }
    }

    /// 액션 i의 리그렛 합계 업데이트 (CFR+ 버전)
    /// CFR+: 누적 후회값이 음수가 되지 않도록 보장
    pub fn update_regret(&mut self, action_idx: usize, value: f64) {
        if action_idx < self.regret_sum.len() {
            self.regret_sum[action_idx] = (self.regret_sum[action_idx] + value).max(0.0);
        }
    }

    /// 액션 i의 전략 합계 업데이트
    pub fn update_strategy(&mut self, action_idx: usize, value: f64) {
        if action_idx < self.strat_sum.len() {
            self.strat_sum[action_idx] += value;
        }
    }
}

/// 스레드 로컬 데이터 - 병렬 CFR 실행을 위한 랜덤 생성기
struct ThreadLocalData {
    rng: ThreadRng,
}

thread_local! {
    static TL_DATA: std::cell::RefCell<ThreadLocalData> = std::cell::RefCell::new(ThreadLocalData {
        rng: rand::thread_rng(),
    });
}

/// CFR 학습기 - 전체 학습 과정을 관리하는 메인 클래스
///
/// 주요 기능:
/// - 여러 루트 상태에서 CFR 알고리즘 실행
/// - 병렬 처리를 통한 빠른 학습
/// - 노드별 전략 저장 및 관리
pub struct Trainer<G: Game> {
    /// 정보 집합별 노드 저장소
    /// 키: 정보 집합 식별자, 값: CFR 노드
    pub nodes: HashMap<G::InfoKey, Node>,
}

impl<G: Game> Trainer<G> {
    /// 새 학습기 생성
    pub fn new() -> Self {
        Self {
            nodes: HashMap::default(),
        }
    }

    /// CFR 학습 실행
    ///
    /// # 매개변수
    /// - roots: 학습할 초기 상태들 (다양한 시나리오)
    /// - iterations: 반복 횟수 (많을수록 정확한 전략)
    ///
    /// # 예시
    /// ```rust
    /// use nice_hand_core::{Trainer, holdem};
    ///
    /// let mut trainer = Trainer::<holdem::State>::new();
    /// let initial_state = holdem::State::new();
    /// trainer.run(vec![initial_state], 10);
    /// ```
    pub fn run(&mut self, roots: Vec<G::State>, iterations: usize) {
        // 성능을 위해 시작/종료만 로그 - 상세 로깅이 큰 속도 저하를 일으킴
        println!(
            "📚 CFR 학습 시작 - {} 시나리오, {} 반복",
            roots.len(),
            iterations
        );

        for iteration in 0..iterations {
            // 콘솔 오버헤드를 줄이기 위해 10번째마다만 로그
            if iteration % 10 == 0 || iteration == iterations - 1 {
                println!("  반복 {}/{} 진행 중...", iteration + 1, iterations);
            }

            for (_root_idx, root) in roots.iter().enumerate() {
                for hero in 0..G::N_PLAYERS {
                    TL_DATA.with(|tl| {
                        let mut tl = tl.borrow_mut();
                        let _result = self.cfr(root, hero, 1.0, &mut tl.rng);
                        // 성능을 위해 플레이어별 로깅 제거
                    });
                }
            }
        }

        println!("✅ CFR 학습 완료 - {} 개 노드 생성", self.nodes.len());
    }

    /// CFR 알고리즘 핵심 재귀 함수
    ///
    /// 각 게임 트리 노드에서 다음을 수행:
    /// 1. 터미널 노드면 유틸리티 반환
    /// 2. 찬스 노드면 랜덤 이벤트 적용 후 재귀
    /// 3. 플레이어 노드면 전략 계산, 리그렛 업데이트
    ///
    /// # 매개변수  
    /// - state: 현재 게임 상태
    /// - hero: 관찰자 플레이어 (0~N_PLAYERS-1)
    /// - prob: 현재 상태에 도달할 확률
    /// - rng: 랜덤 생성기
    ///
    /// # 반환값
    /// 히어로의 기댓값 (expected value)
    fn cfr(&mut self, state: &G::State, hero: usize, prob: f64, rng: &mut ThreadRng) -> f64 {
        self.cfr_with_depth(state, hero, prob, rng, 0)
    }

    /// CFR 알고리즘 (깊이 추적 버전)
    fn cfr_with_depth(
        &mut self,
        state: &G::State,
        hero: usize,
        prob: f64,
        rng: &mut ThreadRng,
        depth: usize,
    ) -> f64 {
        // 매우 보수적인 깊이 제한으로 무한 재귀 방지
        if depth > 15 {
            return 0.0;
        }

        let result = if let Some(player) = G::current_player(state) {
            // 플레이어 노드: 전략 계산 및 리그렛 업데이트
            let actions = G::legal_actions(state);
            if actions.is_empty() {
                G::util(state, hero)
            } else {
                let info_key = G::info_key(state, player);

                // 노드가 없으면 생성 (균일 선호도로 초기화)
                if !self.nodes.contains_key(&info_key) {
                    let delta_prefs = vec![1.0; actions.len()];
                    self.nodes
                        .insert(info_key, Node::new(actions.len(), delta_prefs));
                }

                let strategy = {
                    let node = self.nodes.get(&info_key).unwrap();
                    node.strategy()
                };

                let mut utilities = vec![0.0; actions.len()];
                let mut node_util = 0.0;

                // 각 액션에 대해 재귀적으로 CFR 실행
                for (i, &action) in actions.iter().enumerate() {
                    let next_state = G::next_state(state, action);
                    utilities[i] =
                        self.cfr_with_depth(&next_state, hero, prob * strategy[i], rng, depth + 1);
                    node_util += strategy[i] * utilities[i];
                }

                // 히어로 플레이어면 리그렛과 전략 합계 업데이트 (CFR+ 버전)
                if player == hero {
                    let node = self.nodes.get_mut(&info_key).unwrap();
                    for i in 0..actions.len() {
                        let regret = utilities[i] - node_util;
                        // CFR+: 누적 후회값이 음수가 되지 않도록 max(0.0) 적용
                        node.regret_sum[i] = (node.regret_sum[i] + prob * regret).max(0.0);
                        node.strat_sum[i] += prob * strategy[i];
                    }
                }

                node_util
            }
        } else {
            // 터미널 또는 찬스 노드
            if state.is_terminal() {
                G::util(state, hero)
            } else {
                // 찬스 노드: 랜덤 이벤트 적용 후 재귀
                let chance_state = G::apply_chance(state, rng);
                self.cfr_with_depth(&chance_state, hero, prob, rng, depth + 1)
            }
        };

        result
    }
}

/// 게임 상태 확장 트레잇 - 터미널/찬스 노드 판별
///
/// 각 게임은 이 트레잇을 구현하여 상태 유형을 정의해야 합니다.
pub trait GameState {
    /// 게임이 끝났는지 확인 (모든 플레이어가 폴드했거나 쇼다운)
    fn is_terminal(&self) -> bool;

    /// 찬스 노드인지 확인 (카드를 딜해야 하는 상황)
    fn is_chance_node(&self) -> bool;
}
