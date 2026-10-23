use spindalis::prelude::*;
use spindalis::polynomials::{IntermediatePolynomial, SimplePolynomial};

#[test]
fn precision_zero_preserves_integer_zeros() {
    let simple = SimplePolynomial::parse("10x+20").unwrap();
    assert_eq!(format!("{:.0}", simple), "10x + 20");
    assert_eq!(format!("{:.2}", simple), "10x + 20");
    let intermediate = IntermediatePolynomial::parse("10x^10+20").unwrap();
    assert_eq!(format!("{:.0}", intermediate), "10x^10 + 20");
    assert_eq!(format!("{:.2}", intermediate), "10x^10 + 20");
    let rounded = IntermediatePolynomial::parse("0.1x^0.1").unwrap();
    assert_eq!(format!("{:.0}", rounded), "0x^0");
}
