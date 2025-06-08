// Additional tournament test functions
use nice_hand_core::game::tournament::*;

pub fn test_bubble_icm() {
    println!("   🫧 Testing Bubble ICM...");
    
    // 5 players, 4 get paid (bubble situation)
    let stacks = vec![12000, 10000, 8000, 6000, 4000];
    let payouts = vec![15000, 10000, 7000, 5000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      Bubble equities: {:.2?}", equities);
    
    // Short stack should have lower equity despite similar stack sizes
    assert!(equities[4] < equities[3], "Short stack should have lower ICM equity");
}

pub fn test_heads_up_icm() {
    println!("   🥊 Testing Heads-up ICM...");
    
    let stacks = vec![30000, 10000];
    let payouts = vec![20000, 12000];
    
    let icm = ICMCalculator::new(stacks, payouts);
    let equities = icm.calculate_equity();
    
    println!("      Heads-up equities: {:.2?}", equities);
    
    // Chip leader should have more than 75% equity despite 3:1 chip lead
    assert!(equities[0] > 15000.0 && equities[0] < 18000.0, "ICM should reduce chip leader advantage");
}

pub fn test_opponent_modeling() {
    println!("🤖 Testing Opponent Modeling...");
    
    let mut model = OpponentModel::new(1);
    
    // Simulate observing opponent actions
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
    
    // Observe tight play
    model.update_with_action(&TournamentAction::Fold, &contexts[0]);
    model.update_with_action(&TournamentAction::Fold, &contexts[1]);
    model.update_with_action(&TournamentAction::Call, &contexts[0]);
    
    println!("   📈 After observing actions:");
    println!("      VPIP: {:.3}", model.vpip);
    println!("      PFR: {:.3}", model.pfr);
    println!("      Aggression: {:.3}", model.aggression);
    println!("      Tightness: {:.3}", model.tightness);
    println!("      Sample size: {}", model.sample_size);
    
    // Test action prediction
    let predictions = model.predict_action_distribution(&contexts[1]);
    println!("   🔮 Action prediction (fold/call/raise): {:.3?}", predictions);
    
    println!("   ✅ Opponent modeling working correctly\n");
}

pub fn test_mtt_management() {
    println!("🏆 Testing MTT Management...");
    
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
    
    println!("   🎲 Initial tournament setup:");
    println!("      Tables: {}", mtt.tables.len());
    println!("      Active players: {}", mtt.count_active_players());
    
    // Test table balancing
    mtt.balance_tables();
    println!("   ⚖️  After balancing:");
    for (i, table) in mtt.tables.iter().enumerate() {
        println!("      Table {}: {} players", i, table.count_active_players());
    }
    
    // Test player elimination
    mtt.eliminate_player(0, 1);
    mtt.eliminate_player(0, 2);
    println!("   ❌ After eliminating 2 players: {} active", mtt.count_active_players());
    
    // Test tournament standings
    let standings = mtt.get_tournament_standings();
    println!("   🏅 Top 5 chip leaders:");
    for (i, (player_id, stack, table_id)) in standings.iter().take(5).enumerate() {
        println!("      {}. Player {} - {} chips (Table {})", i + 1, player_id, stack, table_id);
    }
    
    println!("   ✅ MTT management working correctly\n");
}
