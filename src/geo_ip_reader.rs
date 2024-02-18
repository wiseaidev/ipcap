use crate::constants::*;
use crate::countries::Country;
use crate::designated_market_area::DesignatedMarketArea;
use crate::errors::GeoIpReaderError;
use crate::time_zones::time_zone_by_country;
use crate::utils::{ip_to_number, read_data};
use dirs::home_dir;
use std::env;
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
/// let mut reader_from_file = GeoIpReader::<File>::new("v4").expect("Failed to create GeoIpReader");
/// ```
#[derive(Debug)]
pub struct GeoIpReader<R>
where
    R: Read + Seek,
{
    /// The underlying reader for the GeoIP v4 database.
    fp: R,
    /// The type of the GeoIP database.
    database_type: u8,
    /// The length of each record in the GeoIP database.
    record_length: usize,
    /// The starting point of the database segments in the GeoIP database.
    database_segments: u32,
    netmask: usize,
}

#[derive(Debug, PartialEq)]
pub struct Record<'a> {
    pub dma: Option<DesignatedMarketArea>,
    pub postal_code: Option<Box<str>>,
    pub country: Country,
    pub region_code: Option<Box<str>>,
    pub city: Option<Box<str>>,
    pub latitude: f64,
    pub longitude: f64,
    pub time_zone: &'a str,
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
    /// let result = GeoIpReader::<File>::new("v4");
    /// match result {
    ///     Ok(reader) => println!("GeoIpReader created successfully: {:?}", reader),
    ///     Err(err) => eprintln!("Error creating GeoIpReader: {:?}", err),
    /// }
    /// ```
    pub fn new(type_: &str) -> Result<GeoIpReader<File>, GeoIpReaderError> {
        const ENV_VAR_NAME: &str = "IPCAP_FILE_PATH";
        let file_path = match env::var(ENV_VAR_NAME) {
            Ok(val) => val,
            Err(_) => {
                let default_path = match type_ {
                    "v4" => {
                        let mut path = home_dir().unwrap_or_default();
                        path.push("ipcap");
                        path.push("geo_ip_city_v4.dat");
                        path
                    }
                    "v6" => {
                        let mut path = home_dir().unwrap_or_default();
                        path.push("ipcap");
                        path.push("geo_ip_city_v6.dat");
                        path
                    }
                    _ => {
                        return Err(GeoIpReaderError::OpenFileError);
                    }
                };
                default_path.to_string_lossy().into_owned()
            }
        };

        let fp = File::open(&file_path).map_err(|_| GeoIpReaderError::OpenFileError)?;

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
    ///     let mut reader = GeoIpReader::<File>::new("v4")?;
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
    /// let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();
    ///
    /// match geo_ip.get_country(16777216) {
    ///     Ok(offset) => println!("Country offset: {}", offset),
    ///     Err(err) => eprintln!("Error: {}", err),
    /// }
    /// ```
    pub fn get_country(&mut self, ip_number: u128) -> Result<usize, GeoIpReaderError> {
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
            let mut reader;
            if seek_depth == 31 {
                reader = GeoIpReader::<File>::new("v4").unwrap();
            } else {
                reader = GeoIpReader::<File>::new("v6").unwrap();
            }
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
    /// # Examples
    /// ```
    /// use ipcap::geo_ip_reader::GeoIpReader;
    /// use std::fs::File;
    ///
    /// let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();
    ///
    /// let record = geo_ip.get_record("185.90.90.120");
    /// println!("Geographical Record: {:?}", record);
    /// ```
    pub fn get_record(&mut self, ip_number: &str) -> Record {
        // Get the offset of the country record for the given IP address
        let seek_country = self
            .get_country(ip_to_number(ip_number).try_into().unwrap())
            .unwrap();

        // Check if the offset is equal to the total number of database segments
        println!("{:?}", self.database_segments);
        if seek_country == self.database_segments.try_into().unwrap() {
            // todo!("Error handling")
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

        let mut latitude = 0;
        let mut longitude = 0;

        let country = Country::from_buffer(buffer[0]).unwrap();

        let (offset, region_code) = read_data(&buffer, 1);
        let (offset, city) = read_data(&buffer, offset + 1);
        let (offset, postal_code) = read_data(&buffer, offset + 1);
        let offset = offset + 1;

        for j in 0..3 {
            latitude += (buffer[offset + j] as i32) << (j * 8);
        }

        for j in 0..3 {
            longitude += (buffer[offset + j + 3] as i32) << (j * 8);
        }

        let latitude = latitude as f64 / 10000.0 - 180.0;
        let longitude = longitude as f64 / 10000.0 - 180.0;

        let dma = if (self.database_type == CITY_EDITION_REV1
            || self.database_type == CITY_EDITION_REV1_V6)
            && country == Country::UnitedStates
        {
            let mut dma_area = 0;
            for j in 0..3 {
                dma_area += (buffer[offset + j + 6] as u32) << (j * 8);
            }

            Some(DesignatedMarketArea(dma_area))
        } else {
            None
        };

        let time_zone = time_zone_by_country(
            country.alphabetic_code_2(),
            match &region_code {
                Some(d) => d,
                None => "default",
            },
            None,
        )
        .unwrap_or_default();

        Record {
            dma,
            postal_code,
            country,
            region_code,
            city,
            latitude,
            longitude,
            time_zone,
        }
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
    pub fn get_time_zone_given_ip_addr(&mut self, addr: &str) -> &str {
        let record = self.get_record(addr);
        record.time_zone
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_geo_ip_reader() {
        let result = GeoIpReader::<File>::new("v4");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_country() {
        let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();

        match geo_ip.get_country(16777216) {
            Ok(offset) => assert_eq!(offset, 2735459),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    #[test]
    fn test_get_time_zone_given_ip_addr() {
        let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();

        let result = geo_ip.get_time_zone_given_ip_addr("185.90.90.120");
        assert_eq!(result, "Asia/Riyadh".to_string());

        let result = geo_ip.get_time_zone_given_ip_addr("108.95.4.105");
        assert_eq!(result, "America/Los_Angeles");
    }

    #[test]
    fn test_get_record_with_valid_ip() {
        let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();
        let record = geo_ip.get_record("185.90.90.120");

        assert_eq!(record.country, Country::SaudiArabia);
    }

    #[test]
    fn test_all_records_with_valid_ip() {
        let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();
        let record = geo_ip.get_record("108.95.4.105");

        let expected_value = Record {
            dma: Some(DesignatedMarketArea(825858)),
            postal_code: Some("92109".into()),
            country: Country::UnitedStates,
            region_code: Some("CA".into()),
            city: Some("San Diego".into()),
            latitude: 32.79769999999999,
            longitude: -117.23349999999999,
            time_zone: "America/Los_Angeles",
        };

        assert_eq!(record, expected_value);
    }

    #[test]
    #[should_panic(expected = "Invalid IP address")]
    fn test_get_record_with_invalid_ip() {
        let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();
        let _record = geo_ip.get_record("-");

        todo!("Error handling")
    }
}
