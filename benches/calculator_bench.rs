// Ralqlator - Calculator Benchmark Tests
// Performance benchmarks for core calculator functionality

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

// Import calculator modules
use ralqlator::parser::{parse_expression, parse_and_eval};
use ralqlator::functions::UserFunctions;
use ralqlator::calculator::UserConstants;
use std::sync::{Arc, Mutex};

/// Create empty context for evaluation
fn empty_context() -> (UserFunctions, UserConstants) {
    (
        Arc::new(Mutex::new(std::collections::HashMap::new())),
        Arc::new(Mutex::new(std::collections::HashMap::new())),
    )
}

// ==================== Parsing Benchmarks ====================

fn bench_parse_simple(c: &mut Criterion) {
    c.bench_function("parse_simple_1_plus_2", |b| {
        b.iter(|| parse_expression(black_box("1 + 2"), false))
    });
}

fn bench_parse_complex(c: &mut Criterion) {
    c.bench_function("parse_complex_expression", |b| {
        b.iter(|| {
            parse_expression(
                black_box("(1 + 2) * 3 - 4 / 2 + 5 ^ 2"),
                false,
            )
        })
    });
}

fn bench_parse_function_call(c: &mut Criterion) {
    c.bench_function("parse_function_call", |b| {
        b.iter(|| parse_expression(black_box("sin(x) + cos(y)"), false))
    });
}

fn bench_parse_nested_functions(c: &mut Criterion) {
    c.bench_function("parse_nested_functions", |b| {
        b.iter(|| {
            parse_expression(
                black_box("sqrt(pow(sin(x), 2) + pow(cos(x), 2))"),
                false,
            )
        })
    });
}

fn bench_parse_scientific_notation(c: &mut Criterion) {
    c.bench_function("parse_scientific_notation", |b| {
        b.iter(|| parse_expression(black_box("1.23e-4 + 5.67e8"), false))
    });
}

fn bench_parse_hex_input(c: &mut Criterion) {
    c.bench_function("parse_hex_input", |b| {
        b.iter(|| parse_expression(black_box("0xFF + 0b1010 * 0o755"), false))
    });
}

// ==================== Evaluation Benchmarks ====================

fn bench_eval_arithmetic(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_arithmetic", |b| {
        b.iter(|| {
            parse_and_eval(black_box("1 + 2 * 3 - 4 / 2"), false, &funcs, &consts)
        })
    });
}

fn bench_eval_exponentiation(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_exponentiation", |b| {
        b.iter(|| {
            parse_and_eval(black_box("2 ^ 10 + 3 ^ 5"), false, &funcs, &consts)
        })
    });
}

fn bench_eval_factorial(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_factorial", |b| {
        b.iter(|| {
            parse_and_eval(black_box("10! + 5!"), false, &funcs, &consts)
        })
    });
}

fn bench_eval_trigonometric(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_trigonometric", |b| {
        b.iter(|| {
            parse_and_eval(
                black_box("sin(0.5) + cos(0.5) + tan(0.5)"),
                false,
                &funcs,
                &consts,
            )
        })
    });
}

fn bench_eval_logarithm(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_logarithm", |b| {
        b.iter(|| {
            parse_and_eval(
                black_box("lg(1000) + ln(C_E) + log2(256)"),
                false,
                &funcs,
                &consts,
            )
        })
    });
}

fn bench_eval_comparison(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_comparison", |b| {
        b.iter(|| {
            parse_and_eval(black_box("5 > 3 && 10 < 20"), false, &funcs, &consts)
        })
    });
}

// ==================== BigInt Benchmarks ====================

fn bench_eval_gcd(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_gcd", |b| {
        b.iter(|| {
            parse_and_eval(black_box("gcd(123456, 789012)"), false, &funcs, &consts)
        })
    });
}

fn bench_eval_lcm(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_lcm", |b| {
        b.iter(|| {
            parse_and_eval(black_box("lcm(123456, 789012)"), false, &funcs, &consts)
        })
    });
}

fn bench_eval_bfactorial(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_bfactorial_100", |b| {
        b.iter(|| {
            parse_and_eval(black_box("bfactorial(100)"), false, &funcs, &consts)
        })
    });
}

fn bench_eval_bpow(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_bpow", |b| {
        b.iter(|| {
            parse_and_eval(black_box("bpow(2, 100)"), false, &funcs, &consts)
        })
    });
}

fn bench_eval_isprime(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    
    c.bench_function("eval_isprime", |b| {
        b.iter(|| {
            parse_and_eval(black_box("isprime(997)"), false, &funcs, &consts)
        })
    });
}

// ==================== Parameterized Benchmarks ====================

fn bench_eval_power_param(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    let mut group = c.benchmark_group("eval_power");
    
    for exp in [10, 50, 100, 500] {
        group.bench_with_input(
            BenchmarkId::from_parameter(exp),
            &exp,
            |b, &exp| {
                b.iter(|| {
                    parse_and_eval(
                        &format!("2 ^ {}", exp),
                        false,
                        &funcs,
                        &consts,
                    )
                })
            },
        );
    }
    group.finish();
}

fn bench_eval_factorial_param(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    let mut group = c.benchmark_group("eval_factorial");
    
    for n in [5, 10, 20, 50] {
        group.bench_with_input(
            BenchmarkId::from_parameter(n),
            &n,
            |b, &n| {
                b.iter(|| {
                    parse_and_eval(
                        &format!("{}!", n),
                        false,
                        &funcs,
                        &consts,
                    )
                })
            },
        );
    }
    group.finish();
}

fn bench_eval_bfactorial_param(c: &mut Criterion) {
    let (funcs, consts) = empty_context();
    let mut group = c.benchmark_group("eval_bfactorial");
    
    for n in [50, 100, 500, 1000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(n),
            &n,
            |b, &n| {
                b.iter(|| {
                    parse_and_eval(
                        &format!("bfactorial({})", n),
                        false,
                        &funcs,
                        &consts,
                    )
                })
            },
        );
    }
    group.finish();
}

// ==================== Criterion Groups ====================

criterion_group!(
    parsing_benches,
    bench_parse_simple,
    bench_parse_complex,
    bench_parse_function_call,
    bench_parse_nested_functions,
    bench_parse_scientific_notation,
    bench_parse_hex_input,
);

criterion_group!(
    evaluation_benches,
    bench_eval_arithmetic,
    bench_eval_exponentiation,
    bench_eval_factorial,
    bench_eval_trigonometric,
    bench_eval_logarithm,
    bench_eval_comparison,
);

criterion_group!(
    bigint_benches,
    bench_eval_gcd,
    bench_eval_lcm,
    bench_eval_bfactorial,
    bench_eval_bpow,
    bench_eval_isprime,
);

criterion_group!(
    parametric_benches,
    bench_eval_power_param,
    bench_eval_factorial_param,
    bench_eval_bfactorial_param,
);

criterion_main!(
    parsing_benches,
    evaluation_benches,
    bigint_benches,
    parametric_benches,
);
