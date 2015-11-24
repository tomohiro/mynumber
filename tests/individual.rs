extern crate mynumber;

use mynumber::individual;

#[test]
fn verify_with_shorter_individual_number_returns_error() {
  let number = "12345";
  assert!(individual::verify(number).is_err());
}

#[test]
fn verify_with_longer_individual_number_returns_error() {
  let number = "12345678901234567890";
  assert!(individual::verify(number).is_err());
}

#[test]
fn verify_with_invalid_individual_number_type_returns_error() {
  let number = "ABCDEFGHIJKL";
  assert!(individual::verify(number).is_err());
}

#[test]
fn verify_with_valid_individual_number_returns_ok() {
  let number = "123456789018";
  assert!(individual::verify(number).is_ok());
}

#[test]
fn verify_with_wrong_check_digit_returns_err() {
  let number = "123456789010";
  assert!(individual::verify(number).is_err());
}

#[test]
fn verify_with_empty_number_returns_error() {
  let number = "";
  assert!(individual::verify(number).is_err());
}
