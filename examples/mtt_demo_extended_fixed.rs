use nice_hand_core::game::tournament::*;

/// Extended Multi-Table Tournament (MTT) Demo
/// 
/// This example demonstrates:
/// - Multi-table balancing algorithms
/// - Player movement and consolidation
/// - Table breaking logic
/// - Real-time tournament progression

fn main() {
    println!("=== Extended Multi-Table Tournament Demo ===\n");

    // Create a large tournament with multiple tables
    let mut mtt_manager = create_large_tournament();
    
    // Demonstrate table balancing
    demonstrate_table_balancing(&mut mtt_manager);
    
    // Simulate tournament progression
    simulate_tournament_progression(&mut mtt_manager);
    
    // Show final table dynamics
    demonstrate_final_table(&mut mtt_manager);
}

fn create_large_tournament() -> MTTManager {
    println!("Creating 180-player tournament with 20 tables...");
    
    // Create tournament structure
    let tournament_structure = TournamentStructure {
        levels: create_extended_blind_structure(),
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };
    
    // Create MTT with 180 players, 9 per table, prize pool of $18,000
    let mtt_manager = MTTManager::new(180, 9, tournament_structure, 18000);
    
    println!("Tournament created with {} players across {} tables", 
             180, mtt_manager.tables.len());
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
    println!("=== Table Balancing Demonstration ===");
    
    // Simulate some eliminations to trigger rebalancing
    println!("Simulating eliminations to trigger table balancing...");
    
    // Eliminate 3 players from table 0
    eliminate_players_from_table(mtt_manager, 0, 3);
    
    // Eliminate 4 players from table 1  
    eliminate_players_from_table(mtt_manager, 1, 4);
    
    // Eliminate 2 players from table 2
    eliminate_players_from_table(mtt_manager, 2, 2);
    
    println!("Before balancing:");
    print_table_summary(mtt_manager);
    
    // Trigger table balancing
    mtt_manager.balance_tables();
    
    println!("After balancing:");
    print_table_summary(mtt_manager);
    println!();
}

fn eliminate_players_from_table(mtt_manager: &mut MTTManager, table_id: usize, count: usize) {
    for i in 0..count {
        mtt_manager.eliminate_player(table_id as u32, (i + 1) as u32);
    }
    println!("Eliminated {} players from table {}", count, table_id);
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
        
        println!("  Table {}: {} players, avg stack: {}", i, player_count, avg_stack);
    }
}

fn simulate_tournament_progression(mtt_manager: &mut MTTManager) {
    println!("=== Tournament Progression Simulation ===");
    
    let mut eliminations = 0;
    let total_players = 180;
    
    // Simulate progression down to final table
    while mtt_manager.count_active_players() > 9 && eliminations < 100 {
        // Simulate elimination every few minutes
        let table_to_eliminate_from = eliminations % mtt_manager.tables.len();
        
        // Try to eliminate a player (use sequential player IDs)
        let player_id_to_eliminate = (eliminations + 1) as u32;
        mtt_manager.eliminate_player(table_to_eliminate_from as u32, player_id_to_eliminate);
        eliminations += 1;
        
        // Show progress every 20 eliminations
        if eliminations % 20 == 0 {
            let remaining = mtt_manager.count_active_players();
            let progress = ((total_players - remaining) as f64 / total_players as f64) * 100.0;
            println!("Progress: {:.1}% - {} players remaining", progress, remaining);
            
            // Show table consolidation
            if mtt_manager.tables.len() < 10 {
                print_table_summary(mtt_manager);
            }
        }
        
        // Rebalance tables every 5 eliminations
        if eliminations % 5 == 0 {
            mtt_manager.balance_tables();
        }
    }
    
    println!("Reached final table stage with {} players!", mtt_manager.count_active_players());
    println!();
}

fn demonstrate_final_table(mtt_manager: &mut MTTManager) {
    println!("=== Final Table Dynamics ===");
    
    let remaining = mtt_manager.count_active_players();
    println!("Final table reached with {} players", remaining);
    
    // Show ICM calculations for final table using tournament standings
    let standings = mtt_manager.get_tournament_standings();
    let stacks: Vec<u32> = standings.iter().map(|(_, stack, _)| *stack).collect();
    
    if !stacks.is_empty() {
        let total_chips: u32 = stacks.iter().sum();
        
        println!("Final table chip distribution:");
        for (i, &stack) in stacks.iter().enumerate() {
            let bb_count = stack / 1000; // Assuming 500/1000 blinds
            let percentage = (stack as f64 / total_chips as f64) * 100.0;
            println!("  Player {}: {} chips ({:.1}%, {} BB)", 
                     i + 1, stack, percentage, bb_count);
        }
        
        // Calculate ICM values
        let payout_structure = create_mtt_payout_structure(180);
        let final_payouts: Vec<f64> = payout_structure.into_iter().take(stacks.len()).collect();
        
        let icm_calculator = ICMCalculator::new(stacks.clone(), final_payouts.iter().map(|&x| x as u64).collect());
        let icm_values = icm_calculator.calculate_equity();
        
        println!("\nICM Values:");
        for (i, &icm_value) in icm_values.iter().enumerate() {
            let chip_value = (stacks[i] as f64 / total_chips as f64) * final_payouts.iter().sum::<f64>();
            let icm_pressure = ((icm_value - chip_value) / chip_value) * 100.0;
            println!("  Player {}: ${:.2} (chip EV: ${:.2}, ICM pressure: {:.1}%)", 
                     i + 1, icm_value, chip_value, icm_pressure);
        }
    }
    
    println!("\n=== Tournament Complete ===");
}
