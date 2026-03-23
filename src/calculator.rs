//! Calculator module - expression calculation core
//!
//! Provides calculation functions for both standard and bitwise modes.

use crate::evaluator::{eval_postfix, eval_postfix_bitwise};
use crate::linked_list::LinkedList;
use crate::shunting_yard::{infix_to_postfix, infix_to_postfix_bitwise};
use crate::token::{resolve_constants, tokenize};

/// Calculate expression in standard mode (floating point)
pub fn calculate(expression: &str) -> Result<f64, String> {
    let tokens = tokenize(expression, false);
    let tokens = resolve_constants(tokens);

    let mut list = LinkedList::new();
    for token in tokens {
        list.push_back(token);
    }

    let postfix = infix_to_postfix(list.to_vec());
    eval_postfix(postfix)
}

/// Calculate expression in bitwise mode (integer)
pub fn calculate_bitwise(expression: &str) -> Result<i64, String> {
    let tokens = tokenize(expression, true);
    let tokens = resolve_constants(tokens);

    let mut list = LinkedList::new();
    for token in tokens {
        list.push_back(token);
    }

    let postfix = infix_to_postfix_bitwise(list.to_vec());
    eval_postfix_bitwise(postfix)
}
