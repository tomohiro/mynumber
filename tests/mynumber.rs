extern crate mynumber;

#[test]
fn verify_with_valid_mynumber_returns_ok() {
  let number = "123456789018";
  assert!(mynumber::verify(number).is_ok());
}
