const OUTWARD_LENGTH_LOWER: usize = 2;
const OUTWARD_LENGTH_UPPER: usize = 4;

use crate::outward::district::{DistrictParseError, PostCodeDistrict};
use areas::{AreaParseError, PostCodeArea};
use std::fmt::Display;

pub mod areas;
pub mod district;

pub struct OutwardCode {
    pub area: PostCodeArea,
    pub district: PostCodeDistrict,
}

#[derive(thiserror::Error, Debug)]
pub enum OutwardCodeParseError {
    #[error("Expected a string of length within the range [2,4], but was given one of length {0}")]
    InvalidLength(usize),

    #[error(transparent)]
    AreaParseError(#[from] AreaParseError),

    #[error(transparent)]
    DistrictParseError(#[from] DistrictParseError),
}

impl OutwardCode {
    pub fn new(s: &str) -> Result<Self, OutwardCodeParseError> {
        if s.len() < OUTWARD_LENGTH_LOWER || s.len() > OUTWARD_LENGTH_UPPER {
            return Err(OutwardCodeParseError::InvalidLength(s.len()));
        }

        let chars = s.chars().collect::<Vec<char>>();

        let raw_area = &chars[..2];
        let is_single_char_area = raw_area[1].is_numeric();
        let area = if is_single_char_area {
            PostCodeArea::new(&raw_area[0].to_string())?
        } else {
            let first_two_chars: String = raw_area[..2].iter().collect();
            PostCodeArea::new(&first_two_chars)?
        };

        let district = if is_single_char_area {
            PostCodeDistrict::new(&chars[1..].iter().collect::<String>())?
        } else {
            PostCodeDistrict::new(&chars[2..].iter().collect::<String>())?
        };

        Ok(OutwardCode { area, district })
    }
}
