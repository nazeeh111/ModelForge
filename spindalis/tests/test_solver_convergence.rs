use spindalis::polynomials::{IntermediatePolynomial, PolynomialTraits, SimplePolynomial};
use spindalis::solvers::{Bounds, SolveMode, SolverError, bisection, newton_raphson_method};

fn midpoint_guess_converges<P: PolynomialTraits>(polynomial: &P) {
    let root = bisection(
        polynomial,
        Bounds {
            lower: 0.0,
            init: 1.0,
            upper: 2.0,
        },
        1e-5,
        100,
        SolveMode::Root,
    )
    .expect("a midpoint initial guess must not end bisection before narrowing the bracket");
    assert!((root - std::f64::consts::SQRT_2).abs() < 1e-5);
}

#[test]
fn bisection_midpoint_guess_simple() {
    midpoint_guess_converges(&SimplePolynomial::parse("x^2 - 2").unwrap());
}

#[test]
fn bisection_midpoint_guess_intermediate() {
    midpoint_guess_converges(&IntermediatePolynomial::parse("x^2 - 2").unwrap());
}

#[test]
fn newton_zero_root_simple_on_last_iteration() {
    let polynomial = SimplePolynomial::parse("x").unwrap();
    let root = newton_raphson_method(&polynomial, 1.0, 1, 1e-5, SolveMode::Root)
        .expect("an exact zero root reached on the last iteration is a solution");
    assert_eq!(root, 0.0);
}

#[test]
fn newton_zero_root_intermediate() {
    let polynomial = IntermediatePolynomial::parse("x").unwrap();
    let root = newton_raphson_method(&polynomial, 1.0, 100, 1e-5, SolveMode::Root)
        .expect("an exact zero root must not retain the initial relative error");
    assert_eq!(root, 0.0);
}

#[test]
fn newton_zero_extremum() {
    let polynomial = SimplePolynomial::parse("x^2").unwrap();
    let extremum = newton_raphson_method(&polynomial, 1.0, 100, 1e-5, SolveMode::Extrema)
        .expect("an exact zero derivative identifies the extremum at zero");
    assert_eq!(extremum, 0.0);
}

#[test]
fn newton_initial_zero_root_with_zero_derivative() {
    let polynomial = SimplePolynomial::parse("x^2").unwrap();
    let root = newton_raphson_method(&polynomial, 0.0, 0, 1e-5, SolveMode::Root)
        .expect("an exact initial root must be recognized before dividing by its derivative");
    assert_eq!(root, 0.0);
}

#[test]
fn newton_zero_iteration_budget_does_not_take_a_step() {
    let polynomial = SimplePolynomial::parse("x").unwrap();
    let result = newton_raphson_method(&polynomial, 1.0, 0, 1e-5, SolveMode::Root);
    assert!(matches!(result, Err(SolverError::MaxIterationsReached)));
}
