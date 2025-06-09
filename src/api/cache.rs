// 캐싱 시스템 - EV 계산 결과 캐싱을 통한 성능 최적화
// 동일한 게임 상황에 대해 반복적인 계산을 피합니다

use crate::api::analysis::{PokerAnalysisResponse, AnalysisRequest};
use crate::api::web_api::WebGameState;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};

/// 게임 상태를 식별하는 시그니처
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSignature {
    /// 플레이어 정보 해시
    players_hash: u64,
    /// 보드 카드 해시
    board_hash: u64,
    /// 팟 크기
    pot: u32,
    /// 현재 스트리트
    street: u8,
    /// 액션할 플레이어
    to_act: usize,
}

impl StateSignature {
    /// WebGameState로부터 시그니처 생성
    pub fn from_web_state(web_state: &WebGameState) -> Self {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        // 플레이어 정보 해시
        let mut players_hasher = DefaultHasher::new();
        web_state.stacks.hash(&mut players_hasher);
        web_state.alive_players.hash(&mut players_hasher);
        web_state.street_investments.hash(&mut players_hasher);
        let players_hash = players_hasher.finish();
        
        // 보드 카드 해시
        let mut board_hasher = DefaultHasher::new();
        web_state.board.hash(&mut board_hasher);
        web_state.hole_cards.hash(&mut board_hasher);
        let board_hash = board_hasher.finish();
        
        StateSignature {
            players_hash,
            board_hash,
            pot: web_state.pot,
            street: web_state.street,
            to_act: web_state.player_to_act,
        }
    }
}

/// 캐시 엔트리
#[derive(Debug, Clone)]
struct CacheEntry {
    /// 캐시된 분석 결과
    result: PokerAnalysisResponse,
    /// 캐시 생성 시간
    created_at: Instant,
    /// 마지막 액세스 시간 (LRU용)
    last_accessed: Instant,
    /// 액세스 횟수
    access_count: u32,
}

impl CacheEntry {
    fn new(result: PokerAnalysisResponse) -> Self {
        let now = Instant::now();
        Self {
            result,
            created_at: now,
            last_accessed: now,
            access_count: 1,
        }
    }
    
    fn access(&mut self) -> PokerAnalysisResponse {
        self.last_accessed = Instant::now();
        self.access_count += 1;
        self.result.clone()
    }
    
    fn is_expired(&self, max_age: Duration) -> bool {
        self.created_at.elapsed() > max_age
    }
}

/// 캐시 설정
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// 최대 캐시 크기
    pub max_size: usize,
    /// 최대 캐시 보관 시간
    pub max_age: Duration,
    /// 정리 주기
    pub cleanup_interval: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            max_age: Duration::from_secs(300), // 5분
            cleanup_interval: Duration::from_secs(60), // 1분
        }
    }
}

/// 캐시된 EV 분석 서비스
pub struct CachedAnalysisService {
    cache: Arc<Mutex<HashMap<StateSignature, CacheEntry>>>,
    config: CacheConfig,
    last_cleanup: Arc<Mutex<Instant>>,
}

impl CachedAnalysisService {
    /// 새로운 캐시 서비스 생성
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            config,
            last_cleanup: Arc::new(Mutex::new(Instant::now())),
        }
    }
    
    /// 기본 설정으로 캐시 서비스 생성
    pub fn default() -> Self {
        Self::new(CacheConfig::default())
    }
    
    /// 분석 결과 가져오기 (캐시 우선)
    pub fn get_analysis(&self, request: AnalysisRequest) -> Result<PokerAnalysisResponse, String> {
        let signature = StateSignature::from_web_state(&request.game_state);
        
        // 정리 작업 확인
        self.maybe_cleanup();
        
        // 캐시 확인
        if let Ok(mut cache) = self.cache.lock() {
            if let Some(entry) = cache.get_mut(&signature) {
                if !entry.is_expired(self.config.max_age) {
                    return Ok(entry.access());
                } else {
                    // 만료된 엔트리 제거
                    cache.remove(&signature);
                }
            }
        }
        
        // 캐시 미스 - 실제 계산 수행
        let result = crate::api::analysis::analyze_poker_state(request)
            .map_err(|e| e.to_string())?;
        
        // 결과 캐싱
        self.cache_result(signature, result.clone());
        
        Ok(result)
    }
    
    /// 결과를 캐시에 저장
    fn cache_result(&self, signature: StateSignature, result: PokerAnalysisResponse) {
        if let Ok(mut cache) = self.cache.lock() {
            // 캐시 크기 확인
            if cache.len() >= self.config.max_size {
                self.evict_lru(&mut cache);
            }
            
            cache.insert(signature, CacheEntry::new(result));
        }
    }
    
    /// LRU 정책으로 캐시 엔트리 제거
    fn evict_lru(&self, cache: &mut HashMap<StateSignature, CacheEntry>) {
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();
        
        for (key, entry) in cache.iter() {
            if entry.last_accessed < oldest_time {
                oldest_time = entry.last_accessed;
                oldest_key = Some(key.clone());
            }
        }
        
        if let Some(key) = oldest_key {
            cache.remove(&key);
        }
    }
    
    /// 정리 작업 수행 (만료된 엔트리 제거)
    fn maybe_cleanup(&self) {
        if let Ok(mut last_cleanup) = self.last_cleanup.lock() {
            if last_cleanup.elapsed() > self.config.cleanup_interval {
                *last_cleanup = Instant::now();
                drop(last_cleanup); // 락 해제
                
                self.cleanup_expired();
            }
        }
    }
    
    /// 만료된 엔트리들 정리
    fn cleanup_expired(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            let expired_keys: Vec<_> = cache
                .iter()
                .filter(|(_, entry)| entry.is_expired(self.config.max_age))
                .map(|(key, _)| key.clone())
                .collect();
            
            for key in expired_keys {
                cache.remove(&key);
            }
        }
    }
    
    /// 캐시 통계 조회
    pub fn get_stats(&self) -> CacheStats {
        if let Ok(cache) = self.cache.lock() {
            let total_access_count: u32 = cache.values().map(|entry| entry.access_count).sum();
            
            CacheStats {
                entries_count: cache.len(),
                total_access_count,
                average_access_per_entry: if cache.is_empty() {
                    0.0
                } else {
                    total_access_count as f64 / cache.len() as f64
                },
            }
        } else {
            CacheStats {
                entries_count: 0,
                total_access_count: 0,
                average_access_per_entry: 0.0,
            }
        }
    }
    
    /// 캐시 비우기
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
}

/// 캐시 통계
#[derive(Debug, Serialize)]
pub struct CacheStats {
    /// 캐시 엔트리 수
    pub entries_count: usize,
    /// 총 액세스 수
    pub total_access_count: u32,
    /// 엔트리당 평균 액세스 수
    pub average_access_per_entry: f64,
}

lazy_static::lazy_static! {
    /// 글로벌 캐시 인스턴스 (싱글톤)
    static ref GLOBAL_CACHE: CachedAnalysisService = CachedAnalysisService::default();
}

/// 글로벌 캐시를 사용한 분석 함수
/// 
/// # 예제
/// ```
/// use nice_hand_core::cached_analysis;
/// 
/// let request = AnalysisRequest { /* ... */ };
/// let result = cached_analysis(request)?;
/// ```
pub fn cached_analysis(request: AnalysisRequest) -> Result<PokerAnalysisResponse, String> {
    GLOBAL_CACHE.get_analysis(request)
}

/// 글로벌 캐시 통계 조회
pub fn get_cache_stats() -> CacheStats {
    GLOBAL_CACHE.get_stats()
}

/// 글로벌 캐시 비우기
pub fn clear_cache() {
    GLOBAL_CACHE.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::web_api::WebGameState;
    use crate::api::analysis::{AnalysisRequest, AnalysisOptions, OpponentModel};

    #[test]
    fn test_state_signature() {
        let web_state = WebGameState {
            hole_cards: [0, 1],
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
        
        let sig1 = StateSignature::from_web_state(&web_state);
        let sig2 = StateSignature::from_web_state(&web_state);
        
        assert_eq!(sig1, sig2);
    }
    
    #[test]
    fn test_cache_functionality() {
        let cache_service = CachedAnalysisService::default();
        
        let web_state = WebGameState {
            hole_cards: [0, 1],
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
        
        let request = AnalysisRequest {
            game_state: web_state,
            options: AnalysisOptions {
                depth: "quick".to_string(),
                include_insights: true,
                include_range_analysis: false,
                include_equity_calculation: false,
                max_calculation_time_ms: None,
                opponent_modeling: OpponentModel::Tight,
            },
        };
        
        // 첫 번째 요청 (캐시 미스)
        let result1 = cache_service.get_analysis(request.clone());
        assert!(result1.is_ok());
        
        // 두 번째 요청 (캐시 히트)
        let result2 = cache_service.get_analysis(request);
        assert!(result2.is_ok());
        
        let stats = cache_service.get_stats();
        assert_eq!(stats.entries_count, 1);
        assert_eq!(stats.total_access_count, 2);
    }
}
