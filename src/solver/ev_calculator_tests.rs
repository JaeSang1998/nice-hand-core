use crate::solver::ev_calculator::*;
use crate::game::holdem::{State, Act};

#[test]
fn test_ev_config_creation() {
    let config = EVConfig::default();
    assert_eq!(config.sample_count, 10000);
    assert_eq!(config.max_depth, 10);
    assert!(config.use_opponent_model);
}

#[test]
fn test_action_ev_creation() {
    let action = Act::Call;
    let ev = ActionEV {
        action: action.clone(),
        ev: 100.0,
        confidence: 0.8,
    };
    assert_eq!(ev.action, action);
    assert_eq!(ev.ev, 100.0);
    assert_eq!(ev.confidence, 0.8);
}

#[test]
fn test_basic_ev_calculation() {
    let calculator = EVCalculator::new(EVConfig::default());
    
    // Create a test state
    let state = create_test_state();
    
    let results = calculator.calculate_action_evs(&state);
    // Should have at least one action
    assert!(!results.is_empty());
}

#[test]
fn test_ev_calculation_stability() {
    let config = EVConfig {
        sample_count: 100, // Smaller sample for faster testing
        max_depth: 5,
        use_opponent_model: true,
    };
    let calculator = EVCalculator::new(config);
    
    let state = create_test_state();
    let results = calculator.calculate_action_evs(&state);
    
    // Should have at least one action
    assert!(!results.is_empty());
    
    // All EVs should be finite
    for action_ev in results {
        assert!(action_ev.ev.is_finite());
        assert!(action_ev.confidence >= 0.0 && action_ev.confidence <= 1.0);
    }
}

#[test]
fn test_quick_ev_analysis() {
    let state = create_test_state();
    let results = quick_ev_analysis(&state, Some(50)); // Small sample for testing
    
    assert!(!results.is_empty());
    for action_ev in &results {
        assert!(action_ev.ev.is_finite());
        assert!(action_ev.confidence >= 0.0);
    }
}

#[test]
fn test_detailed_ev_analysis() {
    let state = create_test_state();
    let results = detailed_ev_analysis(&state);
    
    assert!(!results.is_empty());
    
    // All results should have finite EV values
    for action_ev in &results {
        assert!(action_ev.ev.is_finite());
        assert!(action_ev.confidence >= 0.0 && action_ev.confidence <= 1.0);
    }
}

#[test]
fn test_different_streets() {
    let config = EVConfig {
        sample_count: 50,
        max_depth: 3,
        use_opponent_model: false,
    };
    let calculator = EVCalculator::new(config);

    // Test preflop state
    let preflop_state = create_test_state_street(0); // 0 = Preflop
    let preflop_results = calculator.calculate_action_evs(&preflop_state);
    assert!(!preflop_results.is_empty());

    // Test flop state  
    let flop_state = create_test_state_street(1); // 1 = Flop
    let flop_results = calculator.calculate_action_evs(&flop_state);
    assert!(!flop_results.is_empty());
}

#[test]
fn test_confidence_bounds() {
    let config = EVConfig {
        sample_count: 50,
        max_depth: 3,
        use_opponent_model: false,
    };
    let calculator = EVCalculator::new(config);

    let state = create_test_state();
    let results = calculator.calculate_action_evs(&state);
    
    for action_ev in results {
        // Confidence should be within reasonable bounds
        assert!(action_ev.confidence >= 0.0);
        assert!(action_ev.confidence <= 1.0);
        assert!(action_ev.ev.is_finite());
    }
}

// Helper function to create a test state
fn create_test_state() -> State {
    create_test_state_street(0) // 0 = Preflop
}

fn create_test_state_street(street: u8) -> State {
    // Create a basic test state 
    let mut state = State::new(); // Use the default constructor
    
    // Set the street
    state.street = street;
    
    // Add some community cards based on street
    match street {
        0 => {
            // Preflop - no community cards
        },
        1 => {
            // Flop 
            state.board = vec![12, 24, 37]; // Ks, Qh, Jd
        },
        2 => {
            // Turn
            state.board = vec![12, 24, 37, 8]; // Ks, Qh, Jd, 9s
        },
        3 => {
            // River
            state.board = vec![12, 24, 37, 8, 21]; // Ks, Qh, Jd, 9s, 9h
        },
        _ => {
            // Default to preflop
        }
    }
    
    state.pot = 150;
    
    state
}
