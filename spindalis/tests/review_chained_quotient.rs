use spindalis::derivatives::advanced_derivative;
use spindalis::polynomials::Polynomial;

fn near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-11 * expected.abs().max(1.0),
        "actual={actual}, independent scalar expected={expected}");
}

#[test]
fn left_associated_division_matches_scalar_arithmetic() {
    for (x, y, z) in [(8.0, 2.0, 2.0), (-9.0, 3.0, -2.0), (1.5, -0.5, 4.0)] {
        for (text, expected) in [
            ("x/y/z", (x / y) / z),
            ("(x/y)/z", (x / y) / z),
            ("x/y/z/2", ((x / y) / z) / 2.0),
            ("3/x/y", (3.0 / x) / y),
        ] {
            let p = Polynomial::parse(text).unwrap();
            near(p.eval_multivariate(&[("x", x), ("y", y), ("z", z)]).unwrap(), expected);
        }
    }
}

#[test]
fn explicit_right_grouping_is_preserved() {
    let p = Polynomial::parse("x/(y/z)").unwrap();
    near(p.eval_multivariate(&[("x", 8.0), ("y", 2.0), ("z", 2.0)]).unwrap(), 8.0);
}

#[test]
fn chained_quotient_partials_match_independent_scalar_derivatives() {
    // f=(x+45)/((34+y)*y). The references below use ordinary scalar arithmetic,
    // without parsing the expected expression or comparing parser-created ASTs.
    let p = Polynomial::parse("(x+45)/(34+y)/y").unwrap();
    let dx = advanced_derivative(&p, "x").unwrap();
    let dy = advanced_derivative(&p, "y").unwrap();
    for x in [-3.0, 0.0, 4.0] {
        for y in [-6.0, -2.0, 0.5, 2.0, 8.0] {
            let variables = [("x", x), ("y", y)];
            near(dx.eval_multivariate(&variables).unwrap(), 1.0 / ((34.0 + y) * y));
            near(dy.eval_multivariate(&variables).unwrap(),
                -(x + 45.0) * (34.0 + 2.0 * y) / ((34.0 + y) * (34.0 + y) * y * y));
        }
    }
}

#[test]
fn existing_valid_polynomial_and_single_quotient_are_preserved() {
    for x in [-2.0, 0.5, 3.0] {
        let p = Polynomial::parse("3x^2+2x+1").unwrap();
        near(p.eval_multivariate(&[("x", x)]).unwrap(), 3.0*x*x+2.0*x+1.0);
        let d = advanced_derivative(&p, "x").unwrap();
        near(d.eval_multivariate(&[("x", x)]).unwrap(), 6.0*x+2.0);
        let p = Polynomial::parse("3/x").unwrap();
        near(p.eval_multivariate(&[("x", x)]).unwrap(), 3.0/x);
        let d = advanced_derivative(&p, "x").unwrap();
        near(d.eval_multivariate(&[("x", x)]).unwrap(), -3.0/(x*x));
    }
}

#[test]
fn missing_variable_and_invalid_syntax_remain_errors() {
    assert!(Polynomial::parse("x/y/").is_err());
    assert!(Polynomial::parse("x/y/z").unwrap().eval_multivariate(&[("x", 8.0), ("y", 2.0)]).is_err());
}
