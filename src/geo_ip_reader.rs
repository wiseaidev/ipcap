use crate::constants::*;
use crate::continent_names::CONTINENT_NAMES;
use crate::countries_codes_three::COUNTRY_CODES_THREE;
use crate::countries_codes_two::COUNTRY_CODES_TWO;
use crate::countries_names::COUNTRY_NAMES;
use crate::designated_market_area::DMAS;
use crate::errors::GeoIpReaderError;
use crate::time_zones::time_zone_by_country;
use crate::utils::{ip_to_number, read_data};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// `GeoIpReader` represents a reader for GeoIP databases, allowing the retrieval
/// of information based on IP addresses.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use ipcap::geo_ip_reader::GeoIpReader;
///
/// // Create a GeoIpReader instance
/// let mut reader_from_file = GeoIpReader::<File>::new().expect("Failed to create GeoIpReader");
/// ```
#[derive(Debug)]
pub struct GeoIpReader<R>
where
    R: Read + Seek,
{
    /// The underlying reader for the GeoIP database.
    fp: R,
    /// The type of the GeoIP database.
    database_type: u8,
    /// The length of each record in the GeoIP database.
    record_length: usize,
    /// The starting point of the database segments in the GeoIP database.
    database_segments: u32,
    netmask: usize,
}

impl<R> GeoIpReader<R>
where
    R: Read + Seek,
{
    /// Constructs a new `GeoIpReader` from the database.
    ///
    /// # Returns
    ///
    /// (`Result<GeoIpReader<File>, GeoIpReaderError>`): A Result containing a `GeoIpReader` on success
    /// or a `GeoIpReaderError` on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ipcap::geo_ip_reader::GeoIpReader;
    ///
    /// let result = GeoIpReader::<File>::new();
    /// match result {
    ///     Ok(reader) => println!("GeoIpReader created successfully: {:?}", reader),
    ///     Err(err) => eprintln!("Error creating GeoIpReader: {:?}", err),
    /// }
    /// ```
    pub fn new() -> Result<GeoIpReader<File>, GeoIpReaderError> {
        let fp = File::open("data/geo_ip_city.dat").map_err(|_| GeoIpReaderError::OpenFileError)?;

        let mut geoip_reader = GeoIpReader {
            fp,
            netmask: 0,
            database_type: 0,
            record_length: 3,
            database_segments: 0,
        };

        geoip_reader.detect_database_type()?;
        Ok(geoip_reader)
    }

    /// Detects the type of the GeoIP database and sets up segment sizes and start points accordingly.
    ///
    /// # Returns
    ///
    /// (`Result<(), GeoIpReaderError>`): A result indicating success or a `GeoIpReaderError` on failure.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue reading or seeking the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use ipcap::geo_ip_reader::GeoIpReader;
    /// use ipcap::errors::GeoIpReaderError;
    /// use std::fs::File;
    ///
    /// fn main() -> Result<(), GeoIpReaderError> {
    ///     let mut reader = GeoIpReader::<File>::new()?;
    ///     reader.detect_database_type()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn detect_database_type(&mut self) -> Result<(), GeoIpReaderError> {
        // Initialize default values
        self.database_type = COUNTRY_EDITION;
        self.record_length = STANDARD_RECORD_LENGTH;
        self.database_segments = COUNTRY_BEGIN;

        // Save current file position
        let file_position = self.fp.stream_position().unwrap();

        // Move to the end of the file minus 3 bytes
        self.fp.seek(SeekFrom::End(-3)).unwrap();

        // Loop to find the database type header
        for _ in 0..STRUCTURE_INFO_MAX_SIZE {
            // Define the expected header characters
            let chars = [255u8, 255u8, 255u8];
            // Read 3 bytes into delimiter
            let mut delimiter = [0u8; 3];
            self.fp.read_exact(&mut delimiter).unwrap();

            // Check if delimiter matches the expected header
            if delimiter == chars {
                // Read one more byte to determine the database type
                let mut byte = [0u8];
                self.fp.read_exact(&mut byte).unwrap();
                self.database_type = byte[0];

                // Adjust the database type if needed
                if self.database_type >= 106 {
                    self.database_type -= 105;
                }

                // Match the database type to set appropriate values
                match self.database_type {
                    REGION_EDITION_REV0 => self.database_segments = STATE_BEGIN_REV0,
                    REGION_EDITION_REV1 => self.database_segments = STATE_BEGIN_REV1,
                    CITY_EDITION_REV0 | CITY_EDITION_REV1 | CITY_EDITION_REV1_V6 | ORG_EDITION
                    | ISP_EDITION | ASNUM_EDITION | ASNUM_EDITION_V6 => {
                        self.database_segments = 0;

                        // Read SEGMENT_RECORD_LENGTH bytes into buf
                        let mut buf = [0u8; SEGMENT_RECORD_LENGTH];
                        self.fp.read_exact(&mut buf).unwrap();

                        // Calculate database_segments from buf
                        for (j, item) in buf.iter().enumerate().take(SEGMENT_RECORD_LENGTH) {
                            self.database_segments += (*item as u32) << (j * 8);
                        }

                        // Adjust record_length for certain database types
                        let long_records = [ORG_EDITION, ISP_EDITION];
                        if long_records.contains(&self.database_type) {
                            self.record_length = ORG_RECORD_LENGTH;
                        }
                    }
                    _ => {}
                }
                // Break the loop once the header is found
                break;
            } else {
                // Move back 4 bytes to continue searching
                self.fp.seek(SeekFrom::Current(-4)).unwrap();
            }
        }

        // Restore the original file position
        self.fp.seek(SeekFrom::Start(file_position)).unwrap();

        Ok(())
    }

    /// Using the record length and appropriate start points, seek to the
    /// country that corresponds to the converted IP address integer.
    ///
    /// # Arguments
    ///
    /// * `ip_number` - Result of ip_to_long conversion.
    ///
    /// # Returns
    ///
    /// The offset of the start of the record.
    ///
    /// # Errors
    ///
    /// Returns a `GeoIpReaderError` if the database is corrupted.
    ///
    /// # Examples
    ///
    /// ```
    /// use ipcap::geo_ip_reader::GeoIpReader;
    /// use std::fs::File;
    ///
    /// let mut geo_ip = GeoIpReader::<File>::new().unwrap();
    ///
    /// match geo_ip.get_country(16777216) {
    ///     Ok(offset) => println!("Country offset: {}", offset),
    ///     Err(err) => eprintln!("Error: {}", err),
    /// }
    /// ```
    pub fn get_country(&mut self, ip_number: u32) -> Result<usize, GeoIpReaderError> {
        // Initialize offset to 0
        let mut offset = 0;

        // Determine seek depth based on the length of the IP address
        let seek_depth = if ip_number.to_string().len() > 10 {
            127
        } else {
            31
        };

        // Iterate through the seek depth in reverse order
        for depth in (0..=seek_depth).rev() {
            // Create a buffer to store read data
            let mut buf: Vec<u8>;

            // Calculate the start index and read length for the database
            let start_index = 2 * self.record_length * offset;
            let read_length = 2 * self.record_length;

            // Create a new GeoIpReader instance for reading the database
            let mut reader = GeoIpReader::<File>::new().unwrap();
            // Seek to the start index in the database
            reader.fp.seek(SeekFrom::Start(start_index as u64)).unwrap();

            // Initialize the buffer with capacity and read data from the database
            buf = Vec::with_capacity(read_length);
            reader
                .fp
                .take(read_length as u64)
                .read_to_end(&mut buf)
                .unwrap();

            // Array to store two 32-bit values
            let mut x: [u32; 2] = [0, 0];

            // Extract values from the buffer
            for i in 0..2 {
                for j in 0..self.record_length {
                    let byte = buf[self.record_length * i + j] as u32;
                    x[i] += byte << (j * 8);
                }
            }

            // Check if the bit at the current depth is set in the IP number
            if ip_number & (1 << depth) != 0 {
                // If true, check if the second value is greater than or equal to database segments
                if x[1] >= self.database_segments {
                    // Set netmask and return the offset
                    self.netmask = seek_depth - depth + 1;
                    return Ok(x[1] as usize);
                }
                // Update offset with the second value
                offset = x[1] as usize;
            } else {
                // If the bit is not set, check if the first value is greater than or equal to database segments
                if x[0] >= self.database_segments {
                    // Set netmask and return the offset
                    self.netmask = seek_depth - depth + 1;
                    return Ok(x[0] as usize);
                }
                // Update offset with the first value
                offset = x[0] as usize;
            }
        }

        // If no valid offset is found, return an error
        Err(GeoIpReaderError::CorruptDatabase)
    }

    /// Get the geographical record for a converted IP address.
    ///
    /// This function retrieves information such as country code, region code,
    /// city, latitude, longitude, time zone, and more based on the provided IP address.
    ///
    /// # Arguments
    ///
    /// * `ip_number` - The converted IP address as a 32-bit unsigned integer.
    ///
    /// # Returns
    ///
    /// A HashMap containing geographical information:
    ///
    /// - "dma_code"
    /// - "area_code"
    /// - "metro_code"
    /// - "postal_code"
    /// - "country_code"
    /// - "country_code3"
    /// - "country_name"
    /// - "continent"
    /// - "region_code"
    /// - "city"
    /// - "latitude"
    /// - "longitude"
    /// - "time_zone"
    ///
    /// # Examples
    ///
    /// ```
    /// use ipcap::geo_ip_reader::GeoIpReader;
    /// use std::fs::File;
    ///
    /// let mut geo_ip = GeoIpReader::<File>::new().unwrap();
    ///
    /// let record = geo_ip.get_record("185.90.90.120");
    /// println!("Geographical Record: {:?}", record);
    /// ```
    ///
    pub fn get_record(&mut self, ip_number: &str) -> HashMap<&str, Option<String>> {
        // Get the offset of the country record for the given IP address
        let seek_country = self
            .get_country(ip_to_number(ip_number).try_into().unwrap())
            .unwrap();
        // Check if the offset is equal to the total number of database segments
        if seek_country == self.database_segments.try_into().unwrap() {
            return HashMap::new();
        }

        // Calculate the read length based on the record length and database segments
        let read_length = (2 * self.record_length - 1) * self.database_segments as usize;
        // Create a buffer to store the read data
        let mut buffer = vec![0; FULL_RECORD_LENGTH];

        // Seek to the position in the file where the record is located
        self.fp
            .seek(SeekFrom::Start(seek_country as u64 + read_length as u64))
            .unwrap();
        // Read the record data into the buffer
        self.fp.read_exact(&mut buffer).unwrap();

        // Initialize the record HashMap with default values
        let mut record = HashMap::new();
        record.insert("dma_code", Some("0".to_string()));
        record.insert("area_code", Some("0".to_string()));
        record.insert("metro_code", None);
        record.insert("postal_code", None);

        // Initialize latitude and longitude variables
        let mut latitude = 0;
        let mut longitude = 0;

        // Extract information from the buffer and populate the record HashMap
        let char = buffer[0] as usize;
        record.insert("country_code", Some(COUNTRY_CODES_TWO[char].to_string()));
        record.insert("country_code3", Some(COUNTRY_CODES_THREE[char].to_string()));
        record.insert("country_name", Some(COUNTRY_NAMES[char].to_string()));
        record.insert("continent", Some(CONTINENT_NAMES[char].to_string()));

        // Read region code from the buffer
        let (offset, region_code) = read_data(&buffer, 1);
        record.insert("region_code", region_code);

        // Read city from the buffer
        let (offset, city) = read_data(&buffer, offset + 1);
        record.insert("city", city);

        // Read postal code from the buffer
        let (offset, postal_code) = read_data(&buffer, offset + 1);
        record.insert("postal_code", postal_code);

        let offset = offset + 1;

        // Calculate latitude and longitude from the buffer
        for j in 0..3 {
            latitude += (buffer[offset + j] as i32) << (j * 8);
        }

        for j in 0..3 {
            longitude += (buffer[offset + j + 3] as i32) << (j * 8);
        }

        // Calculate latitude and longitude values and insert into the record
        record.insert(
            "latitude",
            Some(((latitude as f64) / 10000.0 - 180.0).to_string()),
        );
        record.insert(
            "longitude",
            Some(((longitude as f64) / 10000.0 - 180.0).to_string()),
        );

        // Process additional information for US in case of specific database types
        if (self.database_type == CITY_EDITION_REV1 || self.database_type == CITY_EDITION_REV1_V6)
            && record.get("country_code").unwrap() == &Some("US".to_string())
        {
            // Process DMA code and area code for US records
            let mut dma_area = 0;
            for j in 0..3 {
                dma_area += (buffer[offset + j + 6] as i32) << (j * 8);
            }
            record.insert("dma_code", Some((dma_area / 1000).to_string()));
            record.insert("area_code", Some((dma_area % 1000).to_string()));
            // Map DMA code to metro code using the DMAS mapping
            record.insert(
                "metro_code",
                Some(
                    DMAS.get(
                        &record
                            .get("dma_code")
                            .unwrap()
                            .clone()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap(),
                    )
                    .cloned()
                    .unwrap_or("")
                    .to_string(),
                ),
            );
        }

        // Obtain country code and region code from the record or provide default values
        let country_code = record.get("country_code").cloned().unwrap_or_else(|| {
            eprintln!("Warning: Country Code is missing or not a valid string");
            Some("default".to_string())
        });

        let region_code = record.get("region_code").cloned().unwrap_or_else(|| {
            eprintln!("Warning: Region Code is missing or not a valid string");
            Some("default".to_string())
        });

        // Obtain time zone information based on country code and region code
        let params = (
            &country_code.unwrap_or("default".to_string()),
            &region_code.unwrap_or("default".to_string()),
        );
        record.insert(
            "time_zone",
            Some(
                time_zone_by_country(params.0, params.1, None)
                    .unwrap_or_default()
                    .to_string(),
            ),
        );

        // Return the populated record HashMap
        println!("{:?}", record);
        record
    }

    /// Look up the time zone for a given IP address.
    /// Use this method if you have a Region or City database.
    ///
    /// # Arguments
    ///
    /// * `addr` - IP address
    ///
    /// # Returns
    ///
    /// Time zone as a string.
    ///
    pub fn get_time_zone_given_ip_addr(&mut self, addr: &str) -> Option<String> {
        let record = self.get_record(addr);
        record.get("time_zone")?.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_geo_ip_reader() {
        let result = GeoIpReader::<File>::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_country() {
        let mut geo_ip = GeoIpReader::<File>::new().unwrap();

        match geo_ip.get_country(16777216) {
            Ok(offset) => assert_eq!(offset, 2735459),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    #[test]
    fn test_get_time_zone_given_ip_addr() {
        // Create a test instance of GeoIpReader
        let mut geo_ip = GeoIpReader::<File>::new().unwrap();

        // Call the get_record method with a test IP number
        let mut result = geo_ip.get_time_zone_given_ip_addr("185.90.90.120");

        // Add assertions based on the expected result
        assert_eq!(result, Some("Asia/Riyadh".to_string()));

        // Call the get_record method with a test IP number
        result = geo_ip.get_time_zone_given_ip_addr("108.95.4.105");

        // Add assertions based on the expected result
        assert_eq!(result, Some("America/Los_Angeles".to_string()));
    }

    #[test]
    fn test_get_record_with_valid_ip() {
        let mut geo_ip = GeoIpReader::<File>::new().unwrap();
        let record = geo_ip.get_record("185.90.90.120");

        // Add assertions based on expected values for the test IP
        assert_eq!(record["country_code"].as_deref(), Some("SA"));
    }

    #[test]
    fn test_all_records_with_valid_ip() {
        let mut geo_ip = GeoIpReader::<File>::new().unwrap();
        let record = geo_ip.get_record("108.95.4.105");

        let mut expected_values = HashMap::new();
        expected_values.insert("country_code3", Some("USA".to_string()));
        expected_values.insert("longitude", Some("-117.23349999999999".to_string()));
        expected_values.insert("country_code", Some("US".to_string()));
        expected_values.insert("continent", Some("NA".to_string()));
        expected_values.insert("postal_code", Some("92109".to_string()));
        expected_values.insert("area_code", Some("858".to_string()));
        expected_values.insert("country_name", Some("United States".to_string()));
        expected_values.insert("region_code", Some("CA".to_string()));
        expected_values.insert("dma_code", Some("825".to_string()));
        expected_values.insert("city", Some("San Diego".to_string()));
        expected_values.insert("latitude", Some("32.79769999999999".to_string()));
        expected_values.insert("time_zone", Some("America/Los_Angeles".to_string()));
        expected_values.insert("metro_code", Some("San Diego, CA".to_string()));

        for (key, expected_value) in expected_values.iter() {
            assert_eq!(record.get(key).cloned(), Some(expected_value).cloned());
        }
    }

    #[test]
    #[should_panic(expected = "Invalid IP address")]
    fn test_get_record_with_invalid_ip() {
        let mut geo_ip = GeoIpReader::<File>::new().unwrap();
        let record = geo_ip.get_record("-");

        assert_eq!(record.is_empty(), true);
    }
}
