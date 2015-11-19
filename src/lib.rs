#[derive(Debug)]
pub enum ValidateError {
    InvalidMyNumberLength,
    InvalidType,
    InvalidCheckDigit
}

pub fn validate(mynumber: &str) -> Result<(), ValidateError> {
    if mynumber.len() != 12 {
        return Err(ValidateError::InvalidMyNumberLength);
    }

    let mut numbers = mynumber.chars().collect::<Vec<char>>();
    let check_digit = if numbers.last().unwrap().is_numeric() {
        numbers.pop().unwrap().to_digit(10).unwrap()
    } else {
        return Err(ValidateError::InvalidType)
    };

    let mut pq = 0;
    for (i, v) in numbers.iter().rev().enumerate() {
        if !v.is_numeric() {
            return Err(ValidateError::InvalidType)
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

    if check_digit != calc_digit {
        return Err(ValidateError::InvalidCheckDigit);
    }

    return Ok(());
}
