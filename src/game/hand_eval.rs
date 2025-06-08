// 포커 핸드 평가 모듈
// 7장 카드로 최고 5장 핸드의 랭킹 계산

/// 7장 카드 핸드 평가 함수
/// 
/// 텍사스 홀덤에서 2장 홀카드 + 5장 보드카드 = 7장으로
/// 가능한 최고 5장 핸드의 강도를 반환합니다.
/// 
/// # 매개변수
/// - cards: 7장 카드 배열 (0-51, 스페이드A=0, 하트A=13, ...)
/// 
/// # 반환값
/// - 핸드 랭킹 값 (낮을수록 강한 핸드)
/// - 1-1599: 스트레이트 플러시
/// - 1600-2499: 포카드  
/// - 2500-3824: 풀하우스
/// - 3825-5108: 플러시
/// - 5109-5863: 스트레이트
/// - 5864-8919: 트리플
/// - 8920-21293: 투페어
/// - 21294-32487: 원페어
/// - 32488-46672: 하이카드
pub fn v7(cards: [u8; 7]) -> u32 {
    // 7장에서 가능한 모든 5장 조합 평가
    let mut best_rank = u32::MAX;
    let mut best_hand = [0u8; 5];
    
    // 7C5 = 21가지 조합을 모두 확인
    for i in 0..7 {
        for j in (i+1)..7 {
            for k in (j+1)..7 {
                for l in (k+1)..7 {
                    for m in (l+1)..7 {
                        let hand = [cards[i], cards[j], cards[k], cards[l], cards[m]];
                        let rank = evaluate_5cards(hand);
                        if rank < best_rank {
                            best_rank = rank;
                            best_hand = hand;
                        }
                    }
                }
            }
        }
    }
    
    // Debug output for one pair test case
    if best_rank >= 32488 && cards.contains(&0) && cards.contains(&13) {
        println!("Debug best hand for As-Ah case: {:?} -> {}", 
                best_hand.iter().map(|&c| card_to_string(c)).collect::<Vec<_>>(), 
                rank_to_string(best_rank));
        
        // Test what a pair of Aces evaluates to
        let ace_pair_hand = [0, 13, 14, 29, 44]; // As Ah 2h 4d 6c
        let ace_pair_rank = evaluate_5cards(ace_pair_hand);
        println!("Ace pair test: {:?} -> rank {} ({})", 
                ace_pair_hand.iter().map(|&c| card_to_string(c)).collect::<Vec<_>>(),
                ace_pair_rank, rank_to_string(ace_pair_rank));
    }
    
    best_rank
}

/// 5장 카드 핸드 평가 (실제 포커 로직)
fn evaluate_5cards(cards: [u8; 5]) -> u32 {
    let mut ranks = [0u8; 5];
    let mut suits = [0u8; 5];
    let mut rank_counts = [0u8; 13];
    
    // 카드를 랭크와 수트로 분해
    for (i, &card) in cards.iter().enumerate() {
        let rank = card % 13;
        let suit = card / 13;
        ranks[i] = rank;
        suits[i] = suit;
        rank_counts[rank as usize] += 1;
    }
    

    
    // 플러시 체크
    let is_flush = suits.iter().all(|&s| s == suits[0]);
    
    // 스트레이트 체크
    let (is_straight, is_low_straight, straight_high) = check_straight(&rank_counts);
    
    // 페어/트리플 등 분석 - 개수별로 정렬
    let mut pair_counts: Vec<(u8, u8)> = rank_counts.iter().enumerate()
        .filter(|(_, &count)| count > 0)
        .map(|(rank, &count)| (count, rank as u8))
        .collect();
    pair_counts.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1))); // 개수 먼저, 그 다음 랭크
    

    
    // 핸드 타입 판정 및 순위 계산

    
    match (is_flush, is_straight || is_low_straight, &pair_counts[..]) {
        // 스트레이트 플러시
        (true, true, _) => {
            if is_low_straight {
                1599 // A-2-3-4-5 스트레이트 플러시 (가장 낮음)
            } else {
                1 + (14 - straight_high) as u32 // 높은 카드일수록 낮은 순위
            }
        },
        
        // 포카드
        (_, _, [(4, quad_rank), (1, kicker), ..]) => {
            1600 + (13 - quad_rank) as u32 * 13 + (13 - kicker) as u32
        },
        
        // 풀하우스
        (_, _, [(3, trip_rank), (2, pair_rank), ..]) => {
            2500 + (13 - trip_rank) as u32 * 13 + (13 - pair_rank) as u32
        },
        
        // 플러시
        (true, false, _) => {
            let mut flush_ranks = ranks;
            flush_ranks.sort_by(|a, b| b.cmp(a)); // 내림차순
            3825 + rank_value_sum(&flush_ranks, &[1, 2, 3, 4, 5])
        },
        
        // 스트레이트
        (false, true, _) => {
            if is_low_straight {
                5863 // A-2-3-4-5 스트레이트 (가장 낮음)
            } else {
                5109 + (14 - straight_high) as u32
            }
        },
        
        // 트리플
        (_, _, [(3, trip_rank), (1, kicker1), (1, kicker2), ..]) => {
            5864 + (13 - trip_rank) as u32 * 169 + (13 - kicker1) as u32 * 13 + (13 - kicker2) as u32
        },
        
        // 투페어
        (_, _, [(2, pair1), (2, pair2), (1, kicker), ..]) => {
            8920 + (13 - pair1) as u32 * 169 + (13 - pair2) as u32 * 13 + (13 - kicker) as u32
        },
        
        // 원페어 - 정확히 1개의 페어와 3개의 킥커가 있는 경우
        (_, _, [(2, pair_rank), (1, k1), (1, k2), (1, k3)]) => {
            let rank = 21294 + (13 - pair_rank) as u32 * 715 + 
                    (13 - k1) as u32 * 55 + 
                    (13 - k2) as u32 * 4 + 
                    (13 - k3) as u32;
            rank
        },
        
        // 하이카드
        _ => {
            let mut sorted_ranks = ranks;
            sorted_ranks.sort_by(|a, b| b.cmp(a));
            32488 + rank_value_sum(&sorted_ranks, &[1, 2, 3, 4, 5])
        }
    }
}

/// 스트레이트 체크 (개선된 버전)
fn check_straight(rank_counts: &[u8; 13]) -> (bool, bool, u8) {
    // A-2-3-4-5 로우 스트레이트 체크
    let is_low_straight = rank_counts[0] > 0 && rank_counts[1] > 0 && 
                         rank_counts[2] > 0 && rank_counts[3] > 0 && rank_counts[4] > 0;
    
    // 일반 스트레이트 체크 (5-6-7-8-9부터 10-J-Q-K-A까지)
    let mut consecutive = 0;
    let mut straight_high = 0;
    
    for i in 0..13 {
        if rank_counts[i] > 0 {
            consecutive += 1;
            if consecutive >= 5 {
                // 스트레이트의 하이카드는 연속된 5장 중 가장 높은 카드
                straight_high = i as u8;
                return (true, is_low_straight, straight_high);
            }
        } else {
            consecutive = 0;
        }
    }
    
    // 10-J-Q-K-A 스트레이트 체크 (Ace가 하이카드인 경우)
    if rank_counts[9] > 0 && rank_counts[10] > 0 && rank_counts[11] > 0 && 
       rank_counts[12] > 0 && rank_counts[0] > 0 {
        return (true, false, 0); // Ace 하이 스트레이트에서 Ace는 랭크 0이지만 실제로는 14로 취급
    }
    
    (false, is_low_straight, straight_high)
}


/// 랭크 값 합계 계산 (타이브레이커용)
fn rank_value_sum(ranks: &[u8], multipliers: &[u32]) -> u32 {
    ranks.iter().zip(multipliers.iter())
        .map(|(&rank, &mult)| (13 - rank) as u32 * mult)
        .sum()
}

/// 7장 카드 핸드 평가 - v7()의 별칭
/// 
/// lib.rs와의 호환성을 위한 함수명
/// v7() 함수와 동일한 기능을 수행합니다.
/// 
/// # 매개변수
/// - cards: 7장 카드 배열
/// 
/// # 반환값  
/// - 핸드 랭킹 값 (낮을수록 강한 핸드)
pub fn evaluate_7cards(cards: [u8; 7]) -> u32 {
    v7(cards)
}

/// 핸드 강도를 텍스트로 변환
/// 
/// # 매개변수
/// - rank: v7() 함수로부터 받은 핸드 랭킹 값
/// 
/// # 반환값
/// - 핸드 이름 문자열
pub fn rank_to_string(rank: u32) -> &'static str {
    match rank {
        1..=1599 => "스트레이트 플러시",
        1600..=2499 => "포카드",
        2500..=3824 => "풀하우스", 
        3825..=5108 => "플러시",
        5109..=5863 => "스트레이트",
        5864..=8919 => "트리플",
        8920..=21293 => "투페어",
        21294..=32487 => "원페어",
        _ => "하이카드",
    }
}

/// 카드 번호를 텍스트로 변환
/// 
/// # 매개변수  
/// - card: 카드 번호 (0-51)
/// 
/// # 반환값
/// - 카드 이름 (예: "As", "Kh", "2c")
pub fn card_to_string(card: u8) -> String {
    let suit = card / 13;
    let rank = card % 13;
    
    let rank_str = match rank {
        0 => "A",
        1..=9 => &(rank + 1).to_string(),
        10 => "J", 
        11 => "Q",
        12 => "K",
        _ => "?",
    };
    
    let suit_str = match suit {
        0 => "s",  // 스페이드
        1 => "h",  // 하트  
        2 => "d",  // 다이아몬드
        3 => "c",  // 클럽
        _ => "?",
    };
    
    format!("{}{}", rank_str, suit_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hand_evaluation() {
        // 테스트 케이스 1: 로얄 스트레이트 플러시 (10s Js Qs Ks As + 2개 더미)
        let royal_flush = [9, 10, 11, 12, 0, 13, 14]; // 스페이드 10,J,Q,K,A + 하트A,2
        let rank = v7(royal_flush);
        println!("로얄 플러시 랭크: {}, 타입: {}", rank, rank_to_string(rank));
        assert!(rank >= 1 && rank <= 1599, "로얄 플러시는 스트레이트 플러시여야 함");
        
        // 테스트 케이스 2: 포카드 (As Ah Ad Ac + 3개 더미)
        let four_aces = [0, 13, 26, 39, 1, 2, 3]; // 4장의 에이스 + 더미카드
        let rank = v7(four_aces);
        println!("포카드 랭크: {}, 타입: {}", rank, rank_to_string(rank));
        assert!(rank >= 1600 && rank <= 2499, "포카드여야 함");
        
        // 테스트 케이스 3: 풀하우스 (As Ah Ad Ks Kh)
        let full_house = [0, 13, 26, 12, 25, 1, 2]; // AAA KK + 더미
        let rank = v7(full_house);
        println!("풀하우스 랭크: {}, 타입: {}", rank, rank_to_string(rank));
        assert!(rank >= 2500 && rank <= 3824, "풀하우스여야 함");
        
        // 테스트 케이스 4: 플러시 (모든 스페이드, 스트레이트 아님)
        let flush = [0, 2, 4, 6, 8, 13, 14]; // A,3,5,7,9 스페이드 + 더미
        let rank = v7(flush);
        println!("플러시 랭크: {}, 타입: {}", rank, rank_to_string(rank));
        assert!(rank >= 3825 && rank <= 5108, "플러시여야 함");
        
        // 테스트 케이스 5: 스트레이트 (6-7-8-9-10, 플러시 아님)
        let straight = [5, 6+13, 7+26, 8, 9+13, 13, 14]; // 6s,7h,8d,9s,10h + 더미  
        let rank = v7(straight);
        println!("스트레이트 랭크: {}, 타입: {}", rank, rank_to_string(rank));
        assert!(rank >= 5109 && rank <= 5863, "스트레이트여야 함");
        
        // 테스트 케이스 6: 투페어 (As Ah Ks Kh + 다른 수트 더미)
        let two_pair = [0, 13, 12, 25, 1+13, 2+26, 3+39]; // As Ah Ks Kh + 다른 수트 더미
        let rank = v7(two_pair);
        println!("투페어 랭크: {}, 타입: {}", rank, rank_to_string(rank));
        assert!(rank >= 8920 && rank <= 21293, "투페어여야 함");
        
        // 테스트 케이스 7: 원페어 (As Ah + 낮은 카드들로 확실하게)
        let one_pair = [0, 13, 1+13, 3+26, 5+39, 7+13, 9+26]; // As Ah + 2,4,6,8,10 (스트레이트 불가능)
        println!("One pair test cards: {:?}", one_pair.iter().map(|&c| card_to_string(c)).collect::<Vec<_>>());
        let rank = v7(one_pair);
        println!("원페어 랭크: {}, 타입: {}", rank, rank_to_string(rank));
        assert!(rank >= 21294 && rank <= 32487, "원페어여야 함");
        
        // 테스트 케이스 8: 하이카드 (완전히 연결되지 않는 카드들)
        let high_card = [0, 2+13, 4+26, 7+39, 9+13, 11+26, 12+39]; // A,3,5,8,10,Q,K 다른 수트
        let rank = v7(high_card);
        println!("하이카드 랭크: {}, 타입: {}", rank, rank_to_string(rank));
        assert!(rank >= 32488, "하이카드여야 함");
        
        println!("모든 핸드 평가 테스트 통과!");
    }
    
    #[test]
    fn test_card_conversion() {
        assert_eq!(card_to_string(0), "As");   // 스페이드 A
        assert_eq!(card_to_string(12), "Ks");  // 스페이드 K
        assert_eq!(card_to_string(13), "Ah");  // 하트 A
        assert_eq!(card_to_string(51), "Kc");  // 클럽 K
        
        println!("카드 변환 테스트 통과");
    }
}
