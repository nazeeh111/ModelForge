use spindalis::prelude::*;
use spindalis::polynomials::IntermediatePolynomial;

#[test]
fn constant_evaluation_and_derivative() {
    for (input, expected) in [("5", 5.0), ("0", 0.0), ("2+3", 5.0), ("-7", -7.0)] {
        let poly = IntermediatePolynomial::parse(input).unwrap();
        assert_eq!(poly.eval_univariate(2.0).unwrap(), expected);
        let derivative = poly.derivate_univariate().unwrap();
        assert_eq!(derivative.eval_univariate(2.0).unwrap(), 0.0);
    }
}
