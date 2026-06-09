use crate::{
    digit::{Digit, DigitParseError},
    inward::InwardCodeParseError::{FailedToParseAsNum, UnexpectedLength},
};
use serde::Serialize;

const INWARDCODE_EXPECTED_LENGTH: usize = 3;

#[derive(Serialize, Debug, Clone)]

pub struct InwardCode {
    pub sector: Digit,
    pub unit: String,
}

impl InwardCode {
    pub fn new(s: &str) -> Result<Self, InwardCodeParseError> {
        if s.len() != 3 {
            return Err(UnexpectedLength(s.len(), s.to_string()));
        }

        let mut chars: Vec<char> = s.chars().collect();
        if chars[0] == 'O' {
            chars[0] = '0';
        } else if chars[0] == 'I' {
            chars[0] = '1';
        }

        let first_char = match chars[0].to_digit(10) {
            Some(s) => s,
            None => return Err(FailedToParseAsNum(chars[0])),
        } as u8;

        let sector = Digit::try_from(first_char)?;
        let unit: String = chars[1..].iter().collect();

        Ok(Self { sector, unit })
    }
}

crate::macros::impl_deserialize!(InwardCode);

#[derive(thiserror::Error, Debug)]
pub enum InwardCodeParseError {
    #[error("Unexpected inward code length: {0} (for inward-code: {1})length: expcted length of 3")]
    UnexpectedLength(usize, String),

    #[error("Failed to parse first character: {0} as a number.")]
    FailedToParseAsNum(char),

    #[error(transparent)]
    DigitParseError(#[from] DigitParseError),
}
