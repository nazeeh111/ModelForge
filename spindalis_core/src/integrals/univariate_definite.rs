use crate::integrals::IntegralError;
use crate::polynomials::structs::PolynomialTraits;

pub fn definite_integral<P>(
    poly: &P,
    start: f64,
    end: f64,
    segments: usize,
) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let segment_width = (end - start) / segments as f64;
    let mut sum = 0.0;
    if segments == 1 {
        let res = trapezoidal_rule(poly, start, end, segments)?;
        return Ok(res);
    }
    let mut remaining_segments = segments;
    if !segments.is_multiple_of(2) {
        // Last four points encapsulate last three segments
        let mut points = [0.0; 4];
        for (i, point) in points.iter_mut().enumerate().skip(1) {
            *point = end - segment_width * i as f64;
        }
        points[0] = end;
        points.reverse();
        sum += simpson38(segment_width, poly, points)?;
        remaining_segments -= 3;
    }
    if remaining_segments > 1 {
        sum += simpson13(segment_width, poly, start, remaining_segments)?;
    }
    Ok(sum)
}

/// Integrate by successively refining the trapezoidal rule and extrapolating.
///
/// `tolerance` is a relative percentage. Reaching `maxiter` returns
/// `MaxIterationsReached`, even if that final refinement meets the tolerance.
/// For compatibility, budgets of zero and one still evaluate the initial
/// trapezoid and first refinement; evaluator errors take precedence over the
/// budget error. Equal-zero estimates require a confirming refinement.
///
/// Through refinement `m`, this implementation makes `2^(m + 1) + m` integrand
/// evaluations, uses O(m) row storage, and performs O(m^2) extrapolation work.
/// A large iteration limit can therefore require exponentially many samples;
/// checked segment multiplication prevents integer overflow, not excessive work.
/// Finite sampled agreement does not guarantee accuracy between sampled points.
pub fn romberg_definite<P>(
    poly: &P,
    start: f64,
    end: f64,
    maxiter: u32,
    tolerance: f64,
) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let mut previous = vec![trapezoidal_rule(poly, start, end, 1)?];
    let mut segments = 1_usize;
    let mut previous_zero_estimate = false;
    for iter in 1..=maxiter.max(1) {
        segments = segments.checked_mul(2)
            .ok_or(IntegralError::MaxIterationsReached)?;
        let mut current = vec![trapezoidal_rule(poly, start, end, segments)?];
        let mut factor = 4.0;
        for (order, prior) in previous.iter().enumerate() {
            current.push((factor * current[order] - prior) / (factor - 1.0));
            factor *= 4.0;
        }
        let estimate = current[current.len() - 1];
        let difference = (estimate - current[current.len() - 2]).abs();
        // Equal finite estimates include zero integrals, for which relative error is 0/0.
        let approx_err = if difference == 0.0 && estimate.is_finite() {
            0.0
        } else {
            difference / estimate.abs() * 100.0
        };
        // Preserve the existing iteration-budget failure contract.
        if iter >= maxiter {
            return Err(IntegralError::MaxIterationsReached);
        }
        // Confirm an equal-zero estimate with another refinement: the initial nodes
        // can all be roots of a nonzero polynomial.
        let zero_estimate = estimate == 0.0 && difference == 0.0;
        if approx_err <= tolerance && (!zero_estimate || previous_zero_estimate) {
            return Ok(estimate);
        }
        previous_zero_estimate = zero_estimate;
        previous = current;
    }
    Err(IntegralError::MaxIterationsReached)
}

fn trapezoidal_rule<P>(
    poly: &P,
    start: f64,
    end: f64,
    segments: usize,
) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let mut xi = start;
    let segment_width = (end - start) / segments as f64;
    let mut sum = poly.eval_univariate(xi)?;
    for _ in 1..segments {
        xi += segment_width;
        sum += 2_f64 * poly.eval_univariate(xi)?;
    }
    sum += poly.eval_univariate(end)?;

    Ok(segment_width * sum / 2_f64)
}

// Handles an odd number of segments
fn simpson38<P>(segment_width: f64, poly: &P, points: [f64; 4]) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let mut f: Vec<f64> = Vec::new();
    for point in points {
        let x = poly.eval_univariate(point)?;
        f.push(x)
    }
    Ok(3_f64 * segment_width * (f[0] + 3_f64 * f[1] + 3_f64 * f[2] + f[3]) / 8_f64)
}

// Handles an even number of segments
fn simpson13<P>(
    segment_width: f64,
    poly: &P,
    start: f64,
    segments: usize,
) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let mut xi = start;
    let mut sum = poly.eval_univariate(xi)?;
    for _ in 1..segments / 2 {
        xi += 2_f64 * segment_width;
        sum +=
            4_f64 * poly.eval_univariate(xi - segment_width)? + 2_f64 * poly.eval_univariate(xi)?;
    }
    xi += 2_f64 * segment_width;
    sum += 4_f64 * poly.eval_univariate(xi - segment_width)? + poly.eval_univariate(xi)?;

    Ok(segment_width * sum / 3_f64)
}

pub fn analytical_integral<P>(poly: &P, a: f64, b: f64) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    // ∫ₐᵇ x dx = F(b) − F(a)

    let integrated_polynomial = poly.indefinite_integral_univariate()?;
    let fa = integrated_polynomial.eval_univariate(a)?;
    let fb = integrated_polynomial.eval_univariate(b)?;

    Ok(fb - fa)
}
