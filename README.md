# ModelForge

**From a polynomial expression to a numerical answer, in Rust.**

Use polynomial parsing and calculus alongside numerical solvers from one Rust namespace. ModelForge re-exports the retained Spindalis crates and provides macros that work through the `model_forge` import; it is a facade over those implementations.

[Examples](spindalis/examples/README.md) · [Verification](docs/VERIFICATION.md) · [Contributing](docs/CONTRIBUTING.md)

## Add to your project

```toml
[dependencies]
model_forge = { git = "https://github.com/nazeeh111/ModelForge.git" }
```

Use a Rust toolchain supporting edition 2024. This Git repository is the installation source; no crates.io publication is implied.

```rust
use model_forge::polynomials::{
    PolynomialError, PolynomialTraits, SimplePolynomial,
};

fn main() -> Result<(), PolynomialError> {
    let polynomial = SimplePolynomial::parse("3x^2+2x+1")?;
    let value = polynomial.eval_univariate(2.0)?;
    let derivative = polynomial.derivate_univariate()?;
    assert_eq!(value, 17.0);
    assert_eq!(
        derivative.coefficients,
        vec![2.0, 6.0]
    );
    Ok(())
}
```

The example carries one expression through three operations:

| Operation | Result |
| --- | --- |
| Parse `3x^2+2x+1` | Coefficients `[1, 2, 3]`, ordered from the constant term upward |
| Evaluate at `x = 2` | `17` |
| Differentiate | `6x + 2`, stored as `[2, 6]` |

The derivative is another polynomial, so it can be evaluated with the same API. These are floating-point operations; the assertions above check this particular example, not arbitrary numerical accuracy.

## What is inside

| Area | Tools |
| --- | --- |
| Expressions | Simple, intermediate, and advanced polynomial representations |
| Calculus | Univariate and partial derivatives; definite and indefinite integration |
| Solvers | Root finding, optimization, eigenvalue routines |
| Modeling | Regression and numerical reduction utilities |
| Compile-time parsing | Polynomial macros through `model_forge::polynomials` |

The `model_forge` crate provides the primary namespace. Existing `spindalis`, `spindalis_core`, and `spindalis_macros` crate APIs remain available for compatibility. The facade re-exports their operations. Targeted bisection and Newton convergence repairs are recorded in [verification](docs/VERIFICATION.md#solver-convergence-repairs-2026-10-02). New macros generate ModelForge paths so downstream consumers do not need a separate core dependency.

## Build and verify

```bash
cargo test --workspace
cargo build --workspace
```

Detailed examples remain in [the example guide](spindalis/examples/README.md). The [verification record](docs/VERIFICATION.md) explains tested behavior and remaining limitations. Some advanced expression capabilities are still evolving; test your application's input domain.

Maintained by [nazeeh111](https://github.com/nazeeh111).
