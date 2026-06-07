use crate::{digit::Digit, inward::InwardCodeParseError, outward::OutwardCodeParseError};
use serde::{Deserialize, Serialize};
#[cfg(feature = "geo")]
use std::{collections::HashMap, sync::LazyLock};

pub use inward::InwardCode;
pub use outward::{OutwardCode, areas::PostCodeArea, district::PostCodeDistrict};

pub mod digit;
pub mod inward;
pub(crate) mod macros;
pub mod outward;
#[cfg(test)]
mod tests;

const POSTCODE_EXPECTED_MINIMUM_LENGTH: usize = 6;
const POSTCODE_EXPECTED_MAXIMUM_LENGTH: usize = 8;

#[derive(thiserror::Error, Debug)]
pub enum PostcodeParseError {
    #[error(
        "Postcode length was invalid, expected a string of 6-8 characters but got one with {0} characters"
    )]
    InvalidLength(usize),

    #[error("Postcode contained more than once space! found {0} spaces.")]
    InvalidSpaceCount(usize),

    #[error(transparent)]
    OutwardCodeParseError(#[from] OutwardCodeParseError),

    #[error(transparent)]
    InwardCodeParseError(#[from] InwardCodeParseError),
}

#[cfg(feature = "geo")]
static MAP: LazyLock<HashMap<String, (f64, f64)>> = LazyLock::new(|| {
    postcard::from_bytes(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/code_point_open_data.bin"
    )))
    .expect("should've found and deserialized downloaded code point data.")
});

#[cfg(feature = "geo")]
#[derive(Serialize, Deserialize)]
pub struct GeoLocation {
    pub easting: f64,
    pub northing: f64,
}

#[derive(Serialize, Deserialize)]
pub struct PostCode {
    pub inward_code: InwardCode,
    pub outward_code: OutwardCode,
    pub as_str: String,
    #[cfg(feature = "geo")]
    pub geo: GeoLocation, // provided in easting-northing format
}

impl PostCode {
    pub fn new(s: String) -> Result<Self, PostcodeParseError> {
        if s.len() < POSTCODE_EXPECTED_MINIMUM_LENGTH || s.len() > POSTCODE_EXPECTED_MAXIMUM_LENGTH
        {
            return Err(PostcodeParseError::InvalidLength(s.len()));
        }

        let split: Vec<&str> = s.split(" ").collect();
        if split.len() != 2 {
            return Err(PostcodeParseError::InvalidSpaceCount(split.len()));
        }

        let outward_code = OutwardCode::new(split[0])?;
        let inward_code = InwardCode::new(split[1])?;
        let as_str = s;
        #[cfg(feature = "geo")]
        let geo_tuple: &'static (f64, f64) = MAP
            .get(&as_str)
            .expect("should've found geo-location data for code from downloaded code-point data.");
        Ok(Self {
            inward_code,
            outward_code,
            #[cfg(feature = "geo")]
            geo: GeoLocation {
                easting: geo_tuple.0,
                northing: geo_tuple.1,
            },
            as_str,
        })
    }

    #[cfg(feature = "geo")]
    pub fn geo(&self) -> &GeoLocation {
        &self.geo
    }

    pub fn inward_code(&self) -> &InwardCode {
        &self.inward_code
    }

    pub fn outward_code(&self) -> &OutwardCode {
        &self.outward_code
    }

    pub fn sector(&self) -> Digit {
        self.inward_code.sector
    }

    pub fn unit(&self) -> &str {
        &self.inward_code.unit
    }

    pub fn area(&self) -> &PostCodeArea {
        &self.outward_code.area
    }

    pub fn district(&self) -> &PostCodeDistrict {
        &self.outward_code.district
    }

    pub fn as_str(&self) -> &str {
        &self.as_str
    }
}

impl PartialEq for PostCode {
    fn eq(&self, other: &Self) -> bool {
        self.as_str == other.as_str
    }
}
