//! A module to verify individual number

/// Size of individual number.
const INDIVIDUAL_NUMBER_DIGITS: usize = 12;

/// Verifies the individual number.
pub fn verify(number: &str) -> Result<(), ::VerifyError> {
    let mut digits = number.chars().filter_map(|x|
        if x.is_numeric() {
            x.to_digit(10)
        } else {
            None
        }
    ).collect::<Vec<u32>>();

    if digits.len() != INDIVIDUAL_NUMBER_DIGITS {
        return Err(::VerifyError::InvalidNumberLength);
    }

    let check_digit = digits.pop().unwrap();
    let mut pq = 0;
    for (i, p) in digits.iter().rev().enumerate() {
        let n = i + 1;
        let q = if n <= 6 { n + 1 } else { n - 5 } as u32;
        pq += p * q;
    }

    let remainder = pq % 11;
    let calc_digit = match remainder {
        0 | 1 => 0,
        n     => 11 - n
    };

    if calc_digit == check_digit {
        Ok(())
    } else {
        Err(::VerifyError::InvalidCheckDigit)
    }
}
