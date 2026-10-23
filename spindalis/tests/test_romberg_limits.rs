use spindalis::prelude::*;
use spindalis::polynomials::SimplePolynomial;
use spindalis::integrals::{romberg_definite, IntegralError};

#[test]
fn romberg_zero_integrals_converge() {
    for (input, a, b) in [("0", 0.0, 1.0), ("x", -1.0, 1.0), ("x^2", 1.0, 1.0)] {
        let poly = SimplePolynomial::parse(input).unwrap();
        assert_eq!(romberg_definite(&poly, a, b, 10, 1e-6).unwrap(), 0.0);
        assert!(matches!(romberg_definite(&poly, a, b, 2, 1e-6), Err(IntegralError::MaxIterationsReached)));
        assert_eq!(romberg_definite(&poly, a, b, 3, 1e-6).unwrap(), 0.0);
    }
}

#[test]
fn romberg_exhaustion_above_original_table_limit() {
    let poly = SimplePolynomial::parse("x^2").unwrap();
    assert!(matches!(romberg_definite(&poly, 0.0, 1.0, 10, -1.0), Err(IntegralError::MaxIterationsReached)));
    assert!(matches!(romberg_definite(&poly, 0.0, 1.0, 0, 1e-6), Err(IntegralError::MaxIterationsReached)));
    assert!(matches!(romberg_definite(&poly, 0.0, 1.0, 1, 1e-6), Err(IntegralError::MaxIterationsReached)));
}

#[test]
fn romberg_refines_when_initial_nodes_are_roots() {
    // x^2 (x - 1/2)^2 (x - 1)^2, with exact integral 1/840 on [0, 1].
    let poly = SimplePolynomial::parse("x^6-3x^5+3.25x^4-1.5x^3+0.25x^2").unwrap();
    let actual = romberg_definite(&poly, 0.0, 1.0, 10, 1e-6).unwrap();
    assert!((actual - 1.0 / 840.0).abs() < 1e-12, "{actual}");
}
