//! A module to verify corporate number

/// Size of corporate number.
const CORPORATE_NUMBER_DIGITS: usize = 13;

/// Verifies the corporate number.
pub fn verify(number: &str) -> Result<(), ::VerifyError> {
    let mut digits = number.chars()
                           .filter_map(|x| {
                               if x.is_numeric() {
                                   x.to_digit(10)
                               } else {
                                   None
                               }
                           })
                           .collect::<Vec<u32>>();

    if digits.len() != CORPORATE_NUMBER_DIGITS {
        return Err(::VerifyError::InvalidNumberLength);
    }

    let check_digit = digits.remove(0);
    if check_digit == 0 {
        return Err(::VerifyError::InvalidCheckDigit);
    }

    let mut pq = 0;
    for (i, p) in digits.iter().enumerate() {
        let n = i + 1;
        let q = if n % 2 == 0 {
            1
        } else {
            2
        };
        pq += p * q;
    }

    let calc_digit = 9 - (pq % 9);
    if calc_digit == check_digit {
        Ok(())
    } else {
        Err(::VerifyError::InvalidCheckDigit)
    }
}
