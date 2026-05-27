use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum DigitParseError {
    #[error("Attempted to construct digit from a number: {0} out of bounds.")]
    OutOfBounds(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Digit(u8);

impl TryFrom<u8> for Digit {
    type Error = DigitParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 9 {
            Err(DigitParseError::OutOfBounds(value))
        } else {
            Ok(Digit(value))
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
