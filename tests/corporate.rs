extern crate mynumber;

use mynumber::corporate;

#[test]
fn verify_with_shorter_corporate_number_returns_error() {
    let number = "12345";
    assert!(corporate::verify(number).is_err());
}

#[test]
fn verify_with_longer_corporate_number_returns_error() {
    let number = "12345678901234567890";
    assert!(corporate::verify(number).is_err());
}

#[test]
fn verify_with_invalid_corporate_number_type_returns_error() {
    let number = "ABCDEFGHIJKLM";
    assert!(corporate::verify(number).is_err());
}

#[test]
fn verify_with_valid_corporate_number_returns_ok() {
    let number = "9234567890123";
    assert!(corporate::verify(number).is_ok());
}

#[test]
fn verify_with_wrong_check_digit_returns_err() {
    let number = "1234567890123";
    assert!(corporate::verify(number).is_err());
}

#[test]
fn verify_with_empty_number_returns_error() {
    let number = "";
    assert!(corporate::verify(number).is_err());
}
