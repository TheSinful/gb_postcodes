**British Postcode types, parsing, and geolocational data.**

## Install 
```toml
[dependencies]
gb_postcode = { version = "1.0.0", features = ["geo"] }
```
*Note: geo-locational data is enabled by the geo feature, and fist compilation will be heavy as geo data must be downloaded*

## Example

```rs
use gb_postcode::PostCode;

fn main() {
    let code_str = "BD1 1AF"; // space is expected and standard among postcodes to separate inward and outward codes.
    let postcode = PostCode::new(code_str.to_string()).unwrap();

    let outward_code = postcode.outward_code(); 
    let area = postcode.area(); 
    let district = postcode.district(); 

    let inward_code = postcode.inward_code();
    let sector = postcode.sector(); 
    let unit = postcode.unit();

    // Note that all shown fields are accessible directly within their parent struct. 
    // For instance, 'inward_code.sector' is equivalent to 'postcode.sector()'
    
    let code_str: &str = postcode.as_str(); 
}
```

## Features

- **Distinct types** for each part of a `PostCode` and their respective parts:
    - `OutwardCode`
        - `PostCodeArea`
        - `PostCodeDistrict`
    - `InwardCode`
        - `PostCodeUnit`
        - `PostCodeSector`
- **Geo-locational data** (within the optional `geo` feature) **in easting/northing format**
  geo data is bundled with the crate during build, utilizing data pre-compiled by
  Code-Point Open: a UK government licensed project for linking postcodes with geolocational data
  therefore geo data is reliable as well as fast in comparison to identical API fetched data.
- **Extensive error handling** through distinct types for each corresponding type.


## Credits

- [Code-Point Open](https://www.ordnancesurvey.co.uk/products/code-point-open) Provided precomputed data linking each postcode to a geolocation.
- [Wikipedia](https://en.wikipedia.org/wiki/Postcodes_in_the_United_Kingdom) Provided detailed breakdowns of postcode structure, as well as links to useful resources (like Code-Point Open)
