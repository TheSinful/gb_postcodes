
#[cfg(feature = "geo")]
mod geo_build {

    /// Code-point open is a project licensed by the UK government to provide postcode to geolocation data
    /// in CSV format. See: https://en.wikipedia.org/wiki/Postcodes_in_the_United_Kingdom#Postcode_Data_-_Licensing_and_Usage
    ///
    /// The URL below is the "unexposed" API url that the download URL on the actual page(https://osdatahub.os.uk/data/downloads/open/CodePointOpen) redirects to.
    /// Which can be found by simply using the dev tools, looking at the network request when downloading, and observing the request URL.
    /// Furthermore, this is necessary to evade a Javascript necessity error provided by the server.  
    const CODE_POINT_OPEN_URL: &'static str = "https://api.os.uk/downloads/v1/products/CodePointOpen/downloads?area=GB&format=CSV&redirect";

    use reqwest::blocking::get;
    use std::{
        collections::HashMap,
        env, fs,
        io::{BufRead, BufReader, Cursor, Read},
        path::PathBuf,
    };
    use zip::ZipArchive;

    pub fn build() {
        let zipped_bytes: Vec<u8> = get(CODE_POINT_OPEN_URL)
            .expect("should've downloaded CSV data from code-point")
            .bytes()
            .expect("should've gotten downloaded CSV data in bytes")
            .into();

        let mut archive = ZipArchive::new(Cursor::new(zipped_bytes)).unwrap();
        let mut map: HashMap<String, (f64, f64)> = HashMap::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();

            if !file.name().starts_with("Data/CSV/") || !file.name().ends_with(".csv") {
                continue;
            }

            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line.unwrap();
                let mut cols = line.split(',');

                let postcode = cols.next().unwrap().trim().trim_matches('"').to_string();
                cols.next(); // skip quality field (first index)

                let easting: f64 = cols.next().unwrap().trim().parse().unwrap();
                let northing: f64 = cols.next().unwrap().trim().parse().unwrap();

                map.insert(postcode, (easting, northing));
            }
        }

        let out_dir = env::var("OUT_DIR").unwrap();
        fs::write(
            PathBuf::from(format!("{}/code_point_open_data.bin", out_dir)),
            postcard::to_allocvec(&map).unwrap(),
        )
        .unwrap();

        // stops cargo from re-running this script each time the user builds
        println!("cargo:rerun-if-changed=build.rs");
    }
}

fn main() {
    #[cfg(feature = "geo")]
    geo_build::build()
}