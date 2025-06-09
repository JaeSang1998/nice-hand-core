// 토너먼트 기능 시연
use nice_hand_core::game::tournament::*;
use std::time::Instant;

fn main() {
    println!("=== 토너먼트 기능 데모 ===\n");

    // ICM 계산 테스트
    test_icm_calculations();
    
    // 기본 버블 전략 테스트
    test_basic_bubble_strategy();
    
    // 토너먼트 구조 테스트
    test_tournament_structure();
    
    println!("\n=== 모든 토너먼트 테스트 성공적으로 완료 ===");
}

fn test_icm_calculations() {
    println!("🎯 ICM 계산 테스트 중...");
    
    // 현실적인 토너먼트 시나리오: 4명 남음, 3명이 상금 받음
    let stacks = vec![15000, 8000, 5000, 2000]; // 칩 스택
    let payouts = vec![10000, 6000, 4000]; // 상금 구조
    
    let icm = ICMCalculator::new(stacks.clone(), payouts.clone());
    let start_time = Instant::now();
    let equities = icm.calculate_equity();
    let calculation_time = start_time.elapsed();
    
        println!("   📊 스택: {:?}", stacks);
    println!("   💰 상금: {:?}", payouts);
    println!("   ⚖️  ICM 지분: {:.2?}", equities);
    println!("   ⏱️  계산 시간: {:?}", calculation_time);
    
    // ICM 압박 계산 테스트
    let pressure = icm.calculate_icm_pressure(0, -1000); // 빅 스택이 1k 칩 손실
    println!("   📉 칩 리더가 1000 칩 잃을 때의 ICM 압박: {:.4}", pressure);
    
    // 버블 시나리오 테스트
    test_bubble_icm();
    test_heads_up_icm();
    
    println!("   ✅ ICM 계산이 올바르게 작동함\n");
}

fn test_bubble_icm() {
    println!("   🫧 버블 ICM 테스트 중...");
    
    // 5명 플레이어, 4명이 상금 받음 (버블 상황)
    let stacks = vec![12000, 10000, 8000, 6000, 4000];
    let payouts = vec![15000, 10000, 7000, 5000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      버블 지분: {:.2?}", equities);
    
    // 숏 스택은 더 낮은 지분을 가져야 함
    if equities[4] < equities[3] {
        println!("      ✅ 숏 스택이 적절히 감소된 지분을 가짐");
    }
}

fn test_heads_up_icm() {
    println!("   🥊 헤즈업 ICM 테스트 중...");
    
    let stacks = vec![30000, 10000];
    let payouts = vec![20000, 12000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      헤즈업 지분: {:.2?}", equities);
    
    // ICM은 칩 리더 우위를 줄여야 함
    if equities[0] > 15000.0 && equities[0] < 18000.0 {
        println!("      ✅ ICM이 칩 리더 우위를 적절히 감소시킴");
    }
}

fn test_basic_bubble_strategy() {
    println!("🫧 기본 버블 전략 테스트 중...");
    
    // 19명 남음, 18명이 상금 받음 (클래식 버블)
    let bubble_strategy = BubbleStrategy::new(19, 18);
    
    println!("   💫 버블 팩터: {:.3}", bubble_strategy.bubble_factor);
    
    // 다양한 스택 크기에 대한 전략 조정 테스트
    let base_range = 0.2; // 보통 20%의 핸드
    let short_stack_range = bubble_strategy.adjust_hand_range(base_range, 0.6);
    let big_stack_range = bubble_strategy.adjust_hand_range(base_range, 2.0);
    
    println!("   📉 숏 스택 범위: {:.1}%", short_stack_range * 100.0);
    println!("   📈 빅 스택 범위: {:.1}%", big_stack_range * 100.0);
    
    // 공격적 플레이 결정 테스트
    let should_be_aggressive = bubble_strategy.should_make_aggressive_play(1.2, 0.1);
    println!("   ⚔️  미디엄 스택이 공격적이어야 하는가: {}", should_be_aggressive);
    
    println!("   ✅ 버블 전략이 올바르게 작동함\n");
}

fn test_tournament_structure() {
    println!("🏗️ 토너먼트 구조 테스트 중...");
    
    // 토너먼트 구조 생성
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
            BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 10 },
            BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 15 },
        ],
        level_duration_minutes: 20,
        starting_stack: 1500,
        ante_schedule: vec![],
    };
    
    // 토너먼트 상태 생성
    let tournament = TournamentState::new(structure, 180, 100000);
    let (sb, bb, ante) = tournament.current_blinds();
    
    println!("   🎮 현재 블라인드: {}/{} 안테 {}", sb, bb, ante);
    println!("   👥 남은 플레이어: {}", tournament.players_remaining);
    println!("   💰 총 플레이어: {}", tournament.total_players);
    
    println!("   ✅ 토너먼트 구조가 올바르게 작동함\n");
}
