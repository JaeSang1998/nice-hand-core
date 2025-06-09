use nice_hand_core::game::tournament::*;

/// 확장 멀티테이블 토너먼트 (MTT) 데모
/// 
/// 이 예제는 다음을 보여줍니다:
/// - 멀티테이블 밸런싱 알고리즘
/// - 플레이어 이동 및 통합
/// - 테이블 브레이킹 로직
/// - 실시간 토너먼트 진행

fn main() {
    println!("=== 확장 멀티테이블 토너먼트 데모 ===\n");

    // 여러 테이블로 대형 토너먼트 생성
    let mut mtt_manager = create_large_tournament();
    
    // 테이블 밸런싱 시연
    demonstrate_table_balancing(&mut mtt_manager);
    
    // 토너먼트 진행 시뮬레이션
    simulate_tournament_progression(&mut mtt_manager);
    
    // 파이널 테이블 역학 보여주기
    demonstrate_final_table(&mut mtt_manager);
}

fn create_large_tournament() -> MTTManager {
    println!("20개 테이블로 180명 플레이어 토너먼트 생성 중...");
    
    // 토너먼트 구조 생성
    let tournament_structure = TournamentStructure {
        levels: create_extended_blind_structure(),
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    // 180명 플레이어, 테이블당 9명, 상금 풀 $18,000로 MTT 생성
    let mtt_manager = MTTManager::new(180, 9, tournament_structure, 18000);
    
    println!("{} 개 테이블에 {} 명 플레이어로 토너먼트 생성됨", 
             mtt_manager.tables.len(), 180);
    println!();
    
    mtt_manager
}

fn create_extended_blind_structure() -> Vec<BlindLevel> {
    vec![
        BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
        BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 0 },
        BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 0 },
        BlindLevel { level: 4, small_blind: 100, big_blind: 200, ante: 25 },
        BlindLevel { level: 5, small_blind: 150, big_blind: 300, ante: 25 },
        BlindLevel { level: 6, small_blind: 200, big_blind: 400, ante: 50 },
        BlindLevel { level: 7, small_blind: 300, big_blind: 600, ante: 75 },
        BlindLevel { level: 8, small_blind: 400, big_blind: 800, ante: 100 },
        BlindLevel { level: 9, small_blind: 500, big_blind: 1000, ante: 100 },
        BlindLevel { level: 10, small_blind: 600, big_blind: 1200, ante: 150 },
        BlindLevel { level: 11, small_blind: 800, big_blind: 1600, ante: 200 },
        BlindLevel { level: 12, small_blind: 1000, big_blind: 2000, ante: 250 },
        BlindLevel { level: 13, small_blind: 1500, big_blind: 3000, ante: 400 },
        BlindLevel { level: 14, small_blind: 2000, big_blind: 4000, ante: 500 },
        BlindLevel { level: 15, small_blind: 3000, big_blind: 6000, ante: 750 },
    ]
}

fn create_mtt_payout_structure(total_players: usize) -> Vec<f64> {
    let total_prize_pool = total_players as f64 * 100.0; // $100 buy-in
    let paid_positions = (total_players / 10).max(15); // Pay top 10% or minimum 15
    
    let mut payouts = Vec::new();
    
    for i in 0..paid_positions {
        let percentage = match i {
            0 => 0.25,    // 1st place: 25%
            1 => 0.15,    // 2nd place: 15%
            2 => 0.10,    // 3rd place: 10%
            3..=5 => 0.07,   // 4th-6th: 7% each
            6..=8 => 0.04,   // 7th-9th: 4% each
            9..=14 => 0.025, // 10th-15th: 2.5% each
            _ => 0.015,      // Others: 1.5% each
        };
        payouts.push(total_prize_pool * percentage);
    }
    
    payouts
}

fn demonstrate_table_balancing(mtt_manager: &mut MTTManager) {
    println!("=== 테이블 밸런싱 시연 ===");
    
    // 리밸런싱을 트리거하기 위해 일부 탈락 시뮬레이션
    println!("테이블 밸런싱을 트리거하기 위해 탈락 시뮬레이션 중...");
    
    // 테이블 0에서 3명 탈락
    eliminate_players_from_table(mtt_manager, 0, 3);
    
    // 테이블 1에서 4명 탈락
    eliminate_players_from_table(mtt_manager, 1, 4);
    
    // 테이블 2에서 2명 탈락
    eliminate_players_from_table(mtt_manager, 2, 2);
    
    println!("밸런싱 전:");
    print_table_summary(mtt_manager);
    
    // 테이블 밸런싱 트리거
    mtt_manager.balance_tables();
    
    println!("밸런싱 후:");
    print_table_summary(mtt_manager);
    println!();
}

fn eliminate_players_from_table(mtt_manager: &mut MTTManager, table_id: usize, count: usize) {
    for i in 0..count {
        mtt_manager.eliminate_player(table_id as u32, (i + 1) as u32);
    }
    println!("테이블 {}에서 {} 명 탈락", table_id, count);
}

fn print_table_summary(mtt_manager: &MTTManager) {
    for (i, table) in mtt_manager.tables.iter().enumerate() {
        let player_count = table.count_active_players();
        let total_chips: u32 = table.seats.iter()
            .filter_map(|seat| seat.as_ref())
            .filter(|player| !player.is_sitting_out && player.stack_size > 0)
            .map(|player| player.stack_size)
            .sum();
        
        let avg_stack = if player_count > 0 {
            total_chips / player_count
        } else { 0 };
        
        println!("  테이블 {}: {} 명, 평균 스택: {}", i, player_count, avg_stack);
    }
}

fn simulate_tournament_progression(mtt_manager: &mut MTTManager) {
    println!("=== 토너먼트 진행 시뮬레이션 ===");
    
    let mut eliminations = 0;
    let total_players = 180;
    
    // 파이널 테이블까지 진행 시뮬레이션
    while mtt_manager.count_active_players() > 9 && eliminations < 100 {
        // 몇 분마다 탈락 시뮬레이션
        let table_to_eliminate_from = eliminations % mtt_manager.tables.len();
        
        // 플레이어 탈락 시도 (순차 플레이어 ID 사용)
        let player_id_to_eliminate = (eliminations + 1) as u32;
        mtt_manager.eliminate_player(table_to_eliminate_from as u32, player_id_to_eliminate);
        eliminations += 1;
        
        // 20명 탈락마다 진행상황 보여주기
        if eliminations % 20 == 0 {
            let remaining = mtt_manager.count_active_players();
            let progress = ((total_players - remaining) as f64 / total_players as f64) * 100.0;
            println!("진행률: {:.1}% - {} 명 남음", progress, remaining);
            
            // 테이블 통합 보여주기
            if mtt_manager.tables.len() < 10 {
                print_table_summary(mtt_manager);
            }
        }
        
        // 5명 탈락마다 테이블 리밸런싱
        if eliminations % 5 == 0 {
            mtt_manager.balance_tables();
        }
    }
    
    println!("파이널 테이블 단계에 {} 명으로 도달!", mtt_manager.count_active_players());
    println!();
}

fn demonstrate_final_table(mtt_manager: &mut MTTManager) {
    println!("=== 파이널 테이블 역학 ===");
    
    let remaining = mtt_manager.count_active_players();
    println!("파이널 테이블에 {} 명으로 도달", remaining);
    
    // 토너먼트 순위를 사용한 파이널 테이블 ICM 계산 보여주기
    let standings = mtt_manager.get_tournament_standings();
    let stacks: Vec<u32> = standings.iter().map(|(_, stack, _)| *stack).collect();
    
    if !stacks.is_empty() {
        let total_chips: u32 = stacks.iter().sum();
        
        println!("파이널 테이블 칩 분포:");
        for (i, &stack) in stacks.iter().enumerate() {
            let bb_count = stack / 1000; // 500/1000 블라인드 가정
            let percentage = (stack as f64 / total_chips as f64) * 100.0;
            println!("  플레이어 {}: {} 칩 ({:.1}%, {} BB)", 
                     i + 1, stack, percentage, bb_count);
        }
        
        // ICM 값 계산
        let payout_structure = create_mtt_payout_structure(180);
        let final_payouts: Vec<f64> = payout_structure.into_iter().take(stacks.len()).collect();
        
        let icm_calculator = ICMCalculator::new(stacks.clone(), final_payouts.iter().map(|&x| x as u64).collect());
        let icm_values = icm_calculator.calculate_equity();
        
        println!("\nICM 값:");
        for (i, &icm_value) in icm_values.iter().enumerate() {
            let chip_value = (stacks[i] as f64 / total_chips as f64) * final_payouts.iter().sum::<f64>();
            let icm_pressure = ((icm_value - chip_value) / chip_value) * 100.0;
            println!("  플레이어 {}: ${:.2} (칩 EV: ${:.2}, ICM 압박: {:.1}%)", 
                     i + 1, icm_value, chip_value, icm_pressure);
        }
    }
    
    println!("\n=== 토너먼트 완료 ===");
}
