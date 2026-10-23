use std::cell::Cell;
use spindalis::integrals::{romberg_definite, IntegralError};
use spindalis::polynomials::{IntermediatePolynomial, PolynomialError, SimplePolynomial};
use spindalis::prelude::*;

// A scalar x^2 oracle with observable evaluations and an injected existing error.
// Other trait operations are not used by the quadrature API under test.
struct Probe {
    calls: Cell<usize>,
    fail_on: Option<usize>,
}
impl PolynomialTraits for Probe {
    fn parse(_: &str) -> Result<Self, PolynomialError> {
        Ok(Self { calls: Cell::new(0), fail_on: None })
    }
    fn eval_univariate<F>(&self, point: F) -> Result<f64, PolynomialError>
    where F: Into<f64> + Clone + std::fmt::Debug {
        let count = self.calls.get() + 1;
        self.calls.set(count);
        if self.fail_on == Some(count) {
            return Err(PolynomialError::VariableNotFound { variable: "injected".into() });
        }
        let x = point.into();
        Ok(x * x)
    }
    fn eval_multivariate<V, S, F>(&self, _: &V) -> Result<f64, PolynomialError>
    where V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
          S: AsRef<str>, F: Into<f64> { unreachable!() }
    fn derivate_univariate(&self) -> Result<Self, PolynomialError> { unreachable!() }
    fn derivate_multivariate<S: AsRef<str>>(&self, _: S) -> Self { unreachable!() }
    fn indefinite_integral_univariate(&self) -> Result<Self, PolynomialError> { unreachable!() }
    fn indefinite_integral_multivariate<S: AsRef<str>>(&self, _: S) -> Self { unreachable!() }
}

#[test]
fn zero_and_one_budget_preserve_evaluator_error_precedence() {
    for budget in [0, 1] {
        // Two initial endpoint samples and three first-refinement samples.
        for failure in 1..=5 {
            let p = Probe { calls: Cell::new(0), fail_on: Some(failure) };
            assert!(matches!(romberg_definite(&p, 0.0, 1.0, budget, 1e-6),
                Err(IntegralError::FunctionError(PolynomialError::VariableNotFound { .. }))));
            assert_eq!(p.calls.get(), failure);
        }
        let p = Probe::parse("").unwrap();
        assert!(matches!(romberg_definite(&p, 0.0, 1.0, budget, 1e-6),
            Err(IntegralError::MaxIterationsReached)));
        assert_eq!(p.calls.get(), 5);
    }
}

#[test]
fn final_permitted_refinement_is_still_budget_exhaustion() {
    let constant = SimplePolynomial::parse("5").unwrap();
    assert!(matches!(romberg_definite(&constant, 0.0, 1.0, 1, 1e-6),
        Err(IntegralError::MaxIterationsReached)));
    assert_eq!(romberg_definite(&constant, 0.0, 1.0, 2, 1e-6).unwrap(), 5.0);
    let quadratic = SimplePolynomial::parse("x^2").unwrap();
    assert!(matches!(romberg_definite(&quadratic, 0.0, 1.0, 2, 1e-6),
        Err(IntegralError::MaxIterationsReached)));
    assert!((romberg_definite(&quadratic, 0.0, 1.0, 3, 1e-6).unwrap() - 1.0 / 3.0).abs() < 1e-14);
    let zero = SimplePolynomial::parse("0").unwrap();
    assert!(matches!(romberg_definite(&zero, 0.0, 1.0, 2, 1e-6),
        Err(IntegralError::MaxIterationsReached)));
    assert_eq!(romberg_definite(&zero, 0.0, 1.0, 3, 1e-6).unwrap(), 0.0);
}

#[test]
fn evaluation_count_exposes_exponential_sampling_work() {
    for budget in [1, 2, 5, 10] {
        let p = Probe::parse("").unwrap();
        assert!(matches!(romberg_definite(&p, 0.0, 1.0, budget, -1.0),
            Err(IntegralError::MaxIterationsReached)));
        // Initial two samples plus sum(2^i + 1), i=1..budget.
        assert_eq!(p.calls.get(), (1_usize << (budget + 1)) + budget as usize);
    }
}

#[test]
fn scalar_integral_oracles_include_signed_bounds_and_false_zero_case() {
    for (input, start, end, exact) in [
        ("x^4", 0.0, 1.0, 1.0 / 5.0),
        ("x^4", 1.0, 0.0, -1.0 / 5.0),
        ("x^6-3x^5+3.25x^4-1.5x^3+0.25x^2", 0.0, 1.0, 1.0 / 840.0),
    ] {
        let p = SimplePolynomial::parse(input).unwrap();
        let actual = romberg_definite(&p, start, end, 10, 1e-8).unwrap();
        assert!((actual - exact).abs() < 1e-12, "{input}: {actual}, exact {exact}");
    }
}

#[test]
fn intermediate_constant_and_existing_variable_contracts() {
    let constant = IntermediatePolynomial::parse("-50").unwrap();
    assert_eq!(constant.eval_univariate(f64::NAN).unwrap(), -50.0);
    assert_eq!(constant.derivate_univariate().unwrap().eval_univariate(9.0).unwrap(), 0.0);
    let p = IntermediatePolynomial::parse("3x^2+2x+1").unwrap();
    assert_eq!(p.eval_univariate(3.0).unwrap(), 34.0);
    assert_eq!(p.derivate_univariate().unwrap().eval_univariate(3.0).unwrap(), 20.0);
    let multi = IntermediatePolynomial::parse("x+y").unwrap();
    assert!(matches!(multi.eval_univariate(3.0), Err(PolynomialError::TooManyVariables { .. })));
    assert!(matches!(multi.derivate_univariate(), Err(PolynomialError::TooManyVariables { .. })));
}

#[test]
fn formatter_retains_integer_digits_and_positive_precision_behavior() {
    let p = SimplePolynomial::parse("100x^2+20x+300").unwrap();
    assert_eq!(format!("{:.0}", p), "100x^2 + 20x + 300");
    assert_eq!(format!("{:.2}", p), "100x^2 + 20x + 300");
    let p = IntermediatePolynomial::parse("-100x^-10+20").unwrap();
    assert_eq!(format!("{:.0}", p), "-100x^-10 + 20");
    assert_eq!(format!("{:.2}", p), "-100x^-10 + 20");
}
