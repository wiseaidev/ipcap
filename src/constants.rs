// Database structure constants
pub const COUNTRY_BEGIN: u32 = 16776960; // Country data begin offset
pub const STATE_BEGIN_REV0: u32 = 16700000; // State data begin offset (Database revision 0)
pub const STATE_BEGIN_REV1: u32 = 16000000; // State data begin offset (Database revision 1)
pub const STRUCTURE_INFO_MAX_SIZE: u32 = 20; // Maximum size of structure information

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

pub const SEGMENT_RECORD_LENGTH: usize = 3; // Length of a segment record
pub const STANDARD_RECORD_LENGTH: usize = 3; // Standard record length
pub const ORG_RECORD_LENGTH: usize = 4; // Organization record length
pub const FULL_RECORD_LENGTH: usize = 50; // Full record length
