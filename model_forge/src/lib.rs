//! Numerical modeling, polynomial calculus, and scientific solvers.
//!
//! The original crate namespaces remain available for existing applications.
pub use spindalis::*;

/// Polynomial types, parsers, and macros using the ModelForge namespace.
pub mod polynomials {
    pub use model_forge_macros as macros;
    pub use model_forge_macros::{parse_intermediate_polynomial, parse_simple_polynomial};
    pub use spindalis::polynomials::*;
}
