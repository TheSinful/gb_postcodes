use super::*;
use outward::areas::PostCodeArea;

mod area_parsing {
    use super::*;

    #[test]
    fn parses_single_char_areas() {
        assert_eq!(PostCodeArea::new("B").unwrap(), PostCodeArea::Birmingham);
        assert_eq!(PostCodeArea::new("E").unwrap(), PostCodeArea::LondonE);
        assert_eq!(PostCodeArea::new("G").unwrap(), PostCodeArea::Glasgow);
        assert_eq!(PostCodeArea::new("L").unwrap(), PostCodeArea::Liverpool);
        assert_eq!(PostCodeArea::new("M").unwrap(), PostCodeArea::Manchester);
        assert_eq!(PostCodeArea::new("N").unwrap(), PostCodeArea::LondonN);
        assert_eq!(PostCodeArea::new("S").unwrap(), PostCodeArea::Sheffield);
        assert_eq!(PostCodeArea::new("W").unwrap(), PostCodeArea::LondonW);
    }

    #[test]
    fn parses_two_char_areas() {
        assert_eq!(PostCodeArea::new("SW").unwrap(), PostCodeArea::LondonSW);
        assert_eq!(PostCodeArea::new("EC").unwrap(), PostCodeArea::LondonEC);
        assert_eq!(PostCodeArea::new("NW").unwrap(), PostCodeArea::LondonNW);
        assert_eq!(PostCodeArea::new("WC").unwrap(), PostCodeArea::LondonWC);
        assert_eq!(PostCodeArea::new("AB").unwrap(), PostCodeArea::Aberdeen);
        assert_eq!(PostCodeArea::new("EH").unwrap(), PostCodeArea::Edinburgh);
    }

    #[test]
    fn parses_area_case_insensitively() {
        assert_eq!(PostCodeArea::new("sw").unwrap(), PostCodeArea::LondonSW);
        assert_eq!(PostCodeArea::new("Sw").unwrap(), PostCodeArea::LondonSW);
        assert_eq!(PostCodeArea::new("b").unwrap(), PostCodeArea::Birmingham);
        assert_eq!(PostCodeArea::new("eh").unwrap(), PostCodeArea::Edinburgh);
    }

    #[test]
    fn rejects_unknown_area_code() {
        assert!(PostCodeArea::new("ZZ").is_err());
        assert!(PostCodeArea::new("XX").is_err());
        assert!(PostCodeArea::new("QQ").is_err());
    }

    #[test]
    fn rejects_empty_area_code() {
        assert!(PostCodeArea::new("").is_err());
    }

    #[test]
    fn rejects_numeric_area_code() {
        assert!(PostCodeArea::new("12").is_err());
        assert!(PostCodeArea::new("1").is_err());
    }
}

mod district_parsing {
    use crate::digit::Digit;
    use crate::outward::district::DistrictParseError;
    use crate::outward::district::PostCodeDistrict;

    #[test]
    fn parses_single_digit_district() {
        let district = PostCodeDistrict::new("1").unwrap();
        assert!(matches!(district, PostCodeDistrict::Normal(1)));
    }

    #[test]
    fn parses_two_digit_district() {
        let district = PostCodeDistrict::new("12").unwrap();
        assert!(
            matches!(district, PostCodeDistrict::Normal(12)),
            "given {}",
            district
        );
    }

    #[test]
    fn parses_irregular_district_with_letter_suffix() {
        let district = PostCodeDistrict::new("1A").unwrap();
        let digit = Digit::try_from(1).unwrap();
        assert!(matches!(district, PostCodeDistrict::Irregular(digit, 'A')));
    }

    #[test]
    fn rejects_empty_district() {
        let result = PostCodeDistrict::new("");
        assert!(matches!(result, Err(DistrictParseError::InvalidLength(0))));
    }

    #[test]
    fn rejects_district_longer_than_two_chars() {
        let result = PostCodeDistrict::new("123");
        assert!(matches!(result, Err(DistrictParseError::InvalidLength(3))));
    }

    #[test]
    fn rejects_non_numeric_first_char() {
        let result = PostCodeDistrict::new("A1");
        assert!(matches!(
            result,
            Err(DistrictParseError::FirstCharNumParseError(_))
        ));
    }
}

mod outward_code_parsing {
    use super::*;
    use crate::digit::Digit;
    use crate::outward::OutwardCodeParseError;
    use crate::outward::district::PostCodeDistrict;

    #[test]
    fn parses_two_char_area_single_digit_district() {
        // e.g. SW1 1AA
        let outward = outward::OutwardCode::new("SW1").unwrap();
        assert_eq!(outward.area, PostCodeArea::LondonSW);
        assert!(matches!(outward.district, PostCodeDistrict::Normal(1)));
    }

    #[test]
    fn parses_single_char_area_single_digit_district() {
        // e.g. B1 1AA
        let outward = outward::OutwardCode::new("B1").unwrap();
        assert_eq!(outward.area, PostCodeArea::Birmingham);
        assert!(matches!(outward.district, PostCodeDistrict::Normal(1)));
    }

    #[test]
    fn parses_two_char_area_two_digit_district() {
        // e.g. AB10 1AA
        let outward = outward::OutwardCode::new("AB10").unwrap();
        assert_eq!(outward.area, PostCodeArea::Aberdeen);
        assert!(
            matches!(outward.district, PostCodeDistrict::Normal(10)),
            "given {}",
            outward.district
        );
    }

    #[test]
    fn parses_two_char_area_irregular_district() {
        // e.g. EC1A 1BB
        let outward = outward::OutwardCode::new("EC1A").unwrap();
        let digit = Digit::try_from(1).unwrap();

        assert_eq!(outward.area, PostCodeArea::LondonEC);
        assert!(matches!(
            outward.district,
            PostCodeDistrict::Irregular(digit, 'A')
        ));
    }

    #[test]
    fn rejects_postcode_below_minimum_length() {
        // Fewer than 6 characters
        let result = outward::OutwardCode::new("B1 1A");
        assert!(matches!(
            result,
            Err(OutwardCodeParseError::InvalidLength(_))
        ));
    }

    #[test]
    fn rejects_postcode_above_maximum_length() {
        // More than 8 characters
        let result = outward::OutwardCode::new("AB101 1AAA");
        assert!(matches!(
            result,
            Err(OutwardCodeParseError::InvalidLength(_))
        ));
    }

    #[test]
    fn rejects_unknown_area_in_outward_code() {
        let result = outward::OutwardCode::new("ZZ1");
        assert!(matches!(
            result,
            Err(OutwardCodeParseError::AreaParseError(_))
        ));
    }
}

#[cfg(feature = "geo")]
mod geo {
    #[test]
    fn test_geo_extraction() {
        use crate::PostCode;

        const EASTING_DATA: f64 = 416546f64;
        const NORTHING_DATA: f64 = 433216f64;

        let code = PostCode::new("BD1 1AF".to_string()).unwrap();
        let geo = code.geo();
        assert_eq!(geo.easting, EASTING_DATA);
        assert_eq!(geo.northing, NORTHING_DATA);
    }
}
