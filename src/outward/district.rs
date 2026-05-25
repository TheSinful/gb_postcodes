use crate::digit::{Digit, DigitParseError};
use std::fmt::Display;

const DISTRICT_LENGTH_LOWER: usize = 1;
const DISTRICT_UPPER_LOWER: usize = 2;

pub enum PostCodeDistrict {
    Normal(u32),
    Irregular(Digit, char),
}

#[derive(thiserror::Error, Debug)]
pub enum DistrictParseError {
    #[error("Expected district length of 1-2 characters, but got {0}")]
    InvalidLength(usize),

    #[error("Failed to parse character: {0} as a number!")]
    FirstCharNumParseError(char),

    #[error("Failed to parse string: {0} as a number!")]
    StrParseError(String),

    #[error(transparent)]
    DigitParseError(#[from] DigitParseError),
}

impl PostCodeDistrict {
    pub fn new(s: &str) -> Result<Self, DistrictParseError> {
        if s.len() < DISTRICT_LENGTH_LOWER || s.len() > DISTRICT_UPPER_LOWER {
            return Err(DistrictParseError::InvalidLength(s.len()));
        }

        let chars: Vec<char> = s.chars().collect();

        let first_char = match chars[0].to_digit(10) {
            Some(s) => s,
            None => return Err(DistrictParseError::FirstCharNumParseError(chars[0])),
        };

        if chars.len() == 1 {
            return Ok(PostCodeDistrict::Normal(first_char));
        }

        if chars[1].is_alphabetic() {
            return Ok(PostCodeDistrict::Irregular(
                Digit::try_from(first_char as u8)?,
                chars[1],
            ));
        } else {
            let range_as_str = chars[..2].iter().collect::<String>();
            let range_as_num = range_as_str
                .parse::<u32>()
                .map_err(|_| DistrictParseError::StrParseError(range_as_str))?;

            return Ok(PostCodeDistrict::Normal(range_as_num));
        }
    }
}

impl Display for PostCodeDistrict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostCodeDistrict::Normal(n) => write!(f, "{}", n),
            PostCodeDistrict::Irregular(n, c) => write!(f, "{}{}", n, c),
        }
    }
}
