use crate::constants::*;
use crate::errors::GeoIpReaderError;
use std::fs::File;
use std::io::{Read, Seek};

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
        let fp = File::open("data/database.dat").map_err(|_| GeoIpReaderError::OpenFileError)?;

        let mut geoip_reader = GeoIpReader {
            fp,
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
        let filepos = self.fp.stream_position().unwrap();

        // Move to the end of the file minus 3 bytes
        self.fp.seek(std::io::SeekFrom::End(-3)).unwrap();

        // Loop to find the database type header
        for _ in 0..STRUCTURE_INFO_MAX_SIZE {
            // Define the expected header characters
            let chars = [255u8, 255u8, 255u8];
            // Read 3 bytes into delim
            let mut delim = [0u8; 3];
            self.fp.read_exact(&mut delim).unwrap();

            // Check if delim matches the expected header
            if delim == chars {
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
                self.fp.seek(std::io::SeekFrom::Current(-4)).unwrap();
            }
        }

        // Restore the original file position
        self.fp.seek(std::io::SeekFrom::Start(filepos)).unwrap();

        Ok(())
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
}
