const INDIVIDUAL_NUMBER_DIGITS: usize = 12;

/// Verifies the individual number.
pub fn verify(number: &str) -> Result<(), ::VerifyError> {
    if number.len() != INDIVIDUAL_NUMBER_DIGITS {
        return Err(::VerifyError::InvalidNumberLength);
    }

    let mut numbers = number.chars().collect::<Vec<char>>();
    let check_digit = if numbers.last().unwrap().is_numeric() {
        numbers.pop().unwrap().to_digit(10).unwrap()
    } else {
        return Err(::VerifyError::InvalidType)
    };


    let mut pq = 0;
    for (i, v) in numbers.iter().rev().enumerate() {
        if !v.is_numeric() {
            return Err(::VerifyError::InvalidType)
        }
        let n = i + 1;
        let p = v.to_digit(10).unwrap();
        let q = if n <= 6 { n + 1 } else { n - 5 } as u32;
        pq += p * q;
    };

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
