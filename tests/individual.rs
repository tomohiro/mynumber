extern crate mynumber;

use mynumber::individual;

#[test]
fn verify_with_short_individual_returns_error() {
  let number = "12345";
  assert!(individual::verify(number).is_err());
}

#[test]
fn verify_with_long_individual_returns_error() {
  let number = "12345678901234567890";
  assert!(individual::verify(number).is_err());
}

#[test]
fn verify_with_invalid_type_individual_returns_error() {
  let number = "ABCDEFGHIJKL";
  assert!(individual::verify(number).is_err());
}

#[test]
fn verify_with_valid_individual_returns_ok() {
  let number = "123456789018";
  assert!(individual::verify(number).is_ok());
}

#[test]
fn verify_with_wrong_check_digit_returns_err() {
  let number = "123456789010";
  assert!(individual::verify(number).is_err());
}

#[test]
fn verify_with_invalid_check_digit_returns_error() {
  let number = "12345678901A";
  assert!(individual::verify(number).is_err());
}
