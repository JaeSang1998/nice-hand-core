use nice_hand_core::game::tournament::{BlindLevel, TournamentState};
use nice_hand_core::ICMCalculator;

fn main() {
    println!("=== ICM Pressure Analysis ===");

    // Example tournament state - 6 players remaining
    let stacks = vec![12000, 8500, 15000, 6000, 9500, 11000];
    let payout_structure = vec![50000, 30000, 20000]; // Top 3 get paid

    println!("Current stacks: {:?}", stacks);
    println!("Payout structure: {:?}", payout_structure);

    // Calculate ICM equity for each player
    let icm = ICMCalculator::new(stacks.clone(), payout_structure.clone());
    let equities = icm.calculate_equity();

    println!("\nICM Equities:");
    for (i, equity) in equities.iter().enumerate() {
        println!("Player {}: {:.2}%", i + 1, equity * 100.0);
    }

    // Analyze bubble pressure
    analyze_bubble_pressure(&stacks, &payout_structure);

    // Simulate different stack distributions
    simulate_stack_changes(&stacks, &payout_structure);

    // Additional analysis
    demonstrate_icm_vs_chip_value();
}

fn analyze_bubble_pressure(stacks: &[u32], payouts: &[u64]) {
    println!("\n=== Bubble Pressure Analysis ===");

    let icm = ICMCalculator::new(stacks.to_vec(), payouts.to_vec());
    let base_equities = icm.calculate_equity();

    // Find chip leader and short stack
    let max_stack = *stacks.iter().max().unwrap();
    let min_stack = *stacks.iter().min().unwrap();
    let chip_leader_idx = stacks.iter().position(|&x| x == max_stack).unwrap();
    let short_stack_idx = stacks.iter().position(|&x| x == min_stack).unwrap();

    println!(
        "Chip leader (Player {}): {} chips, {:.2}% equity",
        chip_leader_idx + 1,
        max_stack,
        base_equities[chip_leader_idx] * 100.0
    );
    println!(
        "Short stack (Player {}): {} chips, {:.2}% equity",
        short_stack_idx + 1,
        min_stack,
        base_equities[short_stack_idx] * 100.0
    );

    // Calculate risk of elimination for short stack
    let elimination_threshold = min_stack;
    println!(
        "Short stack elimination threshold: {} chips",
        elimination_threshold
    );

    // Simulate what happens if short stack doubles up
    let mut doubled_stacks = stacks.to_vec();
    doubled_stacks[short_stack_idx] *= 2;
    doubled_stacks[chip_leader_idx] -= min_stack; // Assuming chips came from chip leader

    let doubled_icm = ICMCalculator::new(doubled_stacks, payouts.to_vec());
    let doubled_equities = doubled_icm.calculate_equity();

    println!("\nIf short stack doubles up:");
    println!(
        "Short stack equity change: {:.2}% -> {:.2}% (+{:.2}%)",
        base_equities[short_stack_idx] * 100.0,
        doubled_equities[short_stack_idx] * 100.0,
        (doubled_equities[short_stack_idx] - base_equities[short_stack_idx]) * 100.0
    );
}

fn simulate_stack_changes(base_stacks: &[u32], payouts: &[u64]) {
    println!("\n=== Stack Change Simulations ===");

    let icm = ICMCalculator::new(base_stacks.to_vec(), payouts.to_vec());
    let base_equities = icm.calculate_equity();

    // Simulate 10% stack increase for each player
    for i in 0..base_stacks.len() {
        let mut modified_stacks = base_stacks.to_vec();
        let increase = (base_stacks[i] as f64 * 0.1) as u32;
        modified_stacks[i] += increase;

        // Remove chips proportionally from others
        let total_decrease = increase;
        let others_count = base_stacks.len() - 1;
        let decrease_per_other = total_decrease / others_count as u32;

        for j in 0..base_stacks.len() {
            if j != i {
                modified_stacks[j] = modified_stacks[j].saturating_sub(decrease_per_other);
            }
        }

        let new_icm = ICMCalculator::new(modified_stacks, payouts.to_vec());
        let new_equities = new_icm.calculate_equity();
        let equity_change = (new_equities[i] - base_equities[i]) * 100.0;

        println!(
            "Player {} +10% chips: {:.2}% equity change",
            i + 1,
            equity_change
        );
    }
}

fn demonstrate_icm_vs_chip_value() {
    println!("\n=== ICM vs. Chip Value Comparison ===");

    // Late tournament scenario
    let stacks = vec![20000, 15000, 10000, 5000];
    let payouts = vec![40000, 30000, 20000, 10000];
    let total_chips: u32 = stacks.iter().sum();

    let icm = ICMCalculator::new(stacks.clone(), payouts);
    let equities = icm.calculate_equity();

    println!("4-handed late tournament:");
    for i in 0..stacks.len() {
        let chip_percentage = (stacks[i] as f64 / total_chips as f64) * 100.0;
        let icm_percentage = equities[i] * 100.0;
        let difference = icm_percentage - chip_percentage;

        println!(
            "Player {}: {}% of chips, {:.1}% ICM equity (diff: {:.1}%)",
            i + 1,
            chip_percentage as u32,
            icm_percentage,
            difference
        );
    }
}
