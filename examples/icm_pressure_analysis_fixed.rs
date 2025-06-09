use nice_hand_core::game::tournament::*;

/// ICM 압박 분석 도구
/// 
/// 이 예제는 다음을 보여줍니다:
/// - 다양한 토너먼트 시나리오에서의 ICM 계산
/// - 버블 압박 분석
/// - 상금 점프가 전략에 미치는 영향
/// - 다양한 스택 크기에서의 위험 대 보상 평가

fn main() {
    println!("=== ICM 압박 분석 도구 ===\n");

    // 다양한 토너먼트 시나리오 분석
    analyze_bubble_scenario();
    analyze_final_table_scenario();
    analyze_pay_jump_scenarios();
    analyze_chip_ev_vs_dollar_ev();
    analyze_stack_size_impact();
}

fn analyze_bubble_scenario() {
    println!("=== 버블 시나리오 분석 ===");
    println!("시나리오: 4명 플레이어 남음, 3명이 상금 받음\n");

    let stacks = vec![8000, 6000, 4000, 2000];
    let payouts = vec![5000.0, 3000.0, 2000.0];

    let icm_calculator = ICMCalculator::new(stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
    let icm_values = icm_calculator.calculate_equity();

    println!("스택 분포 및 ICM 분석:");
    let total_chips: u32 = stacks.iter().sum();
    let total_payouts: f64 = payouts.iter().sum();

    for (i, (&stack, &icm_value)) in stacks.iter().zip(&icm_values).enumerate() {
        let chip_percentage = (stack as f64 / total_chips as f64) * 100.0;
        let chip_ev = (stack as f64 / total_chips as f64) * total_payouts;
        let icm_pressure = ((icm_value - chip_ev) / chip_ev) * 100.0;
        let bubble_factor = calculate_bubble_factor(stack, &stacks, &payouts);

        println!("플레이어 {} (스택: {}):", i + 1, stack);
        println!("  칩 %: {:.1}%", chip_percentage);
        println!("  칩 EV: ${:.2}", chip_ev);
        println!("  ICM 값: ${:.2}", icm_value);
        println!("  ICM 압박: {:.1}%", icm_pressure);
        println!("  버블 팩터: {:.3}", bubble_factor);
        println!("  전략: {}", get_bubble_strategy_advice(stack, &stacks, bubble_factor));
        println!();
    }

    // 특정 상황 분석
    analyze_all_in_scenarios(&stacks, &payouts);
}

fn analyze_final_table_scenario() {
    println!("=== 파이널 테이블 분석 ===");
    println!("시나리오: 파이널 테이블 6명과 큰 상금 점프\n");

    let stacks = vec![25000, 18000, 12000, 8000, 5000, 2000];
    let payouts = vec![15000.0, 9000.0, 6000.0, 4000.0, 2500.0, 1500.0];

    let icm_calculator = ICMCalculator::new(stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
    let icm_values = icm_calculator.calculate_equity();

    println!("파이널 테이블 ICM 역학:");
    let total_chips: u32 = stacks.iter().sum();

    for (i, (&stack, &icm_value)) in stacks.iter().zip(&icm_values).enumerate() {
        let chip_ev = (stack as f64 / total_chips as f64) * payouts.iter().sum::<f64>();
        let icm_pressure = ((icm_value - chip_ev) / chip_ev) * 100.0;
        
        let next_payout_jump = if i < payouts.len() - 1 {
            payouts[i] - payouts[i + 1]
        } else {
            payouts[i]
        };

        println!("순위 {} (스택: {}):", i + 1, stack);
        println!("  ICM 값: ${:.2}", icm_value);
        println!("  ICM 압박: {:.1}%", icm_pressure);
        println!("  다음 상금 점프: ${:.2}", next_payout_jump);
        println!("  전략: {}", get_final_table_strategy(i, stack, &stacks, icm_pressure));
        println!();
    }
}

fn analyze_pay_jump_scenarios() {
    println!("=== 상금 점프 영향 분석 ===");
    
    // 상금 구조별 비교 시나리오
    let stacks = vec![6000, 4000, 3000, 2000];
    
    println!("동일한 스택, 다른 상금 구조:\n");
    
    // 균등 상금 구조
    let flat_payouts = vec![2500.0, 2500.0, 2500.0, 2500.0];
    analyze_payout_structure("균등 구조", &stacks, &flat_payouts);
    
    // 승자 독식
    let winner_takes_all = vec![10000.0];
    analyze_payout_structure("승자 독식", &stacks, &winner_takes_all);
    
    // 표준 토너먼트 구조
    let standard_payouts = vec![5000.0, 3000.0, 2000.0];
    analyze_payout_structure("표준 (3명 상금)", &stacks, &standard_payouts);
    
    // 상위 집중 구조
    let top_heavy = vec![7000.0, 2000.0, 1000.0];
    analyze_payout_structure("상위 집중", &stacks, &top_heavy);
}

fn analyze_payout_structure(name: &str, stacks: &[u32], payouts: &[f64]) {
    let icm_calculator = ICMCalculator::new(stacks.to_vec(), payouts.iter().map(|&x| x as u64).collect());
    let icm_values = icm_calculator.calculate_equity();
    
    println!("{}: ", name);
    let total_chips: u32 = stacks.iter().sum();
    let total_payouts: f64 = payouts.iter().sum();
    
    for (i, (&stack, &icm_value)) in stacks.iter().zip(&icm_values).enumerate() {
        let chip_ev = (stack as f64 / total_chips as f64) * total_payouts;
        let icm_premium = icm_value - chip_ev;
        println!("  플레이어 {}: ICM 프리미엄: ${:.2}", i + 1, icm_premium);
    }
    println!();
}

fn analyze_chip_ev_vs_dollar_ev() {
    println!("=== 칩 EV vs 달러 EV 분석 ===");
    
    let base_stacks = vec![5000, 4000, 3000, 2000, 1000];
    let payouts = vec![7000.0, 4000.0, 2500.0, 1500.0, 1000.0];
    
    println!("다양한 올인 시나리오 분석:\n");
    
    // 시나리오 1: 숏 스택이 더블업
    println!("시나리오 1: 숏 스택 (1000)이 미디엄 스택 (3000)을 통해 더블업");
    let scenario1_win = vec![5000, 4000, 1000, 2000, 2000];
    let scenario1_lose = vec![5000, 4000, 4000, 2000, 0];
    
    analyze_scenario_comparison("숏 스택 승리", &base_stacks, &scenario1_win, &payouts, 4);
    analyze_scenario_comparison("숏 스택 패배", &base_stacks, &scenario1_lose, &payouts, 4);
    
    println!();
    
    // 시나리오 2: 빅 스택 vs 미디엄 스택
    println!("시나리오 2: 빅 스택 (5000) vs 미디엄 스택 (4000)");
    let scenario2_big_wins = vec![9000, 0, 3000, 2000, 1000];
    let scenario2_medium_wins = vec![1000, 8000, 3000, 2000, 1000];
    
        analyze_scenario_comparison("빅 스택 승리", &base_stacks, &scenario2_big_wins, &payouts, 0);
    analyze_scenario_comparison("미디엄 스택 승리", &base_stacks, &scenario2_medium_wins, &payouts, 1);
}

fn analyze_scenario_comparison(scenario_name: &str, before: &[u32], after: &[u32], payouts: &[f64], acting_player: usize) {
    let icm_calculator_before = ICMCalculator::new(before.to_vec(), payouts.iter().map(|&x| x as u64).collect());
    let before_icm = icm_calculator_before.calculate_equity();
    
    let icm_calculator_after = ICMCalculator::new(after.to_vec(), payouts.iter().map(|&x| x as u64).collect());
    let after_icm = icm_calculator_after.calculate_equity();
    
    let ev_change = after_icm[acting_player] - before_icm[acting_player];
    
    println!("  {}: 플레이어 {}의 EV 변화: ${:.2}", 
             scenario_name, acting_player + 1, ev_change);
}

fn analyze_stack_size_impact() {
    println!("=== 스택 크기가 ICM 압박에 미치는 영향 ===");
    
    let payouts = vec![5000.0, 3000.0, 2000.0];
    
    println!("ICM 압박이 스택 크기에 따라 어떻게 변하는지 (3명 플레이어, 버블 상황):\n");
    
    // 다양한 스택 분포 테스트
    let test_scenarios = vec![
        ("숏 스택", vec![1000, 4000, 5000]),
        ("미디엄 스택", vec![3000, 3000, 4000]),
        ("빅 스택", vec![5000, 3000, 2000]),
        ("칩 리더", vec![7000, 2000, 1000]),
    ];
    
    for (scenario_name, stacks) in test_scenarios {
        let icm_calculator = ICMCalculator::new(stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
        let icm_values = icm_calculator.calculate_equity();
        
        let total_chips: u32 = stacks.iter().sum();
        let total_payouts: f64 = payouts.iter().sum();
        
        println!("{} 시나리오:", scenario_name);
        for (i, (&stack, &icm_value)) in stacks.iter().zip(&icm_values).enumerate() {
            let chip_ev = (stack as f64 / total_chips as f64) * total_payouts;
            let icm_pressure = ((icm_value - chip_ev) / chip_ev) * 100.0;
            let risk_tolerance = calculate_risk_tolerance(stack, &stacks, icm_pressure);
            
            println!("  플레이어 {} ({} 칩): ICM 압박: {:.1}%, 위험 허용도: {}", 
                     i + 1, stack, icm_pressure, risk_tolerance);
        }
        println!();
    }
}

fn analyze_all_in_scenarios(stacks: &[u32], payouts: &[f64]) {
    println!("올인 시나리오 분석:");
    
    let icm_calculator = ICMCalculator::new(stacks.to_vec(), payouts.iter().map(|&x| x as u64).collect());
    let current_icm = icm_calculator.calculate_equity();
    
    // 숏 스택 (플레이어 4)이 올인
    let short_stack_pos = 3; // 0-indexed, 플레이어 4
    let short_stack = stacks[short_stack_pos];
    
    println!("숏 스택 (플레이어 4, {} 칩)이 올인하는 경우:", short_stack);
    
    // 다른 각 플레이어의 콜 수익성 분석
    for i in 0..stacks.len() {
        if i == short_stack_pos { continue; }
        
        let caller_stack = stacks[i];
        if caller_stack <= short_stack { continue; } // 올인을 커버할 수 없음
        
        // 시나리오 계산: 콜 후 승리 vs 콜 후 패배
        let mut win_stacks = stacks.to_vec();
        let mut lose_stacks = stacks.to_vec();
        
        win_stacks[i] += short_stack;
        win_stacks[short_stack_pos] = 0;
        
        lose_stacks[i] -= short_stack;
        lose_stacks[short_stack_pos] = short_stack * 2;
        
        let win_calculator = ICMCalculator::new(win_stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
        let win_icm = win_calculator.calculate_equity();
        
        let lose_calculator = ICMCalculator::new(lose_stacks.clone(), payouts.iter().map(|&x| x as u64).collect());
        let lose_icm = lose_calculator.calculate_equity();
        
        // 최소 승률 계산 필요
        let current_value = current_icm[i];
        let win_value = win_icm[i];
        let lose_value = lose_icm[i];
        
        let breakeven_percentage = (current_value - lose_value) / (win_value - lose_value);
        
        println!("  플레이어 {} 콜 분석:", i + 1);
        println!("    현재 ICM: ${:.2}", current_value);
        println!("    승리 시: ${:.2} (이득: ${:.2})", win_value, win_value - current_value);
        println!("    패배 시: ${:.2} (손실: ${:.2})", lose_value, current_value - lose_value);
        println!("    손익분기점: {:.1}%", breakeven_percentage * 100.0);
        println!("    권장사항: {}", 
                 if breakeven_percentage > 0.6 { "타이트한 콜 - 강한 핸드 필요" }
                 else if breakeven_percentage > 0.4 { "표준 콜 범위" }
                 else { "넓은 콜 범위 허용" });
        println!();
    }
}

fn calculate_bubble_factor(stack: u32, all_stacks: &[u32], payouts: &[f64]) -> f64 {
    let players_remaining = all_stacks.len();
    let paid_positions = payouts.len();
    
    if players_remaining <= paid_positions {
        return 1.0; // 상금권 진입
    }
    
    let excess_players = players_remaining - paid_positions;
    let stack_rank = all_stacks.iter()
        .filter(|&&s| s > stack)
        .count() + 1;
    
    // 버블 거리와 상대적 스택 크기를 모두 고려하는 팩터
    let base_factor = 1.0 - (excess_players as f64 / 6.0).min(0.9);
    let stack_adjustment = (stack_rank as f64 / players_remaining as f64).min(1.0);
    
    (base_factor * stack_adjustment).max(0.1)
}

fn get_bubble_strategy_advice(stack: u32, all_stacks: &[u32], bubble_factor: f64) -> &'static str {
    let total_chips: u32 = all_stacks.iter().sum();
    let stack_percentage = (stack as f64 / total_chips as f64) * 100.0;
    
    match (stack_percentage, bubble_factor) {
        (p, _f) if p > 40.0 => "공격적 - 작은 스택들에게 압박",
        (p, _f) if p > 30.0 => "선택적 공격 - 약점 타겟",
        (p, _f) if p > 20.0 => "신중함 - 큰 대결 피하기",
        (p, _f) if p > 10.0 => "생존 모드 - 매우 타이트, 프리미엄 핸드 대기",
        _ => "절망적 - 더블업 기회 찾아야 함"
    }
}

fn get_final_table_strategy(position: usize, _stack: u32, _all_stacks: &[u32], icm_pressure: f64) -> &'static str {
    match (position, icm_pressure) {
        (0..=1, p) if p > 10.0 => "보수적 칩 리더 - 리드 보호",
        (0..=1, _) => "공격적 칩 리더 - 압도적 리드 구축", 
        (2..=3, p) if p > 5.0 => "균형 잡힌 - 공격성과 신중함 혼합",
        (2..=3, _) => "표준 플레이 - 계산된 위험 감수",
        (4..=5, p) if p < -5.0 => "공격적 숏 스택 - 칩 축적 필요",
        _ => "타이트 - 다음 상금 점프까지 생존"
    }
}

fn calculate_risk_tolerance(_stack: u32, _all_stacks: &[u32], icm_pressure: f64) -> &'static str {
    match icm_pressure {
        p if p > 10.0 => "낮음 - 스택 우위 보호",
        p if p > 5.0 => "중간-낮음 - 선택적 기회만",
        p if p > -5.0 => "중간 - 표준 위험 평가",
        p if p > -10.0 => "높음 - 칩 축적 필요",
        _ => "매우 높음 - 절망적 상황에서는 큰 위험 필요"
    }
}
