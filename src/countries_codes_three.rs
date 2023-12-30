/// Static array containing country codes.
///
/// The `COUNTRY_CODES_THREE` array holds 255 country codes. Each code is represented as a
/// string slice ('&'static str). The array is designed to map to specific locations
/// or regions globally, providing a standardized way of identifying countries.
///
/// # Usage
///
/// ```
/// use ipcap::countries_codes_three::COUNTRY_CODES_THREE;
///
/// // Accessing country code for the first country
/// let first_country_code = COUNTRY_CODES_THREE[1];
/// assert_eq!(first_country_code, "AP");
///
/// // Accessing country code for Lebanon
/// let usa_country_code = COUNTRY_CODES_THREE[124];
/// assert_eq!(usa_country_code, "LBN");
/// ```
pub static COUNTRY_CODES_THREE: [&str; 255] = [
    "", "AP", "EU", "AND", "ARE", "AFG", "ATG", "AIA", "ALB", "ARM", "ANT", "AGO", "AQ", "ARG",
    "ASM", "AUT", "AUS", "ABW", "AZE", "BIH", "BRB", "BGD", "BEL", "BFA", "BGR", "BHR", "BDI",
    "BEN", "BMU", "BRN", "BOL", "BRA", "BHS", "BTN", "BV", "BWA", "BLR", "BLZ", "CAN", "CC", "COD",
    "CAF", "COG", "CHE", "CIV", "COK", "CHL", "CMR", "CHN", "COL", "CRI", "CUB", "CPV", "CX",
    "CYP", "CZE", "DEU", "DJI", "DNK", "DMA", "DOM", "DZA", "ECU", "EST", "EGY", "ESH", "ERI",
    "ESP", "ETH", "FIN", "FJI", "FLK", "FSM", "FRO", "FRA", "FX", "GAB", "GBR", "GRD", "GEO",
    "GUF", "GHA", "GIB", "GRL", "GMB", "GIN", "GLP", "GNQ", "GRC", "GS", "GTM", "GUM", "GNB",
    "GUY", "HKG", "HM", "HND", "HRV", "HTI", "HUN", "IDN", "IRL", "ISR", "IND", "IO", "IRQ", "IRN",
    "ISL", "ITA", "JAM", "JOR", "JPN", "KEN", "KGZ", "KHM", "KIR", "COM", "KNA", "PRK", "KOR",
    "KWT", "CYM", "KAZ", "LAO", "LBN", "LCA", "LIE", "LKA", "LBR", "LSO", "LTU", "LUX", "LVA",
    "LBY", "MAR", "MCO", "MDA", "MDG", "MHL", "MKD", "MLI", "MMR", "MNG", "MAC", "MNP", "MTQ",
    "MRT", "MSR", "MLT", "MUS", "MDV", "MWI", "MEX", "MYS", "MOZ", "NAM", "NCL", "NER", "NFK",
    "NGA", "NIC", "NLD", "NOR", "NPL", "NRU", "NIU", "NZL", "OMN", "PAN", "PER", "PYF", "PNG",
    "PHL", "PAK", "POL", "SPM", "PCN", "PRI", "PSE", "PRT", "PLW", "PRY", "QAT", "REU", "ROU",
    "RUS", "RWA", "SAU", "SLB", "SYC", "SDN", "SWE", "SGP", "SHN", "SVN", "SJM", "SVK", "SLE",
    "SMR", "SEN", "SOM", "SUR", "STP", "SLV", "SYR", "SWZ", "TCA", "TCD", "TF", "TGO", "THA",
    "TJK", "TKL", "TKM", "TUN", "TON", "TLS", "TUR", "TTO", "TUV", "TWN", "TZA", "UKR", "UGA",
    "UM", "USA", "URY", "UZB", "VAT", "VCT", "VEN", "VGB", "VIR", "VNM", "VUT", "WLF", "WSM",
    "YEM", "YT", "SRB", "ZAF", "ZMB", "MNE", "ZWE", "A1", "A2", "O1", "ALA", "GGY", "IMN", "JEY",
    "BLM", "MAF", "BES", "SSD",
];
