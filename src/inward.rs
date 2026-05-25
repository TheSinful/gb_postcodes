use crate::{
    digit::{Digit, DigitParseError},
    inward::InwardCodeParseError::{FailedToParseAsNum, UnexpectedLength},
};

const INWARDCODE_EXPECTED_LENGTH: usize = 3;

pub struct InwardCode {
    pub sector: Digit,
    pub unit: String,
}

impl InwardCode {
    pub fn new(s: &str) -> Result<Self, InwardCodeParseError> {
        if s.len() != 3 {
            return Err(UnexpectedLength(INWARDCODE_EXPECTED_LENGTH));
        }

        let chars: Vec<char> = s.chars().collect();
        let first_char = match chars[0].to_digit(10) {
            Some(s) => s,
            None => return Err(FailedToParseAsNum(chars[0])),
        } as u8;

        let sector = Digit::try_from(first_char)?;
        let unit: String = chars[1..].iter().collect();

        Ok(Self { sector, unit })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InwardCodeParseError {
    #[error("Unexpected length of string: {0}, expcted length of 3")]
    UnexpectedLength(usize),

    #[error("Failed to parse first character: {0} as a number.")]
    FailedToParseAsNum(char),

    #[error(transparent)]
    DigitParseError(#[from] DigitParseError),
}
