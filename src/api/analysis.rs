// 포커 분석 API 모듈
// 게임 상태 검증, EV 계산, 고급 분석 기능 제공

use crate::game::holdem::{Act, State as HoldemState};
use crate::solver::ev_calculator::{ActionEV, EVCalculator, EVConfig};
use crate::api::web_api::WebGameState;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::Instant;

/// 분석 요청 설정
#[derive(Debug, Deserialize, Clone)]
pub struct AnalysisRequest {
    pub game_state: WebGameState,
    pub options: AnalysisOptions,
}

/// 분석 옵션
#[derive(Debug, Deserialize, Clone)]
pub struct AnalysisOptions {
    /// 분석 깊이 ("quick", "standard", "deep")
    pub depth: String,
    /// 최대 계산 시간 (밀리초)
    pub max_calculation_time_ms: Option<u64>,
    /// 포함할 분석 요소들
    pub include_insights: bool,
    pub include_range_analysis: bool,
    pub include_equity_calculation: bool,
    /// 상대방 모델링 수준
    pub opponent_modeling: OpponentModel,
}

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            depth: "standard".to_string(),
            max_calculation_time_ms: None,
            include_insights: true,
            include_range_analysis: false,
            include_equity_calculation: false,
            opponent_modeling: OpponentModel::Tight,
        }
    }
}

/// 상대방 모델링 타입
#[derive(Debug, Deserialize, Clone)]
pub enum OpponentModel {
    /// 완전 랜덤 상대
    Random,
    /// 기본 TAG 스타일
    Tight,
    /// 공격적 스타일  
    Aggressive,
    /// 사용자 정의 (추후 구현)
    Custom,
}

/// 포괄적인 분석 응답
#[derive(Debug, Serialize, Clone)]
pub struct PokerAnalysisResponse {
    /// 기본 EV 분석
    pub ev_analysis: EVAnalysisResponse,
    /// 추가 인사이트
    pub insights: Option<AnalysisInsights>,
    /// 메타데이터
    pub metadata: AnalysisMetadata,
}

/// EV 분석 결과
#[derive(Debug, Serialize, Clone)]
pub struct EVAnalysisResponse {
    /// 각 액션별 EV 및 신뢰도 정보
    pub action_evs: Vec<ActionEV>,
    /// 사용된 분석 타입 ("quick" 또는 "detailed")
    pub analysis_type: String,
    /// 변환 과정이나 결과에 대한 추가 정보
    pub notes: Option<String>,
}

/// 분석 인사이트
#[derive(Debug, Serialize, Clone)]
pub struct AnalysisInsights {
    /// 추천 액션 (가장 높은 EV)
    pub recommended_action: Act,
    /// 각 액션의 상대적 강도 (0-100)
    pub action_strength: HashMap<String, f32>,
    /// 포지션별 조언
    pub positional_advice: Option<String>,
    /// 리스크 평가
    pub risk_assessment: RiskLevel,
    /// 핸드 스트렝스 점수
    pub hand_strength: f64,
}

/// 리스크 레벨
#[derive(Debug, Serialize, Clone)]
pub enum RiskLevel {
    Low,
    Medium, 
    High,
    Extreme,
}

/// 분석 메타데이터
#[derive(Debug, Serialize, Clone)]
pub struct AnalysisMetadata {
    pub calculation_time_ms: u64,
    pub analysis_depth: String,
    pub confidence_level: f32,
    pub limitations: Vec<String>,
    pub game_state_valid: bool,
}

/// 상태 검증 에러
#[derive(Debug, Serialize)]
pub enum ValidationError {
    InvalidPlayerCount(usize),
    InvalidStack(i32),
    InvalidCard(u8),
    InvalidBettingSequence,
    InconsistentState(String),
    InvalidPosition(usize),
    InvalidPot(i32),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPlayerCount(count) => write!(f, "유효하지 않은 플레이어 수: {}", count),
            Self::InvalidStack(stack) => write!(f, "유효하지 않은 스택 크기: {}", stack),
            Self::InvalidCard(card) => write!(f, "유효하지 않은 카드: {}", card),
            Self::InvalidBettingSequence => write!(f, "유효하지 않은 베팅 시퀀스"),
            Self::InconsistentState(msg) => write!(f, "일관성 없는 게임 상태: {}", msg),
            Self::InvalidPosition(pos) => write!(f, "유효하지 않은 포지션: {}", pos),
            Self::InvalidPot(pot) => write!(f, "유효하지 않은 팟 크기: {}", pot),
        }
    }
}

/// 분석 에러
#[derive(Debug, Serialize)]
pub enum AnalysisError {
    InvalidGameState { reason: String },
    CalculationTimeout,
    InsufficientData,
    InternalError { message: String },
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidGameState { reason } => write!(f, "게임 상태가 유효하지 않습니다: {}", reason),
            Self::CalculationTimeout => write!(f, "계산 시간이 초과되었습니다"),
            Self::InsufficientData => write!(f, "분석에 필요한 데이터가 부족합니다"),
            Self::InternalError { message } => write!(f, "내부 오류: {}", message),
        }
    }
}

pub type AnalysisResult = Result<PokerAnalysisResponse, AnalysisError>;

/// 상태 빌더 - 안전한 상태 변환을 위한 빌더 패턴
pub struct HoldemStateBuilder {
    num_players: Option<usize>,
    stacks: Option<Vec<i32>>,
    board: Option<Vec<u8>>,
    pot: Option<i32>,
    to_act: Option<usize>,
    hole_cards: Option<Vec<[u8; 2]>>,
}

impl HoldemStateBuilder {
    pub fn new() -> Self {
        Self {
            num_players: None,
            stacks: None,
            board: None,
            pot: None,
            to_act: None,
            hole_cards: None,
        }
    }
    
    /// WebGameState로부터 HoldemState 생성
    pub fn from_web_state(web_state: &WebGameState) -> Result<HoldemState, ValidationError> {
        let mut builder = Self::new();
        
        // 플레이어 수 검증 (스택 개수 기준)
        builder = builder.validate_player_count(web_state.stacks.len())?;
        
        // 스택 검증
        builder = builder.validate_stacks(&web_state.stacks)?;
        
        // 보드 카드 검증
        builder = builder.validate_board(&web_state.board)?;
        
        // 팟 검증
        builder = builder.validate_pot(web_state.pot)?;
        
        // 포지션 검증
        builder = builder.validate_position(web_state.player_to_act, web_state.stacks.len())?;
        
        // 홀 카드 설정
        builder = builder.set_hole_cards_from_web(web_state);
        
        builder.build()
    }
    
    fn validate_player_count(mut self, player_count: usize) -> Result<Self, ValidationError> {
        if player_count < 2 || player_count > 6 {
            return Err(ValidationError::InvalidPlayerCount(player_count));
        }
        self.num_players = Some(player_count);
        Ok(self)
    }
    
    fn validate_stacks(mut self, stacks: &[u32]) -> Result<Self, ValidationError> {
        let stacks_i32: Vec<i32> = stacks.iter().map(|&s| s as i32).collect();
        for &stack in &stacks_i32 {
            if stack < 0 {
                return Err(ValidationError::InvalidStack(stack));
            }
        }
        self.stacks = Some(stacks_i32);
        Ok(self)
    }
    
    fn validate_board(mut self, board: &[u8]) -> Result<Self, ValidationError> {
        if board.len() > 5 {
            return Err(ValidationError::InconsistentState("보드 카드는 최대 5장입니다".to_string()));
        }
        
        for &card in board {
            if card >= 52 {
                return Err(ValidationError::InvalidCard(card));
            }
        }
        
        self.board = Some(board.to_vec());
        Ok(self)
    }
    
    fn validate_pot(mut self, pot: u32) -> Result<Self, ValidationError> {
        self.pot = Some(pot as i32);
        Ok(self)
    }
    
    fn validate_position(mut self, position: usize, player_count: usize) -> Result<Self, ValidationError> {
        if position >= player_count {
            return Err(ValidationError::InvalidPosition(position));
        }
        self.to_act = Some(position);
        Ok(self)
    }
    
    fn set_hole_cards_from_web(mut self, web_state: &WebGameState) -> Self {
        // 현재는 hero의 홀 카드만 알고 있고, 나머지는 기본값 사용
        let mut hole_cards = Vec::new();
        let player_count = web_state.stacks.len();
        
        for i in 0..player_count {
            if i == web_state.hero_position {
                hole_cards.push(web_state.hole_cards);
            } else {
                hole_cards.push([i as u8 * 2, i as u8 * 2 + 1]); // 임시 카드
            }
        }
        self.hole_cards = Some(hole_cards);
        self
    }
    
    fn build(self) -> Result<HoldemState, ValidationError> {
        let num_players = self.num_players.ok_or_else(|| 
            ValidationError::InconsistentState("플레이어 수가 설정되지 않았습니다".to_string()))?;
        let stacks = self.stacks.ok_or_else(|| 
            ValidationError::InconsistentState("스택이 설정되지 않았습니다".to_string()))?;
        let board = self.board.unwrap_or_default();
        let pot = self.pot.unwrap_or(0);
        let to_act = self.to_act.unwrap_or(0);
        let hole_cards = self.hole_cards.unwrap_or_default();
        
        // 스트리트 계산
        let street = match board.len() {
            0 => 0,      // 프리플랍
            3 => 1,      // 플랍
            4 => 2,      // 턴
            5 => 3,      // 리버
            _ => return Err(ValidationError::InconsistentState("유효하지 않은 보드 카드 수".to_string())),
        };
        
        // HoldemState 생성
        let mut stacks_array = [0u32; 6];
        for (i, &stack) in stacks.iter().enumerate() {
            if i < 6 {
                stacks_array[i] = stack as u32;
            }
        }
        
        let mut state = HoldemState::new_hand(
            [10, 20], // 기본 스몰/빅 블라인드
            stacks_array,
            num_players,
        );
        
        // 상태 설정
        state.pot = pot as u32;
        state.board = board;
        state.to_act = to_act;
        state.street = street;
        
        // 홀 카드 설정 (가능한 범위 내에서)
        for (i, hole_card) in hole_cards.into_iter().enumerate() {
            if i < 6 {
                state.hole[i] = hole_card;
            }
        }
        
        Ok(state)
    }
}

/// 메인 분석 함수
pub fn analyze_poker_state(request: AnalysisRequest) -> AnalysisResult {
    let start_time = Instant::now();
    let mut limitations = Vec::new();
    
    // 1. 상태 변환 및 검증
    let internal_state = match HoldemStateBuilder::from_web_state(&request.game_state) {
        Ok(state) => state,
        Err(e) => return Err(AnalysisError::InvalidGameState { 
            reason: e.to_string() 
        }),
    };
    
    // 2. EV 계산 설정
    let ev_config = match request.options.depth.as_str() {
        "quick" => EVConfig {
            sample_count: 1000,
            max_depth: 5,
            use_opponent_model: false,
        },
        "standard" => EVConfig::default(),
        "deep" => EVConfig {
            sample_count: 50000,
            max_depth: 15,
            use_opponent_model: true,
        },
        _ => EVConfig::default(),
    };
    
    // 3. EV 계산 수행
    let calculator = EVCalculator::new(ev_config);
    let action_evs = calculator.calculate_action_evs(&internal_state);
    
    if action_evs.is_empty() {
        limitations.push("유효한 액션이 없습니다".to_string());
    }
    
    // 4. 인사이트 생성 (옵션에 따라)
    let insights = if request.options.include_insights && !action_evs.is_empty() {
        Some(generate_insights(&action_evs, &internal_state, &request.options))
    } else {
        None
    };
    
    // 5. 응답 구성
    let calculation_time = start_time.elapsed().as_millis() as u64;
    
    let ev_analysis = EVAnalysisResponse {
        action_evs,
        analysis_type: request.options.depth.clone(),
        notes: Some("상태 변환이 완전히 구현되지 않아 일부 정보가 기본값으로 설정됩니다".to_string()),
    };
    
    let metadata = AnalysisMetadata {
        calculation_time_ms: calculation_time,
        analysis_depth: request.options.depth,
        confidence_level: if limitations.is_empty() { 0.8 } else { 0.6 },
        limitations,
        game_state_valid: true,
    };
    
    Ok(PokerAnalysisResponse {
        ev_analysis,
        insights,
        metadata,
    })
}

/// 인사이트 생성
fn generate_insights(action_evs: &[ActionEV], state: &HoldemState, _options: &AnalysisOptions) -> AnalysisInsights {
    // 최고 EV 액션 찾기
    let best_action = action_evs.iter()
        .max_by(|a, b| a.ev.partial_cmp(&b.ev).unwrap_or(std::cmp::Ordering::Equal))
        .map(|a| a.action)
        .unwrap_or(Act::Fold);
    
    // 액션 강도 계산
    let mut action_strength = HashMap::new();
    let max_ev = action_evs.iter().map(|a| a.ev).fold(f64::NEG_INFINITY, f64::max);
    let min_ev = action_evs.iter().map(|a| a.ev).fold(f64::INFINITY, f64::min);
    
    for action_ev in action_evs {
        let normalized = if max_ev == min_ev {
            50.0 // 모든 액션이 같은 EV면 중간값
        } else {
            ((action_ev.ev - min_ev) / (max_ev - min_ev) * 100.0).max(0.0).min(100.0)
        };
        action_strength.insert(format!("{:?}", action_ev.action), normalized as f32);
    }
    
    // 핸드 스트렝스 계산 (현재 플레이어 기준)
    let current_player = state.to_act;
    let hole_cards = state.hole[current_player];
    let hand_strength = crate::game::card_abstraction::hand_strength(hole_cards, &state.board);
    
    // 리스크 평가
    let risk_assessment = if hand_strength > 0.8 {
        RiskLevel::Low
    } else if hand_strength > 0.6 {
        RiskLevel::Medium
    } else if hand_strength > 0.3 {
        RiskLevel::High
    } else {
        RiskLevel::Extreme
    };
    
    // 포지션별 조언
    let positional_advice = match current_player {
        0..=1 => Some("얼리 포지션: 보수적인 플레이를 권장합니다".to_string()),
        2..=3 => Some("미들 포지션: 표준적인 전략을 사용하세요".to_string()),
        4..=5 => Some("레이트 포지션: 더 공격적으로 플레이할 수 있습니다".to_string()),
        _ => None,
    };
    
    AnalysisInsights {
        recommended_action: best_action,
        action_strength,
        positional_advice,
        risk_assessment,
        hand_strength,
    }
}

/// 온디맨드 EV 분석 (기존 함수 유지)
pub fn get_on_demand_ev_analysis(
    web_state: &WebGameState,
    use_detailed_analysis: bool,
) -> Result<EVAnalysisResponse, String> {
    let analysis_request = AnalysisRequest {
        game_state: web_state.clone(),
        options: AnalysisOptions {
            depth: if use_detailed_analysis { "deep".to_string() } else { "quick".to_string() },
            include_insights: false,
            include_range_analysis: false,
            include_equity_calculation: false,
            ..Default::default()
        },
    };
    
    match analyze_poker_state(analysis_request) {
        Ok(response) => Ok(response.ev_analysis),
        Err(e) => Err(e.to_string()),
    }
}
