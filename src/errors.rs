use std::fmt;

/// Enum representing various errors that may occur while using `GeoIpReader`.
#[derive(Debug)]
pub enum GeoIpReaderError {
    /// Error indicating a failure to retrieve host information by name.
    GetHostByNameError,
    /// Error indicating an invalid GeoIP database type.
    InvalidDatabaseType,
    /// Error indicating a failure to open a file.
    OpenFileError,
    CorruptDatabase,
}

impl fmt::Display for GeoIpReaderError {
    /// Implements the `fmt::Display` trait to customize the error message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeoIpReaderError::GetHostByNameError => write!(f, "Error getting host by name"),
            GeoIpReaderError::InvalidDatabaseType => write!(f, "Invalid database type"),
            GeoIpReaderError::OpenFileError => write!(f, "Cannot open file"),
            GeoIpReaderError::CorruptDatabase => write!(f, "Corrupt database"),
        }
    }
}
