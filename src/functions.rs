/// Calculate logarithm with custom base: log_base(base, x) = ln(x) / ln(base)
pub fn eval_log_base(base: f64, x: f64) -> Result<f64, String> {
    if base <= 0.0 || base == 1.0 {
        return Err("log: base must be positive and not equal to 1".to_string());
    }
    if x <= 0.0 {
        return Err("log: argument must be positive".to_string());
    }
    Ok(x.ln() / base.ln())
}

/// Evaluate lg (base 10 logarithm)
pub fn eval_lg(x: f64) -> Result<f64, String> {
    if x <= 0.0 {
        return Err("lg: argument must be positive".to_string());
    }
    Ok(x.log10())
}

/// Evaluate ln (natural logarithm)
pub fn eval_ln(x: f64) -> Result<f64, String> {
    if x <= 0.0 {
        return Err("ln: argument must be positive".to_string());
    }
    Ok(x.ln())
}

/// Evaluate sqrt (square root)
pub fn eval_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        return Err("sqrt: argument must be non-negative".to_string());
    }
    Ok(x.sqrt())
}

/// Evaluate pow (power function)
pub fn eval_pow(base: f64, exp: f64) -> Result<f64, String> {
    Ok(base.powf(exp))
}

/// Evaluate sin (sine function, input in radians)
pub fn eval_sin(x: f64) -> f64 {
    x.sin()
}

/// Evaluate cos (cosine function, input in radians)
pub fn eval_cos(x: f64) -> f64 {
    x.cos()
}

/// Evaluate tan (tangent function, input in radians)
pub fn eval_tan(x: f64) -> Result<f64, String> {
    // Check for undefined values (cos(x) == 0)
    if x.cos() == 0.0 {
        return Err("tan: undefined at this value".to_string());
    }
    Ok(x.tan())
}

/// Evaluate asin (inverse sine function, output in radians)
pub fn eval_asin(x: f64) -> Result<f64, String> {
    if !(-1.0..=1.0).contains(&x) {
        return Err("asin: argument must be in range [-1, 1]".to_string());
    }
    Ok(x.asin())
}

/// Evaluate acos (inverse cosine function, output in radians)
pub fn eval_acos(x: f64) -> Result<f64, String> {
    if !(-1.0..=1.0).contains(&x) {
        return Err("acos: argument must be in range [-1, 1]".to_string());
    }
    Ok(x.acos())
}

/// Evaluate atan (inverse tangent function, output in radians)
pub fn eval_atan(x: f64) -> f64 {
    x.atan()
}

/// Evaluate mod (modulo function)
pub fn eval_mod(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("mod: modulo by zero".to_string());
    }
    Ok(a % b)
}
