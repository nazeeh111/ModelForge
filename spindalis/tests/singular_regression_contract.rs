use spindalis::prelude::*;
use spindalis::regressors::PolynomialRegression;
use spindalis::solvers::{SolverError, gaussian_elimination};

#[test]
fn singular_dependent_rows_are_errors_for_both_rhs_cases() {
    for rhs in [[3.0, 6.0], [3.0, 7.0]] {
        assert!(matches!(gaussian_elimination(vec![vec![1.0, 2.0], vec![2.0, 4.0]], &rhs, 1e-12), Err(SolverError::SingularMatrix)));
    }
}

#[test]
fn zero_rows_and_zero_scalar_are_errors() {
    assert!(matches!(gaussian_elimination(vec![vec![0.0]], &[0.0], 1e-12), Err(SolverError::SingularMatrix)));
    for matrix in [[[0.0, 0.0], [1.0, 2.0]], [[1.0, 2.0], [0.0, 0.0]]] {
        assert!(matches!(gaussian_elimination(matrix.map(|row| row.to_vec()).to_vec(), &[0.0, 0.0], 1e-12), Err(SolverError::SingularMatrix)));
    }
}

#[test]
fn exact_zero_pivot_is_refused_even_at_zero_tolerance() {
    assert!(matches!(gaussian_elimination(vec![vec![1.0, 2.0], vec![2.0, 4.0]], &[3.0, 6.0], 0.0), Err(SolverError::SingularMatrix)));
}

#[test]
fn pivoted_success_has_the_known_solution_and_residual() {
    let matrix = [[0.0, 2.0], [1.0, 3.0]];
    let rhs = [4.0, 7.0];
    let solution = gaussian_elimination(matrix.map(|row| row.to_vec()).to_vec(), &rhs, 1e-12).unwrap();
    assert_eq!(solution, [1.0, 2.0]);
    for (row, expected) in matrix.iter().zip(rhs) {
        assert!((row.iter().zip(&solution).map(|(a, x)| a*x).sum::<f64>()-expected).abs() < 1e-12);
    }
}

#[test]
fn repeated_predictors_return_undefined_model_without_panicking() {
    for x in [[1.0; 4], [0.0; 4]] {
        let fitted = std::panic::catch_unwind(|| PolynomialRegression { order: 1 }.fit(&x, &[1.0, 2.0, 3.0, 4.0]));
        assert!(fitted.is_ok(), "singularity must not become an unwrap panic");
        let fitted = fitted.unwrap();
        assert_eq!(fitted.coefficients.len(), 2);
        assert!(fitted.coefficients.iter().all(|v| v.is_nan()));
        assert!(fitted.std_err.is_nan() && fitted.r2.is_nan());
        assert!(fitted.intercept().is_nan() && fitted.predict(2.0).is_nan());
    }
}

#[test]
fn insufficient_distinct_predictors_for_quadratic_are_undefined() {
    let fitted = PolynomialRegression { order: 2 }.fit(&[0.0, 1.0, 0.0, 1.0], &[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(fitted.coefficients.len(), 3);
    assert!(fitted.coefficients.iter().all(|v| v.is_nan()));
    assert!(fitted.std_err.is_nan() && fitted.r2.is_nan());
}

#[test]
fn successful_quadratic_fit_preserves_known_coefficients() {
    let x = [-2.0, -1.0, 0.0, 1.0, 2.0];
    let y = x.map(|x| 1.0 + 2.0*x + 3.0*x*x);
    let fitted = PolynomialRegression { order: 2 }.fit(&x, &y);
    for (actual, expected) in fitted.coefficients.iter().zip([1.0, 2.0, 3.0]) {
        assert!((actual-expected).abs() < 1e-12);
    }
    assert!(fitted.std_err.abs() < 1e-12);
    assert!((fitted.r2-1.0).abs() < 1e-12);
}
