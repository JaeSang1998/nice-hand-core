use nice_hand_core::game::tournament::*;
use nice_hand_core::ICMCalculator;

/// 버블 전략 최적화 도구
/// 
/// 이 예제는 다음을 보여줍니다:
/// - 최적 버블 플레이 전략
/// - 스택별 조정
/// - 포지션 및 상대방 인식 버블 플레이
/// - 버블 결정의 수학적 분석

fn main() {
    println!("=== 버블 전략 최적화 ===\n");

    // 다양한 버블 시나리오 분석
    analyze_standard_bubble();
    analyze_super_bubble();
    analyze_stone_bubble();
    optimize_bubble_strategies();
    analyze_position_on_bubble();
}

fn analyze_standard_bubble() {
    println!("=== 표준 버블 분석 ===");
    println!("시나리오: 4명 플레이어, 3명 상금, 상당히 균등한 스택\n");

    let stacks = vec![6000, 5000, 4500, 4500];
    let payouts = vec![50000, 30000, 20000]; // u64로 변환됨
    let blind_level = BlindLevel { level: 1, small_blind: 200, big_blind: 400, ante: 50 };

    let bubble_strategy = BubbleStrategy::new(stacks.len() as u32, payouts.len() as u32);
    
    println!("스택 분포:");
    for (i, &stack) in stacks.iter().enumerate() {
        let bb_count = stack / blind_level.big_blind;
        println!("  플레이어 {}: {} 칩 ({} BB)", i + 1, stack, bb_count);
    }
    println!("블라인드: {}/{} + {} 앤티\n", blind_level.small_blind, blind_level.big_blind, blind_level.ante);

    // 각 플레이어의 최적 전략 분석
    for (i, &stack) in stacks.iter().enumerate() {
        analyze_player_bubble_strategy(i, stack, &stacks, &payouts, &blind_level, &bubble_strategy);
    }
}

fn analyze_super_bubble() {
    println!("=== 슈퍼 버블 분석 ===");
    println!("시나리오: 5명 플레이어, 4명 상금, 매우 짧은 스택 하나\n");

    let stacks = vec![8000, 6000, 4000, 3000, 500];
    let payouts = vec![35000, 25000, 18000, 12000]; // u64로 변환됨
    let blind_level = BlindLevel { level: 2, small_blind: 300, big_blind: 600, ante: 75 };

    let _bubble_strategy = BubbleStrategy::new(stacks.len() as u32, payouts.len() as u32);
    
    println!("슈퍼 버블 역학 (매우 짧은 스택이 보호 버블 생성):");
    
    for (i, &stack) in stacks.iter().enumerate() {
        let bb_count = stack / blind_level.big_blind;
        let protection_level = calculate_protection_level(i, &stacks);
        let strategy = get_super_bubble_strategy(i, stack, &stacks, &blind_level);
        
        println!("플레이어 {} ({} BB, 보호: {:.2}): {}", 
                 i + 1, bb_count, protection_level, strategy);
    }
}

fn analyze_stone_bubble() {
    println!("=== 스톤 버블 분석 ===");
    println!("시나리오: 6명 플레이어, 5명 상금, 매우 짧은 스택 둘\n");

    let stacks = vec![10000, 8000, 6000, 4000, 800, 600];
    let payouts = vec![35000, 25000, 18000, 12000, 10000]; // u64로 변환됨
    let blind_level = BlindLevel { level: 3, small_blind: 200, big_blind: 400, ante: 50 };

    println!("스톤 버블 시나리오 - 여러 숏스택이 중간 스택에게 극도의 보호 제공:");
    
    // 스톤 버블 팩터 계산
    let stone_factor = calculate_stone_bubble_factor(&stacks, &payouts);
    println!("스톤 버블 팩터: {:.3} (높을수록 중간 스택에게 더 많은 보호)\n", stone_factor);
    
    for (i, &stack) in stacks.iter().enumerate() {
        let bb_count = stack / blind_level.big_blind;
        let protection_level = calculate_protection_level(i, &stacks);
        let strategy = get_stone_bubble_strategy(i, stack, &stacks, protection_level);
        
        println!("플레이어 {} ({} BB, 보호: {:.2}): {}", 
                 i + 1, bb_count, protection_level, strategy);
    }
}

fn optimize_bubble_strategies() {
    println!("=== 버블 전략 최적화 ===");
    println!("각 스택 크기별 전략 최적화:\n");

    // 다양한 스택 구성 테스트
    let test_configurations = vec![
        ("숏스택 (8 BB)", vec![1200, 6000, 5000, 4000], vec![50000, 30000, 20000]),
        ("미디엄스택 (15 BB)", vec![6000, 6000, 5000, 3000], vec![50000, 30000, 20000]),
        ("빅스택 (25 BB)", vec![10000, 5000, 3000, 2000], vec![50000, 30000, 20000]),
        ("칩리더 (35 BB)", vec![14000, 3000, 2000, 1000], vec![50000, 30000, 20000]),
    ];

    for (scenario_name, stacks, payouts) in test_configurations {
        println!("{} 시나리오:", scenario_name);
        analyze_optimal_strategy_for_hero(&stacks, &payouts, 0);
        println!();
    }
}

fn analyze_position_on_bubble() {
    println!("=== 버블에서의 포지션 분석 ===");
    println!("포지션이 버블 전략에 미치는 영향:\n");

    let stacks = vec![5000, 4500, 4000, 3500];
    let _payouts = vec![50000, 30000, 20000]; // u64로 변환됨
    let blind_level = BlindLevel { level: 4, small_blind: 150, big_blind: 300, ante: 25 };

    for position in 0..stacks.len() {
        println!("포지션 {}: {}", position + 1, get_position_strategy(position, &stacks, &blind_level));
    }
}

fn analyze_player_bubble_strategy(
    player_idx: usize,
    stack: u32,
    all_stacks: &[u32],
    payouts: &[u64],
    _blind_level: &BlindLevel,
    _bubble_strategy: &BubbleStrategy
) {
    // Calculate ICM pressure
    let icm_calculator = ICMCalculator::new(all_stacks.to_vec(), payouts.to_vec());
    let icm_values = icm_calculator.calculate_equity();
    
    let total_chips: u32 = all_stacks.iter().sum();
    let chip_percentage = (stack as f64 / total_chips as f64) * 100.0;
    let icm_percentage = icm_values[player_idx] * 100.0;
    let icm_pressure = icm_percentage - chip_percentage;
    
    let strategy = match icm_pressure {
        p if p > 5.0 => "보수적 - 칩 리드 보존",
        p if p > 0.0 => "균형잡힌 - 표준 레인지",
        p if p > -5.0 => "약간 공격적 - 칩 축적",
        _ => "공격적 - 칩 절실히 필요"
    };
    
    println!("플레이어 {} 분석: {:.1}% 칩, {:.1}% ICM 에퀴티, 전략: {}",
             player_idx + 1, chip_percentage, icm_percentage, strategy);
}

fn get_super_bubble_strategy(_player_idx: usize, stack: u32, all_stacks: &[u32], _blind_level: &BlindLevel) -> &'static str {
    let total_chips: u32 = all_stacks.iter().sum();
    let stack_percentage = (stack as f64 / total_chips as f64) * 100.0;
    let shortest_stack = *all_stacks.iter().min().unwrap();
    
    if stack == shortest_stack {
        "절망적 - 괜찮은 핸드로 올인"
    } else if stack_percentage > 35.0 {
        "익스플로잇 - 약점 공격하되 큰 대결 피하기"
    } else if stack_percentage > 20.0 {
        "보호됨 - 타이트하게 플레이, 숏스택 버스트 기다리기"
    } else {
        "조심스러운 공격성 - 생존과 칩 축적의 균형"
    }
}

fn get_stone_bubble_strategy(_player_idx: usize, stack: u32, all_stacks: &[u32], protection_level: f64) -> &'static str {
    let total_chips: u32 = all_stacks.iter().sum();
    let stack_percentage = (stack as f64 / total_chips as f64) * 100.0;
    
    match (stack_percentage, protection_level) {
        (p, _pr) if p < 5.0 => "올인 모드 - 살아있는 카드 아무거나",
        (p, pr) if p < 15.0 && pr < 0.3 => "절망적 - 넓은 쇼빙 레인지", 
        (p, pr) if p < 15.0 && pr > 0.7 => "낙관적 생존 - 압박에 폴드",
        (p, pr) if p > 30.0 && pr > 0.8 => "하이퍼 공격적 - 보호 악용",
        (p, pr) if p > 20.0 && pr > 0.5 => "선택적 공격성 - 좋은 스팟 선택",
        _ => "균형잡힌 - 표준 버블 조정"
    }
}

fn calculate_protection_level(player_idx: usize, stacks: &[u32]) -> f64 {
    let player_stack = stacks[player_idx];
    let shorter_stacks = stacks.iter().filter(|&&s| s < player_stack).count();
    let total_players = stacks.len();
    
    (shorter_stacks as f64) / (total_players as f64 - 1.0)
}

fn calculate_stone_bubble_factor(stacks: &[u32], _payouts: &[u64]) -> f64 {
    let total_chips: u32 = stacks.iter().sum();
    let mut sorted_stacks = stacks.to_vec();
    sorted_stacks.sort();
    
    // 매우 짧은 스택이 몇 개 존재하는지 계산
    let avg_stack = total_chips / stacks.len() as u32;
    let very_short_threshold = avg_stack / 4;
    let very_short_count = sorted_stacks.iter().filter(|&&s| s < very_short_threshold).count();
    
    // 스톤 팩터는 더 많은 숏스택으로 증가
    very_short_count as f64 / stacks.len() as f64
}

fn analyze_optimal_strategy_for_hero(stacks: &[u32], payouts: &[u64], hero_position: usize) {
    let icm_calculator = ICMCalculator::new(stacks.to_vec(), payouts.to_vec());
    let current_icm = icm_calculator.calculate_equity();
    
    println!("  히어로 (포지션 {}): {:.1}% ICM 에퀴티", 
             hero_position + 1, current_icm[hero_position] * 100.0);
    
    // 다양한 시나리오 시뮬레이션
    for change_percentage in vec![-20, -10, 10, 20] {
        let mut new_stacks = stacks.to_vec();
        let stack_change = (stacks[hero_position] as i32 * change_percentage / 100) as i32;
        new_stacks[hero_position] = (new_stacks[hero_position] as i32 + stack_change).max(0) as u32;
        
        // 총합을 유지하기 위해 칩 재분배
        let total_change = -stack_change;
        let others_change = total_change / ((stacks.len() - 1) as i32);
        for i in 0..stacks.len() {
            if i != hero_position {
                new_stacks[i] = (new_stacks[i] as i32 + others_change).max(0) as u32;
            }
        }
        
        let new_icm = ICMCalculator::new(new_stacks, payouts.to_vec());
        let new_equities = new_icm.calculate_equity();
        let equity_change = (new_equities[hero_position] - current_icm[hero_position]) * 100.0;
        
        println!("    스택 변화 {:+}%: {:+.1}% 에퀴티 변화", change_percentage, equity_change);
    }
}

fn get_position_strategy(position: usize, _stacks: &[u32], _blind_level: &BlindLevel) -> &'static str {
    match position {
        0 => "스몰 블라인드 - 매우 타이트, 애매한 스팟 피하기",
        1 => "빅 블라인드 - 좁게 디펜드, 팟 오즈는 ICM보다 부차적",
        2 => "얼리 포지션 - 프리미엄 핸드만, 후반 스트리트 준비",
        _ => "레이트 포지션 - 타이트한 플레이 익스플로잇, 프리미엄 핸드 없이 큰 팟 피하기"
    }
}
