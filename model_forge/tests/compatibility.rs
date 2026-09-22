use model_forge::polynomials::{PolynomialTraits, SimplePolynomial};

#[test]
fn facade_preserves_polynomial_values_and_derivatives() {
    let branded = SimplePolynomial::parse("3x^2+2x+1").unwrap();
    let legacy = spindalis::polynomials::SimplePolynomial::parse("3x^2+2x+1").unwrap();
    for x in [-3.0_f64, -0.5, 0.0, 1.0, 4.25] {
        assert_eq!(
            branded.eval_univariate(x).unwrap().to_bits(),
            legacy.eval_univariate(x).unwrap().to_bits()
        );
    }
    assert_eq!(
        branded.derivate_univariate().unwrap(),
        legacy.derivate_univariate().unwrap()
    );
}

#[test]
fn branded_macro_works_without_direct_core_dependency() {
    let polynomial = model_forge::polynomials::parse_simple_polynomial!(3 x^2 + 2 x + 1);
    assert_eq!(polynomial, SimplePolynomial::parse("3x^2+2x+1").unwrap());
}

#[test]
fn branded_intermediate_macro_matches_runtime_parser() {
    let polynomial = model_forge::polynomials::parse_intermediate_polynomial!(3 x^2 + 2 y);
    assert_eq!(
        polynomial,
        model_forge::polynomials::IntermediatePolynomial::parse("3x^2+2y").unwrap()
    );
}
