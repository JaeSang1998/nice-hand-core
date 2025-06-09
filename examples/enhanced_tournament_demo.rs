use nice_hand_core::game::tournament::ICMCalculator;

fn main() {
    println!("=== 강화된 토너먼트 마무리 확률 데모 ===\n");
    
    // Example tournament scenario: 4 players remaining
    let stacks = vec![5000, 3000, 1500, 500]; // Big stack, medium, short, very short
    let payouts = vec![10000, 6000, 4000, 2000]; // Prize structure
    
    let icm_calculator = ICMCalculator::new(stacks.clone(), payouts.clone());
    
    println!("토너먼트 상황:");
    println!("스택: {:?}", stacks);
    println!("상금: {:?}", payouts);
    println!("총 칩: {}\n", stacks.iter().sum::<u32>());
    
    // Calculate finish probabilities for each player in each position
    let remaining_players: Vec<usize> = (0..stacks.len()).collect();
    
    for player_idx in 0..stacks.len() {
        println!("플레이어 {} (스택: {}):", player_idx + 1, stacks[player_idx]);
        
        let mut total_prob = 0.0;
        for position in 0..stacks.len() {
            let finish_prob = icm_calculator.calculate_finish_probability_exact(
                player_idx, 
                position, 
                &remaining_players
            );
            
            let position_name = match position {
                0 => "1등",
                1 => "2등", 
                2 => "3등",
                3 => "4등",
                _ => "기타",
            };
            
            println!("  {} 확률: {:.2}%", position_name, finish_prob * 100.0);
            total_prob += finish_prob;
        }
        println!("  총 확률: {:.2}%\n", total_prob * 100.0);
    }
    
    // Calculate ICM equity for comparison
    let equities = icm_calculator.calculate_equity();
    println!("ICM 지분:");
    for (i, equity) in equities.iter().enumerate() {
        println!("  플레이어 {}: ${:.2}", i + 1, equity);
    }
    
    println!("\n=== 극단적 스택 시나리오 ===");
    
    // More extreme scenario to test edge cases
    let extreme_stacks = vec![8000, 1000, 500, 500];
    let extreme_icm = ICMCalculator::new(extreme_stacks.clone(), payouts.clone());
    let extreme_remaining: Vec<usize> = (0..extreme_stacks.len()).collect();
    
    println!("극단적 스택: {:?}\n", extreme_stacks);
    
    // Show chip leader's probabilities
    println!("칩 리더 (플레이어 1) 마무리 확률:");
    for position in 0..extreme_stacks.len() {
        let prob = extreme_icm.calculate_finish_probability_exact(0, position, &extreme_remaining);
        let position_name = match position {
            0 => "1등",
            1 => "2등",
            2 => "3등", 
            3 => "4등",
            _ => "기타",
        };
        println!("  {} 확률: {:.2}%", position_name, prob * 100.0);
    }
    
    // Show short stack's probabilities
    println!("\n숏 스택 (플레이어 3) 마무리 확률:");
    for position in 0..extreme_stacks.len() {
        let prob = extreme_icm.calculate_finish_probability_exact(2, position, &extreme_remaining);
        let position_name = match position {
            0 => "1등",
            1 => "2등",
            2 => "3등",
            3 => "4등", 
            _ => "기타",
        };
        println!("  {} 확률: {:.2}%", position_name, prob * 100.0);
    }
}
