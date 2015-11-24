//! A crate for My Number functions.
//!
//! # Example
//!
//! ```
//! extern crate mynumber;
//!
//! match mynumber::verify("123456789018") {
//!     Ok(()) => println!("valid"),
//!     Err(e) => println!("invalid: {:?}", e),
//! }
//! ```

#[derive(Debug)]
pub enum VerifyError {
    InvalidNumberLength,
    InvalidType,
    InvalidCheckDigit
}

pub mod individual;

pub fn verify(mynumber: &str) -> Result<(), VerifyError> {
    individual::verify(mynumber)
}
