/// Check if token is an operator
pub fn is_operator(token: &str) -> bool {
    matches!(
        token,
        "+" | "-" | "*" | "/" | "%" | "^" | "&" | "|" | "<<" | ">>"
    )
}

/// Check if token is a bitwise operator
pub fn is_bitwise_operator(token: &str) -> bool {
    matches!(token, "&" | "|" | "^" | "<<" | ">>" | "~")
}

/// Check if token is a unary operator
pub fn is_unary_operator(token: &str) -> bool {
    token == "~"
}

/// Check if token is a function
pub fn is_function(token: &str) -> bool {
    matches!(
        token,
        "lg" | "log"
            | "ln"
            | "sqrt"
            | "pow"
            | "sin"
            | "cos"
            | "tan"
            | "asin"
            | "acos"
            | "atan"
            | "mod"
    )
}

/// Operator precedence (standard mode)
pub fn precedence(op: &str) -> u8 {
    match op {
        "+" | "-" => 1,
        "*" | "/" | "%" => 2,
        "^" => 3,
        _ => 0,
    }
}

/// Operator precedence (bitwise mode)
pub fn bitwise_precedence(op: &str) -> u8 {
    match op {
        "|" => 1, // lowest
        "^" => 2,
        "&" => 3,
        "<<" | ">>" => 4,
        "~" => 5, // unary, highest
        _ => 0,
    }
}

/// Check operator associativity (true for left-associative, false for right-associative)
pub fn is_left_associative(op: &str) -> bool {
    op != "^" // Exponentiation is right-associative
}
