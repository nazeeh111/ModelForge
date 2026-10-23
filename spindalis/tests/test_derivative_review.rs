use spindalis::derivatives::advanced_derivative;
use spindalis::polynomials::Polynomial;

fn check(cases: &[(&str, f64)]) {
    let mut failures = Vec::new();
    for &(expr, expected) in cases {
        let p = Polynomial::parse(expr).unwrap();
        let derivative = advanced_derivative(&p, "x").unwrap();
        let actual = derivative.eval_multivariate(&[("x", 2.0), ("y", 3.0)]).unwrap();
        if !actual.is_finite() || (actual - expected).abs() > 1e-10 * expected.abs().max(1.0) {
            failures.push(format!("d/dx {expr}: {derivative} gives {actual}, expected {expected}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn derivative_of_constants_and_independent_functions_is_zero() {
    check(&[("pi", 0.0), ("e", 0.0), ("sin(y)", 0.0), ("ln(y)", 0.0), ("sqrt(y)", 0.0)]);
}

#[test]
fn exponential_chain_rule_uses_inner_derivative() {
    let e6 = 6.0_f64.exp();
    check(&[("e^(3x)", 3.0 * e6), ("2^(3x)", 3.0 * 2.0_f64.ln() * 64.0), ("e^(x^2)", 4.0 * 4.0_f64.exp())]);
}

#[test]
fn exponential_products_use_full_product_rule() {
    let e2 = 2.0_f64.exp();
    check(&[("x*e^x", 3.0 * e2), ("e^x*x", 3.0 * e2), ("e^x*sin(x)", e2 * (2.0_f64.sin() + 2.0_f64.cos()))]);
}
