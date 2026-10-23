use spindalis::derivatives::advanced_derivative;
use spindalis::polynomials::{Polynomial, PolynomialError};

fn value(expr: &str, variable: &str, u: f64, v: f64) -> f64 {
    let p = Polynomial::parse(expr).unwrap();
    advanced_derivative(&p, variable).unwrap()
        .eval_multivariate(&[("u", u), ("v", v)]).unwrap()
}
fn check(cases: Vec<(&str, f64)>, variable: &str, u: f64, v: f64) {
    let mut failures = Vec::new();
    for (expr, expected) in cases {
        let actual = value(expr, variable, u, v);
        if !actual.is_finite() || (actual - expected).abs() > 1e-10 * expected.abs().max(1.0) {
            failures.push(format!("d/d{variable} {expr} at u={u}, v={v}: {actual}; scalar expected {expected}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn named_constants_and_independent_functions() {
    for u in [0.25, 0.75, 1.5] {
        let expressions = ["pi", "e", "tau", "phi", "sin(v)", "cos(v)", "tan(v)",
            "cot(v)", "sec(v)", "csc(v)", "ln(v)", "log(v)", "sqrt(v)", "e^v"];
        check(expressions.into_iter().map(|s| (s, 0.0)).collect(), "u", u, 2.5);
    }
}

#[test]
fn chains_match_independent_positive_domain_scalar_formulas() {
    for u in [0.25_f64, 0.75, 1.5] {
        let v = 2.5_f64;
        check(vec![
            ("e^(3u)", 3.0 * (3.0 * u).exp()),
            ("e^(u^2)", 2.0 * u * (u*u).exp()),
            ("e^(sin(u))", u.cos() * u.sin().exp()),
            ("2^(3u)", 3.0 * 2.0_f64.ln() * 2.0_f64.powf(3.0*u)),
            ("pi^(u^2)", 2.0 * u * std::f64::consts::PI.ln() * std::f64::consts::PI.powf(u*u)),
            ("2^(sin(u))", 2.0_f64.ln() * u.cos() * 2.0_f64.powf(u.sin())),
            ("tau^(3u)", 3.0 * std::f64::consts::TAU.ln() * std::f64::consts::TAU.powf(3.0*u)),
            ("phi^(3u)", 3.0 * std::f64::consts::GOLDEN_RATIO.ln() * std::f64::consts::GOLDEN_RATIO.powf(3.0*u)),
        ], "u", u, v);
    }
}

#[test]
fn exponential_products_keep_both_product_terms() {
    for u in [0.25_f64, 0.75, 1.5] {
        let v = 2.5_f64;
        check(vec![
            ("u*e^u", (1.0 + u) * u.exp()),
            ("e^u*u", (1.0 + u) * u.exp()),
            ("sin(u)*e^(3u)", (3.0*u).exp() * (u.cos() + 3.0*u.sin())),
            ("e^u*e^(3u)", 4.0 * (4.0*u).exp()),
            ("pi*u", std::f64::consts::PI),
            ("e*sin(u)", std::f64::consts::E * u.cos()),
            ("v*e^(3u)", 3.0 * v * (3.0*u).exp()),
            ("sin(v)*e^u", v.sin() * u.exp()),
        ], "u", u, v);
    }
}

#[test]
fn selected_variable_is_not_hardcoded() {
    let u = 2.5_f64;
    for v in [0.25_f64, 0.75, 1.5] {
        check(vec![
            ("2^(3v)", 3.0 * 2.0_f64.ln() * 2.0_f64.powf(3.0*v)),
            ("sin(u)*e^v", u.sin() * v.exp()),
            ("v^2", 2.0 * v),
        ], "v", u, v);
    }
}

#[test]
fn parse_errors_and_required_result_variables_keep_their_contract() {
    assert!(Polynomial::parse("e^(").is_err());
    let p = Polynomial::parse("u*e^u").unwrap();
    let derivative = advanced_derivative(&p, "u").unwrap();
    assert!(matches!(derivative.eval_multivariate(&[("v", 2.5)]),
        Err(PolynomialError::VariableNotFound { variable }) if variable == "u"));
    let p = Polynomial::parse("3u^2+2u+1").unwrap();
    assert_eq!(advanced_derivative(&p, "u").unwrap()
        .eval_multivariate(&[("u", 3.0)]).unwrap(), 20.0);
}

#[test]
fn zero_parameter_base_preserves_existing_valid_values() {
    for u in [0.25, 0.5, 1.5] {
        check(vec![("v^(3u)", 0.0), ("0^(3u)", 0.0)], "u", u, 0.0);
    }
}
