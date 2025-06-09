use nice_hand_core::game::tournament::*;
use nice_hand_core::ICMCalculator;

fn main() {
    println!("=== MTT Demo Extended ===");

    // Create blind structure
    let blind_structure = vec![
        BlindLevel {
            level: 1,
            small_blind: 25,
            big_blind: 50,
            ante: 0,
        },
        BlindLevel {
            level: 2,
            small_blind: 50,
            big_blind: 100,
            ante: 0,
        },
        BlindLevel {
            level: 3,
            small_blind: 75,
            big_blind: 150,
            ante: 25,
        },
        BlindLevel {
            level: 4,
            small_blind: 100,
            big_blind: 200,
            ante: 25,
        },
        BlindLevel {
            level: 5,
            small_blind: 150,
            big_blind: 300,
            ante: 50,
        },
    ];

    // Create tournament structure
    let tournament_structure = TournamentStructure {
        levels: blind_structure.clone(),
        level_duration_minutes: 20,
        starting_stack: 10000,
        ante_schedule: vec![],
    };

    // Initialize tournament
    let buy_in = 100;
    let starting_stack = 10000;
    let player_count = 180;
    let prize_pool = (player_count * buy_in) as u64;

    // Create tournament manager
    let mut manager = MTTManager::new(player_count, 9, tournament_structure, prize_pool);

    println!("Tournament created with {} players", player_count);
    println!("Starting stack: {}", starting_stack);
    println!("Buy-in: ${}", buy_in);

    // Simulate tournament progression
    let mut current_level = 0;

    for round in 1..=10 {
        println!("\n--- Round {} ---", round);

        // Check if we need to advance blind level
        if round % 3 == 0 && current_level < blind_structure.len() - 1 {
            current_level += 1;
            println!(
                "Advancing to blind level {}: {}/{} (ante: {})",
                current_level + 1,
                blind_structure[current_level].small_blind,
                blind_structure[current_level].big_blind,
                blind_structure[current_level].ante
            );
        }

        let remaining_players = manager.count_active_players();
        println!("Players remaining: {}", remaining_players);

        if remaining_players <= 10 {
            println!("Final table reached!");
            demonstrate_icm_calculations(&manager);
            break;
        }

        // Simulate some eliminations
        if remaining_players > 10 {
            let eliminations = std::cmp::min(
                (remaining_players as f32 * 0.1) as u32,
                remaining_players - 10,
            );

            for i in 0..eliminations {
                if manager.count_active_players() > 10 {
                    // Eliminate players from different tables
                    let table_id = (i % manager.tables.len() as u32) as u32;
                    let player_id = i + 1; // Use sequential player IDs
                    manager.eliminate_player(table_id, player_id);
                }
            }
        }
    }

    println!("\n=== Tournament Complete ===");
    let final_standings = manager.get_tournament_standings();
    println!("Final standings: {} players", final_standings.len());
}

fn demonstrate_icm_calculations(manager: &MTTManager) {
    println!("\n=== ICM Analysis ===");

    let standings = manager.get_tournament_standings();
    let stacks: Vec<u32> = standings.iter().map(|(_, stack, _)| *stack).collect();
    let payouts = vec![
        50000u64, 30000, 20000, 15000, 12000, 10000, 8000, 6000, 4000, 3000,
    ];

    if !stacks.is_empty() {
        let icm_calculator = ICMCalculator::new(stacks, payouts);
        let equity_values = icm_calculator.calculate_equity();

        println!("ICM equity distribution:");
        for (i, equity) in equity_values.iter().enumerate() {
            if i < standings.len() {
                let (player_id, _, _) = standings[i];
                println!("Player {}: ${:.2}", player_id, equity);
            }
        }
    }
}
