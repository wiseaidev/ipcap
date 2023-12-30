// GeoIP options
pub const GEOIP_STANDARD: u32 = 0;
pub const GEOIP_MEMORY_CACHE: u32 = 1;

// Storage / caching flags
pub const STANDARD: u32 = 0; // Standard storage
pub const MEMORY_CACHE: u32 = 1; // Memory caching
pub const MMAP_CACHE: u32 = 8; // MMAP caching

// Database structure constants
pub const COUNTRY_BEGIN: u32 = 16776960; // Country data begin offset
pub const STATE_BEGIN_REV0: u32 = 16700000; // State data begin offset (Database revision 0)
pub const STATE_BEGIN_REV1: u32 = 16000000; // State data begin offset (Database revision 1)

pub const STRUCTURE_INFO_MAX_SIZE: u32 = 20; // Maximum size of structure information
pub const DATABASE_INFO_MAX_SIZE: u32 = 100; // Maximum size of database information

// Database editions
pub const COUNTRY_EDITION: u8 = 1; // Country edition identifier
pub const COUNTRY_EDITION_V6: u8 = 12; // Country edition for IPv6 identifier
pub const REGION_EDITION_REV0: u8 = 7; // Region edition for Database revision 0 identifier
pub const REGION_EDITION_REV1: u8 = 3; // Region edition for Database revision 1 identifier
pub const CITY_EDITION_REV0: u8 = 6; // City edition for Database revision 0 identifier
pub const CITY_EDITION_REV1: u8 = 2; // City edition for Database revision 1 identifier
pub const CITY_EDITION_REV1_V6: u8 = 30; // City edition for Database revision 1 for IPv6 identifier
pub const ORG_EDITION: u8 = 5; // Organization edition identifier
pub const ISP_EDITION: u8 = 4; // Internet Service Provider edition identifier
pub const ASNUM_EDITION: u8 = 9; // Autonomous System Number edition identifier
pub const ASNUM_EDITION_V6: u8 = 21; // Autonomous System Number edition for IPv6 identifier

// Not yet supported databases
pub const PROXY_EDITION: u32 = 8; // Proxy edition identifier
pub const NETSPEED_EDITION: u32 = 11; // NetSpeed edition identifier

// Collection of databases
pub const IPV6_EDITIONS: [u8; 3] = [COUNTRY_EDITION_V6, ASNUM_EDITION_V6, CITY_EDITION_REV1_V6];
pub const CITY_EDITIONS: [u8; 3] = [CITY_EDITION_REV0, CITY_EDITION_REV1, CITY_EDITION_REV1_V6];
pub const REGION_EDITIONS: [u8; 2] = [REGION_EDITION_REV0, REGION_EDITION_REV1];
pub const REGION_CITY_EDITIONS: [u8; 5] = [
    REGION_EDITION_REV0,
    REGION_EDITION_REV1,
    CITY_EDITION_REV0,
    CITY_EDITION_REV1,
    CITY_EDITION_REV1_V6,
];

pub const SEGMENT_RECORD_LENGTH: usize = 3; // Length of a segment record
pub const STANDARD_RECORD_LENGTH: usize = 3; // Standard record length
pub const ORG_RECORD_LENGTH: usize = 4; // Organization record length
pub const MAX_RECORD_LENGTH: u32 = 4; // Maximum record length
pub const MAX_ORG_RECORD_LENGTH: u32 = 300; // Maximum organization record length
pub const FULL_RECORD_LENGTH: u32 = 50; // Full record length

pub const US_OFFSET: u32 = 1; // United States offset
pub const CANADA_OFFSET: u32 = 677; // Canada offset
pub const WORLD_OFFSET: u32 = 1353; // World offset
pub const FIPS_RANGE: u32 = 360; // FIPS range
pub const ENCODING: &str = "iso-8859-1"; // Database encoding
