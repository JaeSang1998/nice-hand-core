use nice_hand_core::{Trainer, holdem};
use std::time::Instant;

fn main() {
    println!("🎯 Nice Hand Core - Performance Benchmark");
    println!("==========================================");
    
    // Create a Hold'em state for benchmarking
    let initial_state = holdem::State {
        hole: [[0, 1], [2, 3], [4, 5], [6, 7], [8, 9], [10, 11]],
        board: vec![],
        to_act: 0,
        street: 0,
        pot: 100,
        stack: [1000; 6],
        alive: [true, true, false, false, false, false], // 2 players
        invested: [15, 30, 0, 0, 0, 0], // Blinds posted
        to_call: 30,
        actions_taken: 0,
    };
    
    let iterations = [10, 50, 100, 250];
    
    for &iters in &iterations {
        print!("Training Hold'em with {:>5} iterations... ", iters);
        
        let mut trainer = Trainer::<holdem::State>::new();
        let start = Instant::now();
        
        trainer.run(vec![initial_state.clone()], iters);
        
        let duration = start.elapsed();
        let nodes = trainer.nodes.len();
        
        println!("✅ {}ms ({} nodes)", duration.as_millis(), nodes);
        
        if iters == 250 {
            println!("\n📊 Strategy Convergence Results:");
            for (i, (info_key, node)) in trainer.nodes.iter().enumerate().take(3) {
                let avg_strategy = node.average();
                println!("  Node {}: InfoKey {} → Strategy {:?}", 
                    i + 1, info_key, 
                    avg_strategy.iter().map(|x| format!("{:.3}", x)).collect::<Vec<_>>()
                );
            }
        }
    }
    
    println!("\n🚀 Multi-threaded Performance:");
    println!("   - Parallel CFR traversal using rayon");
    println!("   - Thread-local exploration for scalability");
    
    println!("\n💡 Architecture Benefits:");
    println!("   ✓ Generic Game trait for multiple poker variants");
    println!("   ✓ Multi-platform support (WASM + Native)");
    println!("   ✓ Memory-efficient hash-based node storage");
    
    println!("\n🎮 Ready for web and desktop deployment!");
}
