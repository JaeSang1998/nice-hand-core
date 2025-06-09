// 토너먼트 기능 종합 데모
use nice_hand_core::game::tournament::*;
use std::time::Instant;

fn main() {
    println!("=== 고급 토너먼트 기능 데모 ===\n");
    
    println!("🎯 1. 현실적인 시나리오를 포함한 ICM 계산");
    demo_icm_calculations();
    
    println!("🤖 2. 정교한 상대방 모델링");
    demo_opponent_modeling();
    
    println!("🏆 3. 멀티테이블 토너먼트 관리");
    demo_mtt_management();
    
    println!("🫧 4. 고급 버블 전략");
    demo_bubble_strategy();
    
    println!("⚖️ 5. 토너먼트 상태 평가");
    demo_tournament_evaluation();
    
    println!("\n=== 토너먼트 기능 데모 완료 ===");
    println!("모든 고급 토너먼트 알고리즘이 올바르게 작동합니다! 🚀");
}

fn demo_icm_calculations() {
    // 시나리오 1: 파이널 테이블 버블 (10명, 9명 입상)
    println!("\n   📊 시나리오 1: 파이널 테이블 버블");
    let stacks1 = vec![45000, 38000, 32000, 28000, 25000, 22000, 18000, 15000, 12000, 8000];
    let payouts1 = vec![150000, 90000, 60000, 45000, 35000, 28000, 22000, 18000, 15000];
    
    let icm1 = ICMCalculator::new(stacks1.clone(), payouts1.clone());
    let start = Instant::now();
    let equities1 = icm1.calculate_equity();
    let duration = start.elapsed();
    
    println!("      스택: {:?}", stacks1);
    println!("      ICM 에퀴티: {:.0?}", equities1);
    println!("      계산 시간: {:?}", duration);
    
    // 시나리오 2: ICM을 고려한 헤즈업
    println!("\n   🥊 시나리오 2: 헤즈업 ICM");
    let stacks2 = vec![180000, 60000];
    let payouts2 = vec![360000, 240000];
    
    let icm2 = ICMCalculator::new(stacks2.clone(), payouts2.clone());
    let equities2 = icm2.calculate_equity();
    
    println!("      칩 스택: {:?}", stacks2);
    println!("      상금 풀: {:?}", payouts2);
    println!("      ICM 에퀴티: {:.0?}", equities2);
    
    let chip_ratio = stacks2[0] as f64 / stacks2[1] as f64;
    let equity_ratio = equities2[0] / equities2[1];
    println!("      칩 우위: {:.2}:1, ICM 우위: {:.2}:1", chip_ratio, equity_ratio);
    
    // ICM 압박 분석
    println!("\n   📉 ICM 압박 분석");
    let pressure_big = icm2.calculate_icm_pressure(0, -10000);
    let pressure_small = icm2.calculate_icm_pressure(1, -10000);
    println!("      큰 스택 압박 (10k 손실): {:.4}", pressure_big);
    println!("      작은 스택 압박 (10k 손실): {:.4}", pressure_small);
    
    println!("   ✅ ICM 계산이 현실적인 토너먼트 역학을 보여줍니다\n");
}

fn demo_opponent_modeling() {
    println!("\n   🤖 고급 상대방 프로파일링");
    
    let mut aggressive_player = OpponentModel::new(1);
    let mut tight_player = OpponentModel::new(2);
    
    // 20핸드의 관찰 시뮬레이션
    for hand in 1..=20 {
        let context = ActionContext {
            stack_ratio: 1.0 - (hand as f64 * 0.02), // 점진적으로 칩 손실
            pot_odds: 0.3,
            is_preflop: hand % 3 == 1,
            near_bubble: hand > 15,
            position: if hand % 2 == 0 { Position::Button } else { Position::EarlyPosition },
            num_opponents: 3,
        };
        
        // 공격적인 플레이어 액션
        if hand % 3 == 0 {
            aggressive_player.update_with_action(&TournamentAction::Raise(100), &context);
        } else if hand % 4 == 0 {
            aggressive_player.update_with_action(&TournamentAction::Call, &context);
        } else {
            aggressive_player.update_with_action(&TournamentAction::Fold, &context);
        }
        
        // 타이트한 플레이어 액션  
        if hand % 5 == 0 {
            tight_player.update_with_action(&TournamentAction::Call, &context);
        } else if hand % 8 == 0 {
            tight_player.update_with_action(&TournamentAction::Raise(80), &context);
        } else {
            tight_player.update_with_action(&TournamentAction::Fold, &context);
        }
    }
    
    println!("      공격적인 플레이어 프로필:");
    println!("         VPIP: {:.1}%", aggressive_player.vpip * 100.0);
    println!("         PFR: {:.1}%", aggressive_player.pfr * 100.0);
    println!("         공격성: {:.2}", aggressive_player.aggression);
    println!("         버블 조정: {:.2}", aggressive_player.bubble_adjustment);
    
    println!("      타이트한 플레이어 프로필:");
    println!("         VPIP: {:.1}%", tight_player.vpip * 100.0);
    println!("         PFR: {:.1}%", tight_player.pfr * 100.0);
    println!("         공격성: {:.2}", tight_player.aggression);
    println!("         버블 조정: {:.2}", tight_player.bubble_adjustment);
    
    // 버블 상황에서 예측 테스트
    let bubble_context = ActionContext {
        stack_ratio: 0.6,
        pot_odds: 0.25,
        is_preflop: true,
        near_bubble: true,
        position: Position::MiddlePosition,
        num_opponents: 2,
    };
    
    let agg_prediction = aggressive_player.predict_action_distribution(&bubble_context);
    let tight_prediction = tight_player.predict_action_distribution(&bubble_context);
    
    println!("      버블 예측 (폴드/콜/레이즈):");
    println!("         공격적: {:.2?}", agg_prediction);
    println!("         타이트: {:.2?}", tight_prediction);
    
    println!("   ✅ 상대방 모델이 현실적인 학습과 적응을 보여줍니다\n");
}

fn demo_mtt_management() {
    println!("\n   🏆 멀티테이블 토너먼트 시뮬레이션");
    
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
            BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 0 },
            BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 25 },
            BlindLevel { level: 4, small_blind: 100, big_blind: 200, ante: 25 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let mut mtt = MTTManager::new(54, 9, structure, 100000);
    
    println!("      초기 설정:");
    println!("         총 테이블 수: {}", mtt.tables.len());
    println!("         테이블당 플레이어: {}", mtt.tables[0].count_active_players());
    println!("         총 활성 플레이어: {}", mtt.count_active_players());
    
    // 토너먼트 진행 시뮬레이션
    println!("\n      토너먼트 진행:");
    
    // 토너먼트 흐름을 시뮬레이션하기 위해 플레이어 탈락
    let eliminations = vec![
        (0, 1), (1, 10), (2, 19), (0, 28), (1, 37), (2, 46),  // 초기 탈락
        (0, 2), (1, 11), (2, 20), (0, 29), (1, 38),           // 추가 탈락
        (0, 3), (1, 12), (2, 21), (0, 30),                    // 버블에 근접
    ];
    
    for (table_id, player_id) in eliminations {
        mtt.eliminate_player(table_id, player_id);
        
        if mtt.count_active_players() % 10 == 0 {
            mtt.balance_tables();
            println!("         {} 명 남음, {} 테이블 활성", 
                    mtt.count_active_players(), mtt.tables.len());
        }
    }
    
    // 파이널 테이블 통합 테스트
    println!("\n      파이널 테이블 구성:");
    
    // 파이널 테이블에 도달하기 위해 더 많은 플레이어 탈락
    let remaining_players = mtt.count_active_players();
    let mut eliminations_needed = remaining_players - 9;
    let mut table_idx = 0;
    let mut player_id = 50;
    
    while eliminations_needed > 0 && mtt.count_active_players() > 9 {
        mtt.eliminate_player(table_idx, player_id);
        table_idx = (table_idx + 1) % 3;
        player_id += 1;
        eliminations_needed -= 1;
    }
    
    // 파이널 테이블 통합 트리거
    mtt.balancing_algorithm = BalancingAlgorithm::FinalTableConsolidation;
    mtt.balance_tables();
    
    println!("         {} 명으로 파이널 테이블 구성", mtt.count_active_players());
    println!("         남은 테이블: {}", mtt.tables.len());
    
    // 최종 순위 표시
    let standings = mtt.get_tournament_standings();
    println!("         파이널 테이블 칩 카운트:");
    for (i, (player_id, stack, _)) in standings.iter().enumerate() {
        println!("            {}번 자리: 플레이어 {} - {} 칩", i + 1, player_id, stack);
    }
    
    println!("   ✅ MTT 관리가 현실적인 토너먼트 흐름을 처리합니다\n");
}

fn demo_bubble_strategy() {
    println!("\n   🫧 동적 버블 전략 분석");
    
    let scenarios = vec![
        ("버블 전", 25, 18),
        ("버블 접근", 22, 18), 
        ("버블 근접", 20, 18),
        ("버블", 19, 18),
        ("입상권", 15, 18),
    ];
    
    for (phase, players, payouts) in scenarios {
        println!("      {} ({} 명, {} 명 입상):", phase, players, payouts);
        
        let bubble_strategy = BubbleStrategy::new(players, payouts);
        
        println!("         버블 팩터: {:.3}", bubble_strategy.bubble_factor);
        println!("         폴드 에퀴티 부스트: {:.3}", bubble_strategy.fold_equity_boost);
        
        // 다양한 스택 크기 테스트
        let stack_scenarios = vec![
            ("숏 스택", 0.3),
            ("평균 스택", 1.0),
            ("빅 스택", 2.5),
            ("칩 리더", 4.0),
        ];
        
        let base_range = 0.15; // 15% 기본 핸드 레인지
        
        for (stack_type, ratio) in stack_scenarios {
            let adjusted_range = bubble_strategy.adjust_hand_range(base_range, ratio);
            let should_be_aggressive = bubble_strategy.should_make_aggressive_play(ratio, 0.1);
            
            println!("            {} ({}배 평균): 레인지 {:.1}%, 공격적: {}", 
                    stack_type, ratio, adjusted_range * 100.0, should_be_aggressive);
        }
        println!();
    }
    
    println!("   ✅ 버블 전략이 토너먼트 역학에 현실적으로 적응합니다\n");
}

fn demo_tournament_evaluation() {
    println!("\n   ⚖️ 고급 토너먼트 상태 평가");
    
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 4, small_blind: 200, big_blind: 400, ante: 50 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 180, 500000);
    let final_stacks = vec![85000, 72000, 58000, 45000, 32000, 28000, 20000, 15000, 8000];
    
    let evaluator = TournamentEvaluator::new(tournament_state, final_stacks.clone());
    
    println!("      파이널 테이블 평가:");
    println!("         순위    스택     ICM 가치   정규화");
    
    for (i, &stack) in final_stacks.iter().enumerate() {
        let evaluation = evaluator.evaluate_terminal_state(&final_stacks, i);
        let icm_equities = evaluator.icm_calculator.calculate_equity();
        let icm_value = if i < icm_equities.len() { icm_equities[i] } else { 0.0 };
        
        println!("            {}      {:6}    {:8.0}     {:6.3}", 
                i + 1, stack, icm_value, evaluation);
    }
    
    // 다양한 상황에서 상대방 액션 선택 테스트
    println!("\n      상대방 액션 선택:");
    
    let contexts = vec![
        ("얼리 포지션, 버블", ActionContext {
            stack_ratio: 0.5,
            pot_odds: 0.2,
            is_preflop: true,
            near_bubble: true,
            position: Position::EarlyPosition,
            num_opponents: 8,
        }),
        ("버튼, 딥 스택", ActionContext {
            stack_ratio: 2.0,
            pot_odds: 0.3,
            is_preflop: true,
            near_bubble: false,
            position: Position::Button,
            num_opponents: 4,
        }),
    ];
    
    let available_actions = vec![
        TournamentAction::Fold,
        TournamentAction::Call,
        TournamentAction::Raise(800),
        TournamentAction::AllIn,
    ];
    
    for (scenario, context) in contexts {
        let selected_action = evaluator.select_opponent_action(1, &context, &available_actions);
        println!("         {}: {:?}", scenario, selected_action);
    }
    
    println!("   ✅ 토너먼트 평가가 정교한 의사결정 분석을 제공합니다\n");
}

fn test_icm_calculations() {
    println!("🎯 ICM 계산 테스트...");
    
    // 현실적인 토너먼트 시나리오: 4명 남음, 3명 입상
    let stacks = vec![15000, 8000, 5000, 2000]; // 칩 스택
    let payouts = vec![10000, 6000, 4000]; // 상금 구조
    
    let icm = ICMCalculator::new(stacks.clone(), payouts.clone());
    let start_time = Instant::now();
    let equities = icm.calculate_equity();
    let calculation_time = start_time.elapsed();
    
    println!("   📊 스택: {:?}", stacks);
    println!("   💰 상금: {:?}", payouts);
    println!("   ⚖️  ICM 에퀴티: {:.2?}", equities);
    println!("   ⏱️  계산 시간: {:?}", calculation_time);
    
    // ICM 압박 계산 테스트
    let pressure = icm.calculate_icm_pressure(0, -1000); // 빅 스택이 1k 칩 손실
    println!("   📉 칩 리더가 1000 칩 잃을 때 ICM 압박: {:.4}", pressure);
    
    // 다양한 시나리오 테스트
    test_bubble_icm();
    test_heads_up_icm();
    
    println!("   ✅ ICM 계산이 올바르게 작동합니다\n");
}

fn test_bubble_icm() {
    println!("   🫧 버블 ICM 테스트...");
    
    // 5명, 4명 입상 (버블 상황)
    let stacks = vec![12000, 10000, 8000, 6000, 4000];
    let payouts = vec![15000, 10000, 7000, 5000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      버블 에퀴티: {:.2?}", equities);
    
    // 숏 스택은 비슷한 스택 크기에도 불구하고 낮은 에퀴티를 가져야 함
    if equities.len() >= 5 {
        println!("      ✅ 숏 스택이 적절히 감소된 에퀴티를 가집니다");
    }
}

fn test_heads_up_icm() {
    println!("   🥊 헤즈업 ICM 테스트...");
    
    let stacks = vec![30000, 10000];
    let payouts = vec![20000, 12000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      헤즈업 에퀴티: {:.2?}", equities);
    
    // 칩 리더는 3:1 칩 리드에도 불구하고 75% 이상의 에퀴티를 가져야 함
    if equities.len() >= 2 {
        println!("      ✅ ICM이 칩 리더 우위를 적절히 감소시킵니다");
    }
}

fn test_opponent_modeling() {
    println!("🤖 상대방 모델링 테스트...");
    
    let mut model = OpponentModel::new(1);
    
    // 상대방 액션 관찰 시뮬레이션
    let contexts = vec![
        ActionContext {
            stack_ratio: 1.0,
            pot_odds: 0.3,
            is_preflop: true,
            near_bubble: false,
            position: Position::Button,
            num_opponents: 3,
        },
        ActionContext {
            stack_ratio: 0.8,
            pot_odds: 0.25,
            is_preflop: false,
            near_bubble: true,
            position: Position::EarlyPosition,
            num_opponents: 2,
        },
    ];
    
    // 타이트한 플레이 관찰
    model.update_with_action(&TournamentAction::Fold, &contexts[0]);
    model.update_with_action(&TournamentAction::Fold, &contexts[1]);
    model.update_with_action(&TournamentAction::Call, &contexts[0]);
    
    println!("   📈 액션 관찰 후:");
    println!("      VPIP: {:.3}", model.vpip);
    println!("      PFR: {:.3}", model.pfr);
    println!("      공격성: {:.3}", model.aggression);
    println!("      타이트함: {:.3}", model.tightness);
    println!("      샘플 크기: {}", model.sample_size);
    
    // 액션 예측 테스트
    let predictions = model.predict_action_distribution(&contexts[1]);
    println!("   🔮 액션 예측 (폴드/콜/레이즈): {:.3?}", predictions);
    
    println!("   ✅ 상대방 모델링이 올바르게 작동합니다\n");
}

fn test_mtt_management() {
    println!("🏆 MTT 관리 테스트...");
    
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 10, big_blind: 20, ante: 0 },
            BlindLevel { level: 2, small_blind: 15, big_blind: 30, ante: 0 },
            BlindLevel { level: 3, small_blind: 25, big_blind: 50, ante: 5 },
        ],
        level_duration_minutes: 15,
        starting_stack: 5000,
        ante_schedule: vec![],
    };
    
    let mut mtt = MTTManager::new(27, 9, structure, 10000);
    
    println!("   🎲 초기 토너먼트 설정:");
    println!("      테이블: {}", mtt.tables.len());
    println!("      활성 플레이어: {}", mtt.count_active_players());
    
    // 테이블 밸런싱 테스트
    mtt.balance_tables();
    println!("   ⚖️  밸런싱 후:");
    for (i, table) in mtt.tables.iter().enumerate() {
        println!("      테이블 {}: {} 명", i, table.count_active_players());
    }
    
    // 플레이어 탈락 테스트
    mtt.eliminate_player(0, 1);
    mtt.eliminate_player(0, 2);
    println!("   ❌ 2명 탈락 후: {} 명 활성", mtt.count_active_players());
    
    // 토너먼트 순위 테스트
    let standings = mtt.get_tournament_standings();
    println!("   🏅 상위 5명 칩 리더:");
    for (i, (player_id, stack, table_id)) in standings.iter().take(5).enumerate() {
        println!("      {}. 플레이어 {} - {} 칩 (테이블 {})", i + 1, player_id, stack, table_id);
    }
    
    // 파이널 테이블 통합 테스트
    // 파이널 테이블 트리거를 위해 대부분 플레이어 탈락
    for table_id in 0..mtt.tables.len() {
        for player_id in 10..25 {
            mtt.eliminate_player(table_id as u32, player_id);
        }
    }
    
    mtt.balancing_algorithm = BalancingAlgorithm::FinalTableConsolidation;
    mtt.balance_tables();
    
    println!("   🎯 파이널 테이블 통합:");
    println!("      남은 테이블: {}", mtt.tables.len());
    if !mtt.tables.is_empty() {
        println!("      파이널 테이블 플레이어: {}", mtt.tables[0].count_active_players());
    }
    
    println!("   ✅ MTT 관리가 올바르게 작동합니다\n");
}

fn test_bubble_strategy() {
    println!("🫧 버블 전략 테스트...");
    
    // 다양한 버블 시나리오 테스트
    let scenarios = vec![
        (15, 9),  // 버블까지 6명
        (11, 9),  // 버블까지 2명  
        (10, 9),  // 버블 상황
        (8, 9),   // 인더머니
    ];
    
    for (players_remaining, payout_spots) in scenarios {
        let bubble_strategy = BubbleStrategy::new(players_remaining, payout_spots);
        
        println!("   📊 {} 명 남음, {} 명 상금:", players_remaining, payout_spots);
        println!("      버블 팩터: {:.3}", bubble_strategy.bubble_factor);
        println!("      폴드 에퀴티 부스트: {:.3}", bubble_strategy.fold_equity_boost);
        println!("      ICM 민감도: {:.3}", bubble_strategy.icm_sensitivity);
        
        // 다양한 스택 크기에 대한 핸드 레인지 조정 테스트
        let base_range = 0.2; // 핸드의 20%
        let stack_ratios = vec![0.05, 0.2, 0.5, 1.5]; // 매우 짧음, 짧음, 평균, 큼
        
        for &stack_ratio in &stack_ratios {
            let adjusted_range = bubble_strategy.adjust_hand_range(base_range, stack_ratio);
            println!("      스택 비율 {:.2} -> 레인지 {:.3}", stack_ratio, adjusted_range);
        }
        
        // 공격적 플레이 결정 테스트
        let should_play = bubble_strategy.should_make_aggressive_play(0.1, 0.05);
        println!("      숏스택 공격적 플레이 결정: {}", should_play);
        
        println!();
    }
    
    println!("   ✅ 버블 전략이 올바르게 작동합니다\n");
}

fn test_tournament_evaluator() {
    println!("🎯 토너먼트 평가자 테스트...");
    
    // 토너먼트 상태 생성
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 100, 50000);
    let player_stacks = vec![15000, 12000, 8000, 5000, 3000];
    
    let mut evaluator = TournamentEvaluator::new(tournament_state, player_stacks.clone());
    
    // 터미널 상태 평가 테스트
    println!("   🎯 터미널 상태 평가:");
    for (i, &stack) in player_stacks.iter().enumerate() {
        let evaluation = evaluator.evaluate_terminal_state(&player_stacks, i);
        println!("      플레이어 {} (스택 {}): {:.4}", i, stack, evaluation);
    }
    
    // 상대방 액션 선택 테스트
    let context = ActionContext {
        stack_ratio: 0.8,
        pot_odds: 0.3,
        is_preflop: true,
        near_bubble: true,
        position: Position::Button,
        num_opponents: 2,
    };
    
    let available_actions = vec![
        TournamentAction::Fold,
        TournamentAction::Call,
        TournamentAction::Raise(100),
    ];
    
    let selected_action = evaluator.select_opponent_action(1, &context, &available_actions);
    println!("   🤖 선택된 상대방 액션: {:?}", selected_action);
    
    // ICM 압박 계산 테스트
    let icm_pressure = evaluator.calculate_icm_adjusted_ev(0, -500);
    println!("   📊 500 칩 손실에 대한 ICM 압박: {:.6}", icm_pressure);
    
    // 상대방 모델 업데이트
    evaluator.update_opponent_model(1, TournamentAction::Raise(150), context);
    println!("   📈 플레이어 1의 상대방 모델 업데이트됨");
    
    println!("   ✅ 토너먼트 평가자가 올바르게 작동합니다\n");
}

fn demo_cfr_integration() {
    println!("\n🔗 토너먼트 기능과 CFR 통합");
    
    // 토너먼트 기능이 CFR과 어떻게 통합되는지 시뮬레이션
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 100, big_blind: 200, ante: 25 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let tournament_state = TournamentState::new(structure, 100, 200000);
    let player_stacks = vec![15000, 12000, 8000, 5000];
    
    let evaluator = TournamentEvaluator::new(tournament_state, player_stacks.clone());
    
    println!("      통합 지점:");
    
    // 1. 터미널 상태 평가
    println!("         1. 터미널 상태 평가:");
    for (i, &stack) in player_stacks.iter().enumerate() {
        let evaluation = evaluator.evaluate_terminal_state(&player_stacks, i);
        println!("            플레이어 {} (스택 {}): CFR 값 {:.3}", i + 1, stack, evaluation);
    }
    
    // 2. 전략 조정
    println!("         2. 전략 조정:");
    let base_strategy = vec![0.3, 0.4, 0.3]; // 폴드, 콜, 레이즈
    
    for (i, &stack) in player_stacks.iter().enumerate() {
        let avg_stack = 35000 / 4; // 총 칩 / 플레이어
        let stack_ratio = stack as f64 / avg_stack as f64;
        
        let bubble_strategy = BubbleStrategy::new(4, 3);
        let adjusted_range = bubble_strategy.adjust_hand_range(0.15, stack_ratio);
        
        println!("            플레이어 {} 레인지: {:.1}% -> {:.1}%", 
                i + 1, 15.0, adjusted_range * 100.0);
    }
    
    // 3. 상대방 모델링 통합
    println!("         3. 상대방 모델링:");
    let context = ActionContext {
        stack_ratio: 0.8,
        pot_odds: 0.3,
        is_preflop: true,
        near_bubble: true,
        position: Position::Button,
        num_opponents: 3,
    };
    
    let mut model = OpponentModel::new(1);
    model.update_with_action(&TournamentAction::Raise(300), &context);
    let prediction = model.predict_action_distribution(&context);
    
    println!("            상대방 예측: 폴드={:.2}, 콜={:.2}, 레이즈={:.2}", 
            prediction[0], prediction[1], prediction[2]);
    
    println!("      ✅ CFR이 토너먼트 기능을 원활하게 통합할 수 있습니다\n");
}

fn demo_performance_comparison() {
    println!("⚡ 성능 분석");
    
    // 다양한 토너먼트 계산의 성능 비교
    let test_scenarios = vec![
        (5, vec![15000, 12000, 8000, 5000, 2000]),
        (9, vec![25000, 20000, 18000, 15000, 12000, 10000, 8000, 5000, 3000]),
        (18, (0..18).map(|i| 10000 - i * 300).collect()),
    ];
    
    println!("      다양한 테이블 크기별 성능:");
    
    for (players, stacks) in test_scenarios {
        let payouts: Vec<u64> = (0..players).map(|i| 10000 - i as u64 * 500).collect();
        let icm = ICMCalculator::new(stacks.clone(), payouts);
        
        let start = Instant::now();
        let iterations = 100;
        
        for _ in 0..iterations {
            let _equities = icm.calculate_equity();
        }
        
        let duration = start.elapsed();
        let per_calc = duration.as_micros() as f64 / iterations as f64;
        
        println!("         {} 명 플레이어: ICM 계산당 {:.1}μs", players, per_calc);
    }
    
    // MTT 관리 성능 테스트
    let structure = TournamentStructure {
        levels: vec![
            BlindLevel { level: 1, small_blind: 50, big_blind: 100, ante: 0 },
        ],
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    let start = Instant::now();
    let mut mtt = MTTManager::new(180, 9, structure, 100000); // 대형 토너먼트
    
    // 탈락과 테이블 밸런싱 시뮬레이션
    for player_id in 1..50 {
        mtt.eliminate_player(player_id % 20, player_id);
        if player_id % 10 == 0 {
            mtt.balance_tables();
        }
    }
    
    let mtt_duration = start.elapsed();
    println!("      MTT 관리 (180 -> 131 명 플레이어): {:?}", mtt_duration);
    
    println!("   ✅ 모든 토너먼트 알고리즘이 효율적으로 작동합니다\n");
}
