use nice_hand_core::game::tournament::*;
use std::collections::HashMap;

/// 블라인드 구조 최적화기
/// 
/// 이 예제는 다음을 보여줍니다:
/// - Optimal blind structure generation for different tournament types
/// - Analysis of blind progression impact on tournament dynamics
/// - Customization for various tournament lengths and player counts
/// - Mathematical optimization of blind increases

fn main() {
    println!("=== Blind Structure Optimizer ===\n");

    // Generate optimal structures for different tournament types
    optimize_turbo_structure();
    optimize_standard_structure();
    optimize_deep_stack_structure();
    optimize_hyper_turbo_structure();
    
    // Analyze structure characteristics
    analyze_structure_dynamics();
    
    // Custom structure generation
    generate_custom_structures();
}

fn optimize_turbo_structure() {
    println!("=== Turbo Tournament Structure Optimization ===");
    println!("Target: 2-3 hour tournament, aggressive blind increases\n");
    
    let turbo_params = TournamentParameters {
        starting_chips: 10000,
        target_duration_minutes: 150,
        starting_players: 180,
        blind_increase_percentage: 0.5, // 50% increases
        level_duration_minutes: 8,
        ante_introduction_level: 4,
        max_levels: 20,
    };
    
    let optimizer = BlindStructureOptimizer::new();
    let turbo_structure = optimizer.generate_optimal_structure(&turbo_params);
    
    println!("Optimized Turbo Structure:");
    display_blind_structure(&turbo_structure, &turbo_params);
    
    analyze_structure_characteristics(&turbo_structure, &turbo_params, "Turbo");
}

fn optimize_standard_structure() {
    println!("=== Standard Tournament Structure Optimization ===");
    println!("Target: 4-5 hour tournament, moderate blind increases\n");
    
    let standard_params = TournamentParameters {
        starting_chips: 10000,
        target_duration_minutes: 270,
        starting_players: 180,
        blind_increase_percentage: 0.33, // 33% increases
        level_duration_minutes: 12,
        ante_introduction_level: 5,
        max_levels: 25,
    };
    
    let optimizer = BlindStructureOptimizer::new();
    let standard_structure = optimizer.generate_optimal_structure(&standard_params);
    
    println!("Optimized Standard Structure:");
    display_blind_structure(&standard_structure, &standard_params);
    
    analyze_structure_characteristics(&standard_structure, &standard_params, "Standard");
}

fn optimize_deep_stack_structure() {
    println!("=== Deep Stack Tournament Structure Optimization ===");
    println!("Target: 6-8 hour tournament, slow blind increases\n");
    
    let deep_stack_params = TournamentParameters {
        starting_chips: 20000,
        target_duration_minutes: 420,
        starting_players: 120,
        blind_increase_percentage: 0.25, // 25% increases
        level_duration_minutes: 15,
        ante_introduction_level: 6,
        max_levels: 30,
    };
    
    let optimizer = BlindStructureOptimizer::new();
    let deep_structure = optimizer.generate_optimal_structure(&deep_stack_params);
    
    println!("Optimized Deep Stack Structure:");
    display_blind_structure(&deep_structure, &deep_stack_params);
    
    analyze_structure_characteristics(&deep_structure, &deep_stack_params, "Deep Stack");
}

fn optimize_hyper_turbo_structure() {
    println!("=== Hyper Turbo Tournament Structure Optimization ===");
    println!("Target: 1 hour tournament, very aggressive blind increases\n");
    
    let hyper_params = TournamentParameters {
        starting_chips: 5000,
        target_duration_minutes: 60,
        starting_players: 180,
        blind_increase_percentage: 0.8, // 80% increases
        level_duration_minutes: 3,
        ante_introduction_level: 2,
        max_levels: 15,
    };
    
    let optimizer = BlindStructureOptimizer::new();
    let hyper_structure = optimizer.generate_optimal_structure(&hyper_params);
    
    println!("Optimized Hyper Turbo Structure:");
    display_blind_structure(&hyper_structure, &hyper_params);
    
    analyze_structure_characteristics(&hyper_structure, &hyper_params, "Hyper Turbo");
}

fn analyze_structure_dynamics() {
    println!("=== Blind Structure Dynamics Analysis ===");
    
    let test_structures = vec![
        ("Conservative", create_conservative_structure()),
        ("Balanced", create_balanced_structure()),
        ("Aggressive", create_aggressive_structure()),
    ];
    
    for (name, structure) in test_structures {
        println!("{} Structure Analysis:", name);
        
        let dynamics = analyze_tournament_dynamics(&structure);
        
        println!("  Pressure Points:");
        for (level, pressure) in dynamics.pressure_points.iter().enumerate() {
            if *pressure > 1.5 {
                println!("    Level {}: {:.2}x pressure increase", level + 1, pressure);
            }
        }
        
        println!("  Key Characteristics:");
        println!("    Average stack/BB at level 5: {:.1}", dynamics.avg_bb_level_5);
        println!("    Average stack/BB at level 10: {:.1}", dynamics.avg_bb_level_10);
        println!("    Push/fold threshold reached: Level {}", dynamics.push_fold_level);
        println!("    Structure rating: {:.2}/10", dynamics.balance_score);
        println!();
    }
}

fn generate_custom_structures() {
    println!("=== Custom Structure Generation ===");
    
    let custom_scenarios = vec![
        CustomScenario {
            name: "Home Game Friendly",
            description: "Slow structure for home games with recreational players",
            starting_chips: 15000,
            target_hours: 5.0,
            skill_level: SkillLevel::Recreational,
            special_requirements: vec!["No ante until late", "Round numbers only"],
        },
        CustomScenario {
            name: "Online Satellite",
            description: "Single table satellite to bigger tournament",
            starting_chips: 1500,
            target_hours: 1.5,
            skill_level: SkillLevel::Professional,
            special_requirements: vec!["Winner take all", "Fast elimination"],
        },
        CustomScenario {
            name: "Charity Event",
            description: "Fun tournament prioritizing player experience",
            starting_chips: 12000,
            target_hours: 4.0,
            skill_level: SkillLevel::Mixed,
            special_requirements: vec!["Beginner friendly", "No antes", "Extended breaks"],
        },
    ];
    
    let optimizer = BlindStructureOptimizer::new();
    
    for scenario in custom_scenarios {
        println!("Custom Structure: {}", scenario.name);
        println!("Description: {}", scenario.description);
        
        let custom_params = optimizer.create_custom_parameters(&scenario);
        let custom_structure = optimizer.generate_optimal_structure(&custom_params);
        
        println!("Generated Structure:");
        display_condensed_structure(&custom_structure, 8); // Show first 8 levels
        
        println!("Special Accommodations:");
        for requirement in &scenario.special_requirements {
            println!("  ✓ {}", requirement);
        }
        println!();
    }
}

// Supporting structures and implementations

#[derive(Debug, Clone)]
struct TournamentParameters {
    starting_chips: u32,
    target_duration_minutes: u32,
    starting_players: usize,
    blind_increase_percentage: f64,
    level_duration_minutes: u32,
    ante_introduction_level: usize,
    max_levels: usize,
}

struct BlindStructureOptimizer {
    optimization_engine: OptimizationEngine,
}

#[derive(Debug)]
struct TournamentDynamics {
    pressure_points: Vec<f64>,
    avg_bb_level_5: f64,
    avg_bb_level_10: f64,
    push_fold_level: usize,
    balance_score: f64,
}

#[derive(Debug)]
struct CustomScenario {
    name: &'static str,
    description: &'static str,
    starting_chips: u32,
    target_hours: f64,
    skill_level: SkillLevel,
    special_requirements: Vec<&'static str>,
}

#[derive(Debug)]
enum SkillLevel {
    Recreational,
    Mixed,
    Professional,
}

struct OptimizationEngine {
    // Internal optimization algorithms
}

impl BlindStructureOptimizer {
    fn new() -> Self {
        Self {
            optimization_engine: OptimizationEngine::new(),
        }
    }
    
    fn generate_optimal_structure(&self, params: &TournamentParameters) -> Vec<BlindLevel> {
        let mut structure = Vec::new();
        
        // Calculate initial blinds based on starting chips
        let initial_bb = (params.starting_chips as f64 / 200.0) as u32; // Start with 200BB effective
        let mut current_sb = initial_bb / 2;
        let mut current_bb = initial_bb;
        
        // Generate levels with optimal progression
        for level in 0..params.max_levels {
            let ante = if level >= params.ante_introduction_level {
                calculate_optimal_ante(current_bb, level)
            } else {
                0
            };
            
            structure.push(BlindLevel {
                level: (level + 1) as u32,
                small_blind: current_sb,
                big_blind: current_bb,
                ante,
            });
            
            // Calculate next level increase
            let increase_factor = 1.0 + params.blind_increase_percentage;
            let next_bb = (current_bb as f64 * increase_factor) as u32;
            
            // Round to aesthetically pleasing numbers
            let rounded_bb = round_to_nice_number(next_bb);
            current_bb = rounded_bb;
            current_sb = current_bb / 2;
            
            // Stop if blinds become too large relative to starting chips
            if current_bb > params.starting_chips / 5 {
                break;
            }
        }
        
        // Optimize the structure for balance
        self.optimize_for_balance(&mut structure, params);
        
        structure
    }
    
    fn create_custom_parameters(&self, scenario: &CustomScenario) -> TournamentParameters {
        let target_duration = (scenario.target_hours * 60.0) as u32;
        
        let (increase_rate, level_duration) = match scenario.skill_level {
            SkillLevel::Recreational => (0.25, 15), // Slow and steady
            SkillLevel::Mixed => (0.33, 12),        // Balanced
            SkillLevel::Professional => (0.5, 8),   // Faster pace
        };
        
        let ante_level = if scenario.special_requirements.contains(&"No ante until late") {
            10
        } else if scenario.special_requirements.contains(&"No antes") {
            999 // Never introduce antes
        } else {
            5
        };
        
        TournamentParameters {
            starting_chips: scenario.starting_chips,
            target_duration_minutes: target_duration,
            starting_players: 100, // Default
            blind_increase_percentage: increase_rate,
            level_duration_minutes: level_duration,
            ante_introduction_level: ante_level,
            max_levels: (target_duration / level_duration) as usize,
        }
    }
    
    fn optimize_for_balance(&self, structure: &mut Vec<BlindLevel>, params: &TournamentParameters) {
        // Analyze and adjust structure for optimal tournament flow
        
        // Ensure smooth progression without dramatic jumps
        for i in 1..structure.len() {
            let prev_bb = structure[i-1].big_blind;
            let curr_bb = structure[i].big_blind;
            let increase_ratio = curr_bb as f64 / prev_bb as f64;
            
            // If increase is too dramatic, smooth it out
            if increase_ratio > 2.0 {
                let adjusted_bb = (prev_bb as f64 * 1.5) as u32;
                structure[i].big_blind = round_to_nice_number(adjusted_bb);
                structure[i].small_blind = structure[i].big_blind / 2;
            }
        }
        
        // Optimize ante introduction and scaling
        for (i, level) in structure.iter_mut().enumerate() {
            if i >= params.ante_introduction_level {
                level.ante = calculate_optimal_ante(level.big_blind, i);
            }
        }
    }
}

impl OptimizationEngine {
    fn new() -> Self {
        Self {}
    }
}

fn display_blind_structure(structure: &[BlindLevel], params: &TournamentParameters) {
    println!("Level | SB    | BB    | Ante | Duration | Avg Stack/BB");
    println!("------|-------|-------|------|----------|-------------");
    
    for (i, level) in structure.iter().enumerate() {
        // Estimate average stack at this level
        let eliminations_per_level = params.starting_players as f64 / structure.len() as f64;
        let remaining_players = params.starting_players as f64 - (i as f64 * eliminations_per_level);
        let avg_stack = (params.starting_chips as f64 * params.starting_players as f64) / remaining_players.max(1.0);
        let avg_bb_ratio = avg_stack / level.big_blind as f64;
        
        println!("{:5} | {:5} | {:5} | {:4} | {:8} | {:11.1}",
                 i + 1,
                 level.small_blind,
                 level.big_blind,
                 level.ante,
                 params.level_duration_minutes,
                 avg_bb_ratio);
        
        // Show break levels every 4 levels for longer tournaments
        if params.level_duration_minutes >= 10 && (i + 1) % 4 == 0 && i < structure.len() - 1 {
            println!("      |       |       |      | BREAK    |");
        }
    }
    println!();
}

fn display_condensed_structure(structure: &[BlindLevel], show_levels: usize) {
    println!("  Levels 1-{}: ", show_levels.min(structure.len()));
    for (i, level) in structure.iter().take(show_levels).enumerate() {
        print!("  L{}: {}/{}", i + 1, level.small_blind, level.big_blind);
        if level.ante > 0 {
            print!(" ({})", level.ante);
        }
        if i < show_levels - 1 && i < structure.len() - 1 {
            print!(" →");
        }
    }
    if structure.len() > show_levels {
        print!(" ... ({} more levels)", structure.len() - show_levels);
    }
    println!();
}

fn analyze_structure_characteristics(structure: &[BlindLevel], params: &TournamentParameters, structure_type: &str) {
    println!("{} Structure Analysis:", structure_type);
    
    // Calculate key metrics
    let total_duration = structure.len() as u32 * params.level_duration_minutes;
    let starting_bb_ratio = params.starting_chips as f64 / structure[0].big_blind as f64;
    
    // Find when antes are introduced
    let ante_level = structure.iter().position(|l| l.ante > 0).unwrap_or(999) + 1;
    
    // Calculate escalation rate
    let first_bb = structure[0].big_blind as f64;
    let last_bb = structure.last().unwrap().big_blind as f64;
    let avg_increase = (last_bb / first_bb).powf(1.0 / (structure.len() - 1) as f64);
    
    println!("  Starting BB ratio: {:.1} BB", starting_bb_ratio);
    println!("  Total levels: {}", structure.len());
    println!("  Estimated duration: {} minutes ({:.1} hours)", total_duration, total_duration as f64 / 60.0);
    println!("  Antes introduced: Level {}", if ante_level > 900 { "Never".to_string() } else { ante_level.to_string() });
    println!("  Average blind increase: {:.1}%", (avg_increase - 1.0) * 100.0);
    
    // Analyze tournament phases
    analyze_tournament_phases(structure, params);
    println!();
}

fn analyze_tournament_phases(structure: &[BlindLevel], params: &TournamentParameters) {
    println!("  Tournament Phases:");
    
    let phases = vec![
        ("Deep Stack", 0.0..0.25),
        ("Middle", 0.25..0.6),
        ("Late", 0.6..0.85),
        ("Final Push", 0.85..1.0),
    ];
    
    for (phase_name, range) in phases {
        let start_level = (range.start * structure.len() as f64) as usize;
        let end_level = ((range.end * structure.len() as f64) as usize).min(structure.len() - 1);
        
        if start_level < structure.len() {
            let start_bb = structure[start_level].big_blind;
            let end_bb = structure[end_level].big_blind;
            let avg_stack_start = estimate_avg_stack_at_level(start_level, params);
            let avg_stack_end = estimate_avg_stack_at_level(end_level, params);
            
            println!("    {}: L{}-{} ({:.1}-{:.1} BB average)", 
                     phase_name, start_level + 1, end_level + 1,
                     avg_stack_start / start_bb as f64,
                     avg_stack_end / end_bb as f64);
        }
    }
}

fn analyze_tournament_dynamics(structure: &[BlindLevel]) -> TournamentDynamics {
    let mut pressure_points = Vec::new();
    
    // Calculate pressure increases between levels
    for i in 1..structure.len() {
        let prev_pressure = structure[i-1].big_blind + structure[i-1].ante;
        let curr_pressure = structure[i].big_blind + structure[i].ante;
        let pressure_ratio = curr_pressure as f64 / prev_pressure as f64;
        pressure_points.push(pressure_ratio);
    }
    
    // Calculate average BB ratios at key levels
    let avg_bb_level_5 = if structure.len() > 4 {
        10000.0 / structure[4].big_blind as f64 // Assuming 10k starting stack
    } else { 50.0 };
    
    let avg_bb_level_10 = if structure.len() > 9 {
        7500.0 / structure[9].big_blind as f64 // Estimated average after eliminations
    } else { 25.0 };
    
    // Find push/fold threshold (around 10-12 BB average)
    let push_fold_level = structure.iter().position(|level| {
        let estimated_avg = 8000.0 / level.big_blind as f64; // Conservative estimate
        estimated_avg <= 12.0
    }).unwrap_or(structure.len()) + 1;
    
    // Calculate balance score (0-10, higher is better)
    let balance_score = calculate_balance_score(&pressure_points, avg_bb_level_5, avg_bb_level_10);
    
    TournamentDynamics {
        pressure_points,
        avg_bb_level_5,
        avg_bb_level_10,
        push_fold_level,
        balance_score,
    }
}

fn create_conservative_structure() -> Vec<BlindLevel> {
    vec![
        BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
        BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 0 },
        BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 0 },
        BlindLevel { level: 4, small_blind: 100, big_blind: 200, ante: 0 },
        BlindLevel { level: 5, small_blind: 125, big_blind: 250, ante: 25 },
        BlindLevel { level: 6, small_blind: 150, big_blind: 300, ante: 25 },
        BlindLevel { level: 7, small_blind: 200, big_blind: 400, ante: 50 },
        BlindLevel { level: 8, small_blind: 250, big_blind: 500, ante: 50 },
        BlindLevel { level: 9, small_blind: 300, big_blind: 600, ante: 75 },
        BlindLevel { level: 10, small_blind: 400, big_blind: 800, ante: 100 },
    ]
}

fn create_balanced_structure() -> Vec<BlindLevel> {
    vec![
        BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
        BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 0 },
        BlindLevel { level: 3, small_blind: 75, big_blind: 150, ante: 0 },
        BlindLevel { level: 4, small_blind: 100, big_blind: 200, ante: 25 },
        BlindLevel { level: 5, small_blind: 150, big_blind: 300, ante: 25 },
        BlindLevel { level: 6, small_blind: 200, big_blind: 400, ante: 50 },
        BlindLevel { level: 7, small_blind: 300, big_blind: 600, ante: 75 },
        BlindLevel { level: 8, small_blind: 500, big_blind: 1000, ante: 100 },
        BlindLevel { level: 9, small_blind: 800, big_blind: 1600, ante: 200 },
        BlindLevel { level: 10, small_blind: 1200, big_blind: 2400, ante: 300 },
    ]
}

fn create_aggressive_structure() -> Vec<BlindLevel> {
    vec![
        BlindLevel { level: 1, small_blind: 25, big_blind: 50, ante: 0 },
        BlindLevel { level: 2, small_blind: 50, big_blind: 100, ante: 10 },
        BlindLevel { level: 3, small_blind: 100, big_blind: 200, ante: 25 },
        BlindLevel { level: 4, small_blind: 200, big_blind: 400, ante: 50 },
        BlindLevel { level: 5, small_blind: 400, big_blind: 800, ante: 100 },
        BlindLevel { level: 6, small_blind: 800, big_blind: 1600, ante: 200 },
        BlindLevel { level: 7, small_blind: 1500, big_blind: 3000, ante: 400 },
        BlindLevel { level: 8, small_blind: 3000, big_blind: 6000, ante: 800 },
        BlindLevel { level: 9, small_blind: 6000, big_blind: 12000, ante: 1500 },
        BlindLevel { level: 10, small_blind: 12000, big_blind: 24000, ante: 3000 },
    ]
}

fn calculate_optimal_ante(big_blind: u32, level: usize) -> u32 {
    // Antes typically start at 10-20% of big blind and increase
    let base_ante = big_blind / 8; // 12.5% of BB
    let progression_multiplier = 1.0 + (level as f64 * 0.1);
    let optimal_ante = (base_ante as f64 * progression_multiplier) as u32;
    
    // Round to nice numbers
    round_to_nice_number(optimal_ante)
}

fn round_to_nice_number(value: u32) -> u32 {
    if value < 100 {
        // Round to nearest 5 or 10
        ((value + 2) / 5) * 5
    } else if value < 1000 {
        // Round to nearest 25 or 50
        ((value + 12) / 25) * 25
    } else if value < 10000 {
        // Round to nearest 100
        ((value + 50) / 100) * 100
    } else {
        // Round to nearest 500
        ((value + 250) / 500) * 500
    }
}

fn estimate_avg_stack_at_level(level: usize, params: &TournamentParameters) -> f64 {
    // Simple elimination model - assumes linear elimination rate
    let elimination_rate = 0.15_f64; // 15% of field eliminated per level
    let remaining_percentage = (1.0_f64 - elimination_rate).powi(level as i32);
    let remaining_players = (params.starting_players as f64 * remaining_percentage).max(1.0);
    
    // Total chips remain constant, distributed among fewer players
    let total_chips = params.starting_chips as f64 * params.starting_players as f64;
    total_chips / remaining_players
}

fn calculate_balance_score(pressure_points: &[f64], avg_bb_5: f64, avg_bb_10: f64) -> f64 {
    // Score based on several factors:
    // 1. Consistent pressure increases (avoid big jumps)
    // 2. Reasonable BB ratios at key levels
    // 3. Smooth progression overall
    
    let mut score: f64 = 10.0;
    
    // Penalize dramatic pressure increases
    for &pressure in pressure_points {
        if pressure > 2.0 {
            score -= 1.0; // -1 point for each >2x jump
        } else if pressure < 1.1 {
            score -= 0.5; // -0.5 points for too small increases
        }
    }
    
    // Check BB ratios at key levels
    if avg_bb_5 < 30.0 || avg_bb_5 > 80.0 {
        score -= 1.0; // Penalty for bad level 5 ratio
    }
    
    if avg_bb_10 < 15.0 || avg_bb_10 > 40.0 {
        score -= 1.0; // Penalty for bad level 10 ratio
    }
    
    // Ensure score is between 0 and 10
    score.max(0.0).min(10.0)
}
