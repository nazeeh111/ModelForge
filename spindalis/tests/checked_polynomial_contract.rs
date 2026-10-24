use spindalis::prelude::*;
use spindalis::regressors::{LinearRegressorError, PolynomialFitError, PolynomialRegression};
use spindalis::solvers::SolverError;

#[test]
fn checked_singular_recovery_retains_solver_reason() {
    for x in [[1.0; 4], [0.0; 4], [0.0, 1.0, 0.0, 1.0]] {
        assert!(matches!(
            PolynomialRegression { order: 2 }.try_fit(&x, &[1.0, 2.0, 3.0, 4.0]),
            Err(PolynomialFitError::NormalSystem(SolverError::SingularMatrix))
        ));
    }
}

#[test]
fn input_validation_precedes_solve_and_preserves_existing_error_order() {
    let fit = PolynomialRegression { order: 1 };
    for (x, y) in [(&[][..], &[1.0][..]), (&[1.0, 1.0][..], &[1.0][..])] {
        match fit.try_fit(x, y) {
            Err(PolynomialFitError::Input(LinearRegressorError::InputLengthMismatch {
                x_length, y_length,
            })) => assert_eq!((x_length, y_length), (x.len(), y.len())),
            other => panic!("Expected input-length error, got {:?}", other.err()),
        }
    }
    assert!(matches!(
        PolynomialRegression { order: usize::MAX }.try_fit(&[], &[]),
        Err(PolynomialFitError::Input(LinearRegressorError::EmptyInput {
            x_length: 0, y_length: 0,
        }))
    ));
}

#[test]
fn constant_response_is_success_with_undefined_r_squared() {
    let model = PolynomialRegression { order: 0 }
        .try_fit(&[0.0, 1.0, 2.0, 3.0], &[4.0; 4]).unwrap();
    assert_eq!(model.coefficients, [4.0]);
    assert_eq!(model.std_err, 0.0);
    assert!(model.r2.is_nan());
}

#[test]
fn legacy_invalid_input_is_not_given_the_new_input_validator() {
    let fit = PolynomialRegression { order: 0 };
    assert!(matches!(fit.try_fit(&[], &[1.0]), Err(PolynomialFitError::Input(_))));
    let legacy = fit.fit(&[], &[1.0]);
    assert_eq!(legacy.coefficients.len(), 1);
    assert!(legacy.coefficients[0].is_nan());
    assert!(legacy.std_err.is_nan() && legacy.r2.is_nan());
    // This mismatched input still follows the old unchecked arithmetic path.
    let legacy = fit.fit(&[0.0, 1.0], &[1.0]);
    assert_eq!(legacy.coefficients, [0.5]);
    assert!(legacy.std_err.is_nan());
    assert_eq!(legacy.r2, f64::NEG_INFINITY);
}

#[test]
fn unsupported_legacy_order_still_panics_instead_of_becoming_a_nan_model() {
    assert!(std::panic::catch_unwind(|| {
        PolynomialRegression { order: usize::MAX }.fit(&[1.0], &[1.0])
    }).is_err());
}

#[test]
fn checked_and_legacy_success_are_bit_identical_for_four_orders() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let y = [1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 8.0, 10.0, 13.0];
    for order in 0..=3 {
        let fit = PolynomialRegression { order };
        let checked = fit.try_fit(&x, &y).unwrap();
        let legacy = fit.fit(&x, &y);
        assert_eq!(checked.coefficients.len(), order + 1);
        assert_eq!(checked.coefficients.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                   legacy.coefficients.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
        assert_eq!(checked.std_err.to_bits(), legacy.std_err.to_bits());
        assert_eq!(checked.r2.to_bits(), legacy.r2.to_bits());
    }
}
