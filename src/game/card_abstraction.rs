// 카드 추상화 및 버킷팅 모듈  
// 유사한 핸드들을 그룹화하여 CFR 학습 효율성 향상

/// 카드 추상화를 위한 버킷 크기 상수
pub const PREFLOP_BUCKETS: usize = 50;    // 프리플랍 핸드 그룹 수
pub const FLOP_BUCKETS: usize = 200;      // 플랍 핸드 그룹 수  
pub const TURN_BUCKETS: usize = 200;      // 턴 핸드 그룹 수
pub const RIVER_BUCKETS: usize = 200;     // 리버 핸드 그룹 수

/// 프리플랍 핸드 강도 계산
/// 
/// 169가지 프리플랍 핸드를 50개 버킷으로 분류합니다.
/// 같은 버킷의 핸드들은 유사한 전략을 사용합니다.
/// 
/// # 매개변수
/// - hole: 2장 홀카드 [카드1, 카드2]
/// 
/// # 반환값  
/// - 버킷 번호 (0-49, 낮을수록 강한 핸드)
pub fn preflop_bucket(hole: [u8; 2]) -> u8 {
    let [c1, c2] = hole;
    let rank1 = c1 % 13;
    let rank2 = c2 % 13;
    let suited = c1 / 13 == c2 / 13;
    
    // 높은 랭크를 첫 번째로 정렬 - Ace는 가장 높은 카드로 처리
    let (high, low) = if rank1 >= rank2 { (rank1, rank2) } else { (rank2, rank1) };
    
    // 핸드 타입별 버킷 할당
    match (high, low, suited) {
        // 프리미엄 포켓 페어 (AA, KK, QQ, JJ) - Ace = 0, King = 12, Queen = 11, Jack = 10
        (0, 0, _) => 0,  // AA 
        (r, r2, _) if r == r2 && r >= 10 => 0,  // KK, QQ, JJ
        
        // 중간 포켓 페어 (TT-66)  
        (r, r2, _) if r == r2 && r >= 6 => 5,
        
        // 낮은 포켓 페어 (55-22)
        (r, r2, _) if r == r2 => 15,
        
        // 프리미엄 수트드 (AKs, AQs, AJs, KQs) - A=0, K=12, Q=11, J=10
        (12, 0, true) | (0, 12, true) => 1,  // AKs
        (11, 0, true) | (0, 11, true) => 2,  // AQs
        (10, 0, true) | (0, 10, true) => 3,  // AJs  
        (12, 11, true) => 4,  // KQs
        
        // 프리미엄 오프수트 (AK, AQ)
        (12, 0, false) | (0, 12, false) => 6, // AKo
        (11, 0, false) | (0, 11, false) => 7, // AQo
        
        // 중간 수트드 커넥터
        (r, r2, true) if r - r2 == 1 && r >= 6 => 10,
        
        // 기타 A 하이 핸드들 - A=0이므로 패턴 수정 필요
        (0, r2, _) if r2 >= 8 => 12,  // A9+
        (0, r2, _) if r2 >= 5 => 20,  // A6-A8  
        (0, _, _) => 25,  // A2-A5
        
        // 킹 하이 핸드들
        (11, r2, _) if r2 >= 9 => 18,
        (11, _, _) => 30,
        
        // 기타 핸드들
        _ => {
            let base = if suited { 35 } else { 40 };
            std::cmp::min(49, base + (13 - high) as u8)
        }
    }
}

/// 포스트플랍 핸드 강도 계산
/// 
/// 홀카드 + 보드카드 조합의 상대적 강도를 평가합니다.
/// 
/// # 매개변수
/// - hole: 2장 홀카드
/// - board: 보드카드 (3-5장)
/// 
/// # 반환값
/// - 핸드 강도 (0.0-1.0, 높을수록 강한 핸드)
pub fn hand_strength(hole: [u8; 2], board: &[u8]) -> f64 {
    if board.len() < 3 {
        // 프리플랍 핸드 강도는 버킷 기반으로 계산
        let bucket = preflop_bucket(hole);
        // 버킷이 낮을수록 강한 핸드이므로 역순으로 정규화
        return 1.0 - (bucket as f64 / PREFLOP_BUCKETS as f64);
    }
    
    // 7장 핸드 구성 (홀카드 2장 + 보드카드 최대 5장)
    let mut cards = [0u8; 7];
    cards[0] = hole[0];
    cards[1] = hole[1];
    
    for (i, &board_card) in board.iter().enumerate().take(5) {
        cards[i + 2] = board_card;
    }
    
    // 실제 핸드 평가 대신 간단한 휴리스틱 사용
    let rank = crate::hand_eval::v7(cards);
    
    // 핸드 랭킹을 0-1 범위로 정규화
    // 낮은 랭크 = 강한 핸드 = 높은 점수
    let normalized = match rank {
        1..=2000 => 0.95,      // 매우 강함 (스트레이트 플러시, 포카드)
        2001..=5000 => 0.80,   // 강함 (풀하우스, 플러시)
        5001..=10000 => 0.65,  // 중간-강함 (스트레이트, 트리플)
        10001..=25000 => 0.45, // 중간 (투페어, 강한 원페어)
        25001..=35000 => 0.25, // 약함 (약한 원페어)
        _ => 0.10,             // 매우 약함 (하이카드)
    };
    
    normalized
}

/// 포스트플랍 버킷 계산
/// 
/// 핸드 강도를 기반으로 버킷을 할당합니다.
/// 
/// # 매개변수
/// - hole: 2장 홀카드
/// - board: 보드카드
/// - street: 현재 스트리트 (1=플랍, 2=턴, 3=리버)
/// 
/// # 반환값
/// - 버킷 번호 (0-199)
pub fn postflop_bucket(hole: [u8; 2], board: &[u8], street: u8) -> u8 {
    let strength = hand_strength(hole, board);
    let bucket_count = match street {
        1 => FLOP_BUCKETS,
        2 => TURN_BUCKETS, 
        3 => RIVER_BUCKETS,
        _ => FLOP_BUCKETS,
    };
    
    // 강도를 버킷 번호로 변환 (0 = 가장 강한 버킷)
    let bucket = ((1.0 - strength) * bucket_count as f64) as u8;
    std::cmp::min(bucket, (bucket_count - 1) as u8)
}

/// 드로우 가능성 평가 (플러시, 스트레이트 드로우)
/// 
/// # 매개변수
/// - hole: 2장 홀카드
/// - board: 보드카드 (플랍/턴)
/// 
/// # 반환값
/// - 드로우 점수 (0.0-1.0, 높을수록 강한 드로우)
pub fn draw_potential(hole: [u8; 2], board: &[u8]) -> f64 {
    if board.len() < 3 {
        return 0.0;
    }
    
    let mut all_cards = Vec::new();
    all_cards.extend_from_slice(&hole);
    all_cards.extend_from_slice(board);
    
    // 수트 분포 계산 (플러시 드로우)
    let mut suit_counts = [0u8; 4];
    for &card in &all_cards {
        suit_counts[(card / 13) as usize] += 1;
    }
    let max_suit = *suit_counts.iter().max().unwrap();
    
    // 연속 카드 계산 (스트레이트 드로우)  
    let mut rank_bits = 0u16;
    for &card in &all_cards {
        rank_bits |= 1 << (card % 13);
    }
    
    let straight_potential = count_straight_draws(rank_bits) as f64 / 8.0;
    let flush_potential = if max_suit >= 4 { 1.0 } else { (max_suit as f64 - 2.0) / 2.0 };
    
    (straight_potential + flush_potential) / 2.0
}

/// 스트레이트 드로우 계산 보조 함수
fn count_straight_draws(rank_bits: u16) -> u8 {
    let mut draws = 0;
    
    // 각 가능한 스트레이트 시작점 확인
    for start in 0..=8 {
        let straight_mask = 0x1F << start; // 연속 5장 마스크
        let overlap = rank_bits & straight_mask;
        let count = overlap.count_ones();
        
        if count >= 3 {
            draws += 1;
        }
    }
    
    // A-2-3-4-5 스트레이트 (wheel) 특별 처리
    let wheel_mask = 0x100F; // A,2,3,4,5
    let wheel_overlap = rank_bits & wheel_mask;
    if wheel_overlap.count_ones() >= 3 {
        draws += 1;
    }
    
    draws
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_preflop_buckets() {
        // AA (포켓 에이스) - 최고 버킷
        assert_eq!(preflop_bucket([0, 13]), 0);
        
        // AKs (수트드 에이스킹) - 프리미엄 버킷
        assert_eq!(preflop_bucket([0, 12]), 1);
        
        // 72o (worst hand) - 낮은 버킷
        let bucket_72o = preflop_bucket([5, 14]);
        assert!(bucket_72o > 40);
        
        println!("프리플랍 버킷 테스트 통과");
    }
    
    #[test]
    fn test_hand_strength() {
        // 강한 핸드 (포켓 에이스) - 보드: 2s, 3h, 3d
        let strength_aa = hand_strength([0, 13], &[2, 15, 28]);
        println!("AA vs 2s3h3d - 스트렝스: {}", strength_aa);
        
        // 약한 핸드 (실제 7-2) - 보드: Ks, Qh, Jd (무관한 하이카드 보드)
        let strength_72 = hand_strength([6, 14], &[12, 24, 37]); // 7s, 2h vs Ks, Qh, Jd
        println!("72o 핸드: 스트렝스: {}", strength_72);
        
        // 카드 확인
        println!("홀카드 - 카드 6: {}, 카드 14: {}", 
                 crate::hand_eval::card_to_string(6), 
                 crate::hand_eval::card_to_string(14));
        println!("보드카드 - 카드 12: {}, 카드 24: {}, 카드 37: {}", 
                 crate::hand_eval::card_to_string(12), 
                 crate::hand_eval::card_to_string(24), 
                 crate::hand_eval::card_to_string(37));
        
        // AA with pair on board should still be quite strong
        assert!(strength_aa > 0.4);
        
        // 7-2 should be weaker than AA
        assert!(strength_72 < strength_aa);
        
        println!("핸드 강도 테스트 통과");
    }
    
    #[test]
    fn test_postflop_buckets() {
        let hole = [0, 13]; // AA
        let board = [2, 15, 28]; // 2s, 3h, 3d
        
        let bucket = postflop_bucket(hole, &board, 1);
        println!("포스트플랍 버킷: {}", bucket);
        
        // AA with pair on board should still be in strong bucket range (low numbers)
        // With 200 buckets (0-199), strong hands should be in lower buckets
        assert!(bucket < 150); // 강한 핸드는 낮은 버킷 번호
        
        println!("포스트플랍 버킷 테스트 통과");
    }
}
