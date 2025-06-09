use nice_hand_core::*;
use std::time::Instant;

fn main() {
    println!("🔍 Nice Hand Core - Project Status & Next Steps");
    println!("===============================================");
    
    current_capabilities_demo();
    development_priorities();
}

fn current_capabilities_demo() {
    println!("\n✅ CURRENT WORKING FEATURES:");
    println!("────────────────────────────");
    
    // CFR Training Demo
    let start = Instant::now();
    let trainer = api::web_api::OfflineTrainer::train_simple_strategy(10);
    let cfr_time = start.elapsed();
    
    println!("🧠 CFR Training: {} nodes in {:?}", trainer.nodes.len(), cfr_time);
    
    // Web API Demo
    let start = Instant::now();
    let api = api::web_api_simple::QuickPokerAPI::new();
    let init_time = start.elapsed();
    
    let state = api::web_api_simple::WebGameState {
        hole_cards: [52, 53], // As, Ah (example values)
        board: vec![12, 25, 38], // Kh, Qd, Jc (example values)
        street: 1, // Flop
        pot: 100,
        to_call: 50,
        my_stack: 1000,
        opponent_stack: 1000,
    };
    
    let result = api.get_optimal_strategy(state.clone());
    println!("🌐 Web API: Init in {:?}, Action: {}", init_time, result.recommended_action);
    
    // Performance Test
    let start = Instant::now();
    for _ in 0..100 {
        let _ = api.get_optimal_strategy(state.clone());
    }
    let perf_time = start.elapsed();
    
    println!("⚡ Performance: 100 decisions in {:?} ({:.2}μs avg)", 
             perf_time, perf_time.as_micros() as f64 / 100.0);
}

fn development_priorities() {
    println!("\n🚀 NEXT DEVELOPMENT PRIORITIES:");
    println!("───────────────────────────────");
    
    println!("🏆 1. TOURNAMENT SUPPORT (1-2 weeks)");
    println!("   • Fix tournament module compilation");
    println!("   • ICM calculations for equity");
    println!("   • Blind structure management");
    println!("   • Bubble strategy adjustments");
    
    println!("\n🧠 2. ADVANCED AI (2-3 weeks)");
    println!("   • Opponent modeling");
    println!("   • Range analysis");
    println!("   • Exploitative strategies");
    println!("   • Meta-game adaptation");
    
    println!("\n🌐 3. WEB INTEGRATION (2-3 weeks)");
    println!("   • WASM browser support");
    println!("   • WebSocket multiplayer");
    println!("   • Database integration");
    println!("   • React/Vue components");
    
    println!("\n📊 4. ANALYTICS & TOOLS (1-2 weeks)");
    println!("   • Real-time HUD");
    println!("   • Session analysis");
    println!("   • Hand history tracking");
    println!("   • Performance profiling");
    
    println!("\n🎯 IMMEDIATE TASKS (This Week):");
    println!("   1. Fix tournament module exports");
    println!("   2. Add comprehensive documentation");
    println!("   3. Expand test coverage");
    println!("   4. Create performance benchmarks");
    println!("   5. Implement error handling");
    
    println!("\n💡 CHOOSE A PRIORITY TO IMPLEMENT NEXT!");
    println!("   The library foundation is solid and ready for expansion.");
    println!("   Which area would you like to develop first?");
}
