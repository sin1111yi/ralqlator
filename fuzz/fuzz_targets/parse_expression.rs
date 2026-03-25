// Ralqlator - Fuzzing Tests
// Fuzzing tests for parser and evaluator

#![no_main]

use libfuzzer_sys::fuzz_target;
use ralqlator::parser::parse_expression;

fuzz_target!(|data: &[u8]| {
    // Convert fuzz input to string
    if let Ok(input) = std::str::from_utf8(data) {
        // Skip empty or very long inputs
        if input.is_empty() || input.len() > 1000 {
            return;
        }
        
        // Try to parse the input
        let _ = parse_expression(input, false);
        let _ = parse_expression(input, true);
    }
});
