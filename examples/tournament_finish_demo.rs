use nice_hand_core::game::tournament::*;

fn main() {
    println!("=== Enhanced Tournament Finish Probability Demo ===\n");
    
    // Example tournament scenario: 4 players remaining
    let stacks = vec![5000, 3000, 1500, 500]; // Big stack, medium, short, very short
    let payouts = vec![10000, 6000, 4000, 2000]; // Prize structure
    
    let icm_calculator = ICMCalculator::new(stacks.clone(), payouts.clone());
    
    println!("Tournament Situation:");
    println!("Stacks: {:?}", stacks);
    println!("Payouts: {:?}", payouts);
    println!("Total Chips: {}\n", stacks.iter().sum::<u32>());
    
    // Calculate finish probabilities for each player in each position
    let remaining_players: Vec<usize> = (0..stacks.len()).collect();
    
    for player_idx in 0..stacks.len() {
        println!("Player {} (Stack: {}):", player_idx + 1, stacks[player_idx]);
        
        let mut total_prob = 0.0;
        for position in 0..stacks.len() {
            let finish_prob = icm_calculator.calculate_finish_probability_exact(
                player_idx, 
                position, 
                &remaining_players
            );
            
            let position_name = match position {
                0 => "1st",
                1 => "2nd", 
                2 => "3rd",
                3 => "4th",
                _ => "Other",
            };
            
            println!("  {} place: {:.2}%", position_name, finish_prob * 100.0);
            total_prob += finish_prob;
        }
        println!("  Total probability: {:.2}%\n", total_prob * 100.0);
    }
    
    // Calculate ICM equity for comparison
    let equities = icm_calculator.calculate_equity();
    println!("ICM Equities:");
    for (i, equity) in equities.iter().enumerate() {
        println!("  Player {}: ${:.2}", i + 1, equity);
    }
}
