// 텍사스 홀덤 6-Max 게임 로직
// Preference CFR과 서브게임 리솔빙을 지원하는 완전한 구현

use crate::solver::cfr_core::{Game, GameState, Trainer};
use crate::game::card_abstraction::*;
use rand::{rngs::ThreadRng, Rng};

/// 텍사스 홀덤 게임 상태
/// 
/// 6명까지 참여 가능한 No-Limit Hold'em 게임의 모든 정보를 포함합니다.
/// CFR 알고리즘이 이 상태를 기반으로 최적 전략을 학습합니다.
#[derive(Clone, Debug)]
pub struct State {
    /// 각 플레이어의 홀카드 [플레이어][카드]  
    pub hole: [[u8; 2]; 6],
    
    /// 보드카드 (플랍/턴/리버)
    pub board: Vec<u8>,
    
    /// 현재 액션할 플레이어 (0-5)
    pub to_act: usize,
    
    /// 현재 스트리트 (0=프리플랍, 1=플랍, 2=턴, 3=리버)
    pub street: u8,
    
    /// 현재 팟 크기
    pub pot: u32,
    
    /// 각 플레이어의 스택 크기
    pub stack: [u32; 6],
    
    /// 살아있는 플레이어 여부 (폴드하지 않음)
    pub alive: [bool; 6],
    
    /// 현재 스트리트에서 각 플레이어가 투자한 금액
    pub invested: [u32; 6],
    
    /// 콜하기 위해 필요한 금액
    pub to_call: u32,
    
    /// 현재 스트리트에서 수행된 액션 수
    pub actions_taken: usize,
}

impl State {
    /// 새 게임 상태 생성 (프리플랍 시작)
    /// 
    /// # 매개변수
    /// - blinds: [스몰블라인드, 빅블라인드] 금액
    /// - stacks: 각 플레이어의 초기 스택
    /// - player_count: 참여 플레이어 수 (2-6)
    /// 
    /// # 반환값
    /// - 초기화된 게임 상태
    pub fn new_hand(blinds: [u32; 2], stacks: [u32; 6], player_count: usize) -> Self {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        
        let mut state = Self {
            hole: [[0; 2]; 6],
            board: Vec::new(),
            to_act: if player_count == 2 { 0 } else { 3 }, // UTG부터 시작 (HU는 버튼부터)
            street: 0,
            pot: blinds[0] + blinds[1],
            stack: stacks,
            alive: [false; 6],
            invested: [0; 6],
            to_call: blinds[1],
            actions_taken: 0,
        };
        
        // 참여 플레이어 설정
        for i in 0..player_count {
            state.alive[i] = true;
        }
        
        // 블라인드 처리
        let sb_pos = if player_count == 2 { 0 } else { player_count - 2 };
        let bb_pos = if player_count == 2 { 1 } else { player_count - 1 };
        
        state.invested[sb_pos] = blinds[0];
        state.invested[bb_pos] = blinds[1];
        state.stack[sb_pos] -= blinds[0];
        state.stack[bb_pos] -= blinds[1];
        
        // 홀카드 딜링 (52장 덱에서 랜덤)
        let mut deck: Vec<u8> = (0..52).collect();
        deck.shuffle(&mut thread_rng());
        
        for i in 0..player_count {
            state.hole[i][0] = deck[i * 2];
            state.hole[i][1] = deck[i * 2 + 1];
        }
        
        state
    }
    
    /// 기본 게임 상태 생성 (테스트/예제용)
    /// 
    /// CFR 학습에 최적화된 헤즈업 게임 설정:
    /// - 블라인드: 50/100
    /// - 스택: 모든 플레이어 1,000 (10bb 짧은 스택으로 복잡성 감소)
    /// - 2명 참여 (헤즈업으로 복잡성 최소화)
    pub fn new() -> Self {
        let blinds = [50, 100]; // 스몰/빅 블라인드
        let stacks = [1000; 6]; // 짧은 스택으로 게임 길이 단축
        let player_count = 2; // 헤즈업만 지원 (CFR 학습 효율성)
        
        Self::new_hand(blinds, stacks, player_count)
    }

    /// 다음 액션할 플레이어 찾기
    fn find_next_player(&self, current: usize) -> Option<usize> {
        let alive_count = self.alive.iter().filter(|&&a| a).count();
        if alive_count <= 1 {
            return None; // 게임 종료
        }
        
        for i in 1..=6 {
            let next = (current + i) % 6;
            if self.alive[next] {
                return Some(next);
            }
        }
        None
    }
    
    /// 베팅 라운드가 끝났는지 확인
    fn is_betting_complete(&self) -> bool {
        let alive_players: Vec<usize> = (0..6).filter(|&i| self.alive[i]).collect();
        
        if alive_players.len() <= 1 {
            return true;
        }
        
        // 모든 살아있는 플레이어가 액션했는지 확인
        if self.actions_taken < alive_players.len() {
            return false;
        }
        
        // 모든 살아있는 플레이어가 같은 금액을 투자했는지 확인
        let max_investment = alive_players.iter().map(|&i| self.invested[i]).max().unwrap_or(0);
        
        for &player in &alive_players {
            // 올인하지 않은 플레이어는 최대 투자액과 같아야 함
            if !self.is_all_in(player) && self.invested[player] < max_investment {
                return false;
            }
        }
        
        true
    }
    
    /// 다음 스트리트로 진행
    fn advance_street(&mut self) {
        self.street += 1;
        self.invested = [0; 6]; // 투자 금액 리셋
        self.to_call = 0;
        self.actions_taken = 0;
        
        // 첫 번째 살아있는 플레이어부터 시작
        self.to_act = (0..6).find(|&i| self.alive[i]).unwrap_or(0);
    }
    
    /// 올인 여부 확인
    pub fn is_all_in(&self, player: usize) -> bool {
        self.stack[player] == 0
    }
    
    /// 현재 최소 레이즈 크기 계산
    pub fn min_raise_size(&self) -> u32 {
        // 마지막 레이즈 크기의 2배 또는 빅블라인드 중 큰 값
        std::cmp::max(self.to_call * 2, 100) // 100 = 기본 빅블라인드
    }
}

impl GameState for State {
    /// 게임 종료 여부 확인
    /// 
    /// 다음 조건 중 하나라도 만족하면 터미널:
    /// - 1명만 남음 (나머지 모두 폴드)
    /// - 리버까지 완료하고 베팅 끝남
    /// - 모든 플레이어가 올인
    /// - CFR 학습 효율성을 위한 조기 종료 조건들
    fn is_terminal(&self) -> bool {
        let alive_count = self.alive.iter().filter(|&&a| a).count();
        
        // 1명만 남으면 게임 종료
        if alive_count <= 1 {
            return true;
        }
        
        // CFR 학습을 위한 보수적인 종료 조건들
        // 게임이 너무 길어지면 강제 종료
        if self.actions_taken > 12 {  // 매우 보수적인 액션 제한 (플레이어당 2액션)
            return true;
        }
        
        // 플랍 이후에는 더 빠른 종료 (포스트플랍 복잡성 감소)
        if self.street >= 1 && self.actions_taken > 6 {
            return true;
        }
        
        // 리버까지 완료되고 베팅이 끝났으면 종료
        if self.street >= 3 && self.is_betting_complete() {
            return true;
        }
        
        // 모든 플레이어가 올인이면 카드만 오픈하고 종료
        let active_players: Vec<usize> = (0..6).filter(|&i| self.alive[i]).collect();
        if active_players.iter().all(|&i| self.is_all_in(i)) {
            return true;
        }
        
        false
    }
    
    /// 찬스 노드 여부 확인
    /// 
    /// 베팅이 끝나고 다음 스트리트로 넘어갈 때 카드를 딜해야 하는 상황
    fn is_chance_node(&self) -> bool {
        if self.is_terminal() {
            return false;
        }
        
        // 베팅이 완료되고 아직 리버가 아니면 카드 딜링 필요
        if self.is_betting_complete() && self.street < 3 {
            return true;
        }
        
        false
    }
}

/// 홀덤 액션 정의
/// 
/// 플레이어가 할 수 있는 모든 행동을 나타냅니다.
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Act {
    /// 포기 (패배 인정)
    Fold,
    
    /// 콜 (현재 베팅에 맞춤)
    Call,
    
    /// 레이즈 (베팅 크기 증가)
    /// 0 = 미니멀 레이즈
    /// 1 = 스몰 레이즈 (팟의 1/2)
    /// 2 = 미디엄 레이즈 (팟 크기)  
    /// 3 = 빅 레이즈 (팟의 2배 또는 올인)
    Raise(u8),
}

impl Game for State {
    type State = State;
    type Action = Act;
    type InfoKey = u64;
    
    /// 6명 최대 참여
    const N_PLAYERS: usize = 6;
    
    /// 현재 액션할 플레이어 반환
    fn current_player(s: &Self::State) -> Option<usize> {
        if s.is_terminal() || s.is_chance_node() {
            return None;
        }
        
        // 베팅이 완료되었으면 현재 플레이어 없음 (찬스 노드 또는 터미널로 진행)
        if s.is_betting_complete() {
            return None;
        }
        
        // 현재 플레이어가 살아있고 액션 가능한지 확인
        if s.alive[s.to_act] && !s.is_all_in(s.to_act) {
            Some(s.to_act)
        } else {
            // 다음 플레이어 찾기
            s.find_next_player(s.to_act)
        }
    }
    
    /// 현재 상황에서 가능한 액션들 반환
    fn legal_actions(s: &Self::State) -> Vec<Self::Action> {
        if s.is_terminal() || s.is_chance_node() {
            return vec![];
        }
        
        let player = s.to_act;
        if !s.alive[player] || s.is_all_in(player) {
            return vec![];
        }
        
        let mut actions = vec![Act::Fold];
        
        // 콜 가능 여부 확인
        let call_amount = s.to_call.saturating_sub(s.invested[player]);
        if call_amount <= s.stack[player] {
            actions.push(Act::Call);
        }
        
        // CFR을 위해 매우 간소화된 액션 스페이스 (게임 트리 복잡도 최소화)
        if s.stack[player] > call_amount {
            let remaining_after_call = s.stack[player] - call_amount;
            
            // 단 1가지 레이즈 크기만 제공 (복잡도 대폭 감소)
            if remaining_after_call > 0 {
                actions.push(Act::Raise(0)); // 올인만 허용
            }
        }
        
        actions
    }
    
    /// 액션 적용하여 다음 상태 생성
    fn next_state(s: &Self::State, a: Self::Action) -> Self::State {
        let mut next = s.clone();
        let player = s.to_act;
        
        match a {
            Act::Fold => {
                next.alive[player] = false;
            }
            
            Act::Call => {
                let call_amount = s.to_call.saturating_sub(s.invested[player]);
                let actual_call = std::cmp::min(call_amount, s.stack[player]);
                
                next.invested[player] += actual_call;
                next.stack[player] -= actual_call;
                next.pot += actual_call;
            }
            
            Act::Raise(size) => {
                let call_amount = s.to_call.saturating_sub(s.invested[player]);
                
                // 레이즈 크기 계산
                let raise_amount = match size {
                    0 => std::cmp::min(s.pot, s.stack[player] - call_amount), // 팟 베팅
                    1 => s.stack[player] - call_amount, // 올인
                    _ => s.stack[player] - call_amount, // 기본값은 올인
                };
                
                let total_investment = call_amount + raise_amount;
                next.invested[player] += total_investment;
                next.stack[player] -= total_investment;
                next.pot += total_investment;
                next.to_call = next.invested[player];
            }
        }
        
        next.actions_taken += 1;
        
        // 베팅 라운드 완료 체크 및 다음 플레이어 설정
        if next.is_betting_complete() {
            // 베팅 라운드가 끝났으면 찬스 노드가 되거나 터미널 상태가 됨
            // advance_street는 apply_chance에서 처리하도록 함
            next.to_act = 6; // 유효하지 않은 플레이어 번호로 설정하여 찬스 노드임을 표시
        } else {
            // 베팅이 계속되면 다음 플레이어 찾기
            if let Some(next_player) = next.find_next_player(player) {
                next.to_act = next_player;
            }
        }
        
        next
    }
    
    /// 찬스 노드에서 카드 딜링
    fn apply_chance(s: &Self::State, rng: &mut ThreadRng) -> Self::State {
        let mut next = s.clone();
        
        if next.is_betting_complete() && next.street < 3 {
            // 다음 스트리트로 진행하고 카드 딜링
            next.advance_street();
            
            match next.street {
                1 => {
                    // 플랍: 3장 추가
                    for _ in 0..3 {
                        next.board.push(rng.gen_range(0..52));
                    }
                }
                2 => {
                    // 턴: 1장 추가
                    next.board.push(rng.gen_range(0..52));
                }
                3 => {
                    // 리버: 1장 추가
                    next.board.push(rng.gen_range(0..52));
                }
                _ => {}
            }
        }
        
        next
    }
    
    /// 터미널 노드에서 유틸리티 계산
    fn util(s: &Self::State, hero: usize) -> f64 {
        if !s.alive[hero] {
            // 폴드했으면 현재 투자 금액만큼 손실
            return -(s.invested[hero] as f64);
        }
        
        let alive_players: Vec<usize> = (0..6).filter(|&i| s.alive[i]).collect();
        
        if alive_players.len() == 1 {
            // 혼자 남았으면 전체 팟 획득
            return s.pot as f64 - s.invested[hero] as f64;
        }
        
        // 쇼다운: 핸드 강도 비교 (간단한 구현)
        if s.board.len() >= 3 {
            let hero_strength = hand_strength(s.hole[hero], &s.board);
            let mut wins = 0;
            let mut total_opponents = 0;
            
            for &opponent in &alive_players {
                if opponent != hero {
                    let opp_strength = hand_strength(s.hole[opponent], &s.board);
                    total_opponents += 1;
                    if hero_strength > opp_strength {
                        wins += 1;
                    }
                }
            }
            
            // 승률에 따른 팟 분배 (간단한 근사)
            let win_rate = if total_opponents > 0 {
                wins as f64 / total_opponents as f64
            } else {
                1.0
            };
            
            return win_rate * s.pot as f64 - s.invested[hero] as f64;
        }
        
        // 보드가 없으면 균등 분할 가정
        s.pot as f64 / alive_players.len() as f64 - s.invested[hero] as f64
    }
    
    /// 정보 집합 키 생성
    fn info_key(s: &Self::State, player: usize) -> Self::InfoKey {
        // 플레이어가 볼 수 있는 정보만 사용하여 키 생성
        let mut key = 0u64;
        
        // 홀카드 정보 (플레이어 본인만)
        let hole_bucket = if s.street == 0 {
            preflop_bucket(s.hole[player]) as u64
        } else {
            postflop_bucket(s.hole[player], &s.board, s.street) as u64
        };
        key ^= hole_bucket;
        
        // 보드카드 정보 (모든 플레이어가 볼 수 있음)
        for &card in &s.board {
            key ^= (card as u64) << 16;
        }
        
        // 베팅 히스토리 (간단한 해시)
        key ^= (s.pot as u64) << 32;
        key ^= (s.to_call as u64) << 24;
        key ^= (s.street as u64) << 20;
        key ^= (s.actions_taken as u64) << 8;
        
        // 스택 크기 구간 (정확한 값 대신 구간 사용)
        let stack_ratio = if s.pot > 0 {
            (s.stack[player] / std::cmp::max(s.pot, 1)) as u64
        } else {
            0
        };
        key ^= stack_ratio << 4;
        
        // 가능한 액션 수도 키에 포함 (같은 상황이라도 액션 수가 다르면 다른 노드)
        let legal_actions = Self::legal_actions(s);
        key ^= (legal_actions.len() as u64) << 60;
        
        key
    }
}

/// 서브게임 리솔빙 함수
/// 
/// 특정 상황에서 더 정확한 전략을 얻기 위해 작은 게임 트리에서 
/// 추가 CFR 학습을 수행합니다. 턴/리버에서 특히 유용합니다.
/// 
/// # 매개변수
/// - global: 메인 CFR 트레이너 (결과가 여기에 병합됨)
/// - root: 서브게임 시작 상태
/// - extra_iter: 추가 학습 반복 횟수
pub fn resolve_subgame(
    global: &mut Trainer<State>,
    root: State,
    extra_iter: usize
) {
    println!("🔍 서브게임 리솔빙 시작 - {} 추가 반복", extra_iter);
    
    // 독립적인 서브게임 트레이너 생성
    let mut sub_trainer = Trainer::<State>::new();
    
    // 서브게임에서 집중 학습
    sub_trainer.run(vec![root.clone()], extra_iter);
    
    println!("  서브게임 학습 완료 - {} 노드 생성", sub_trainer.nodes.len());
    
    // 서브게임 결과를 글로벌 전략에 병합
    for (key, node) in sub_trainer.nodes {
        global.nodes.entry(key)
            .and_modify(|existing_node| existing_node.merge(&node))
            .or_insert(node);
    }
    
    println!("✅ 서브게임 전략 병합 완료");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_game_state_creation() {
        let state = State::new_hand([25, 50], [1000; 6], 2);
        
        assert_eq!(state.pot, 75);
        assert_eq!(state.stack[0], 975); // SB 차감
        assert_eq!(state.stack[1], 950); // BB 차감
        assert!(state.alive[0] && state.alive[1]);
        assert!(!state.alive[2]); // 비참여 플레이어
        
        println!("게임 상태 생성 테스트 통과");
    }
    
    #[test] 
    fn test_legal_actions() {
        let state = State::new_hand([25, 50], [1000; 6], 2);
        let actions = State::legal_actions(&state);
        
        assert!(actions.contains(&Act::Fold));
        assert!(actions.contains(&Act::Call));
        assert!(actions.len() >= 2); // 최소 폴드, 콜 가능
        
        println!("액션 생성 테스트 통과");
    }
    
    #[test]
    fn test_state_transitions() {
        let mut state = State::new_hand([25, 50], [1000; 6], 2);
        
        // 콜 액션 적용
        state = State::next_state(&state, Act::Call);
        assert_eq!(state.invested[0], 50); // SB가 BB에 맞춤
        
        // 폴드 액션 적용  
        state = State::next_state(&state, Act::Fold);
        assert!(!state.alive[1]); // BB 폴드
        assert!(state.is_terminal()); // 게임 종료
        
        println!("상태 전환 테스트 통과");
    }
    
    #[test]
    fn test_info_key_generation() {
        let state = State::new_hand([25, 50], [1000; 6], 2);
        
        let key1 = State::info_key(&state, 0);
        let key2 = State::info_key(&state, 1);
        
        // 다른 플레이어는 다른 키를 가져야 함 (다른 홀카드)
        assert_ne!(key1, key2);
        
        println!("정보 집합 키 생성 테스트 통과");
    }
}
