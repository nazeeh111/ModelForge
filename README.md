# ModelForge

**Development history:** Developed locally before publication. These repositories were uploaded together, so their GitHub publication dates do not indicate when development began.

**From expressions to numerical answers.** A Rust toolkit for polynomial parsing, differentiation, integration, regression, and scientific solvers.

[Examples](spindalis/examples/README.md) · [Verification](docs/VERIFICATION.md) · [Contributing](docs/CONTRIBUTING.md)

## Add to your project

```toml
[dependencies]
model_forge = { git = "https://github.com/nazeeh111/ModelForge.git" }
```

Use a Rust toolchain supporting edition 2024. This Git repository is the installation source; no crates.io publication is implied.

```rust
use model_forge::polynomials::{PolynomialTraits, SimplePolynomial};

let polynomial = SimplePolynomial::parse("3x^2+2x+1")?;
let value = polynomial.eval_univariate(2.0)?;
let derivative = polynomial.derivate_univariate()?;
```

## What is inside

| Area | Tools |
| --- | --- |
| Expressions | Simple, intermediate, and advanced polynomial representations |
| Calculus | Univariate and partial derivatives; definite and indefinite integration |
| Solvers | Root finding, optimization, eigenvalue routines |
| Modeling | Regression and numerical reduction utilities |
| Compile-time parsing | Polynomial macros through `model_forge::polynomials` |

The `model_forge` crate provides the primary namespace. Existing `spindalis`, `spindalis_core`, and `spindalis_macros` crate APIs remain available for compatibility. Their numerical implementations are preserved; the new facade re-exports the same operations. New macros generate ModelForge paths so downstream consumers do not need a separate core dependency.

## Build and verify

```bash
cargo test --workspace
cargo build --workspace
```

Detailed examples remain in [the example guide](spindalis/examples/README.md). The [verification record](docs/VERIFICATION.md) explains tested behavior and remaining limitations. Some advanced expression capabilities are still evolving; test your application's input domain.

Maintained by [nazeeh111](https://github.com/nazeeh111).
