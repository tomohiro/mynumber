extern crate mynumber;

#[test]
fn validate_with_short_mynumber_returns_error() {
  let number = "12345";
  assert!(mynumber::validate(number).is_err());
}

#[test]
fn validate_with_long_mynumber_returns_error() {
  let number = "12345678901234567890";
  assert!(mynumber::validate(number).is_err());
}

#[test]
fn validate_with_invalid_type_mynumber_returns_error() {
  let number = "ABCDEFGHIJKL";
  assert!(mynumber::validate(number).is_err());
}

#[test]
fn validate_with_valid_mynumber_returns_ok() {
  let number = "123456789018";
  assert!(mynumber::validate(number).is_ok());
}

#[test]
fn validate_with_wrong_check_digit_returns_err() {
  let number = "123456789010";
  assert!(mynumber::validate(number).is_err());
}

#[test]
fn validate_with_invalid_check_digit_returns_error() {
  let number = "12345678901A";
  assert!(mynumber::validate(number).is_err());
}
