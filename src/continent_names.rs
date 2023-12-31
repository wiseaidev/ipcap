/// Static array containing continent names based on countries' indices.
///
/// The index corresponds to the index of a given country.
/// For example, `CONTINENT_NAMES[124]` represents the continent code for Lebanon, which is "AS" for Asia.
///
/// # Usage
///
/// ```
/// use ipcap::continent_names::CONTINENT_NAMES;
///
/// // Accessing continent name for the first country
/// // let first_continent_code = CONTINENT_NAMES[1];
/// // assert_eq!(first_continent_code, "AS");
///
/// // Accessing continent name for Lebanon
/// // let lebanon_continent_name = CONTINENT_NAMES[124];
/// // assert_eq!(lebanon_continent_name, "AS");
/// ```
pub static CONTINENT_NAMES: [&str; 255] = [
    "--", "AS", "EU", "EU", "AS", "AS", "NA", "NA", "EU", "AS", "NA", "AF", "AN", "SA", "OC", "EU",
    "OC", "NA", "AS", "EU", "NA", "AS", "EU", "AF", "EU", "AS", "AF", "AF", "NA", "AS", "SA", "SA",
    "NA", "AS", "AN", "AF", "EU", "NA", "NA", "AS", "AF", "AF", "AF", "EU", "AF", "OC", "SA", "AF",
    "AS", "SA", "NA", "NA", "AF", "AS", "AS", "EU", "EU", "AF", "EU", "NA", "NA", "AF", "SA", "EU",
    "AF", "AF", "AF", "EU", "AF", "EU", "OC", "SA", "OC", "EU", "EU", "NA", "AF", "EU", "NA", "AS",
    "SA", "AF", "EU", "NA", "AF", "AF", "NA", "AF", "EU", "AN", "NA", "OC", "AF", "SA", "AS", "AN",
    "NA", "EU", "NA", "EU", "AS", "EU", "AS", "AS", "AS", "AS", "AS", "EU", "EU", "NA", "AS", "AS",
    "AF", "AS", "AS", "OC", "AF", "NA", "AS", "AS", "AS", "NA", "AS", "AS", "AS", "NA", "EU", "AS",
    "AF", "AF", "EU", "EU", "EU", "AF", "AF", "EU", "EU", "AF", "OC", "EU", "AF", "AS", "AS", "AS",
    "OC", "NA", "AF", "NA", "EU", "AF", "AS", "AF", "NA", "AS", "AF", "AF", "OC", "AF", "OC", "AF",
    "NA", "EU", "EU", "AS", "OC", "OC", "OC", "AS", "NA", "SA", "OC", "OC", "AS", "AS", "EU", "NA",
    "OC", "NA", "AS", "EU", "OC", "SA", "AS", "AF", "EU", "EU", "AF", "AS", "OC", "AF", "AF", "EU",
    "AS", "AF", "EU", "EU", "EU", "AF", "EU", "AF", "AF", "SA", "AF", "NA", "AS", "AF", "NA", "AF",
    "AN", "AF", "AS", "AS", "OC", "AS", "AF", "OC", "AS", "EU", "NA", "OC", "AS", "AF", "EU", "AF",
    "OC", "NA", "SA", "AS", "EU", "NA", "SA", "NA", "NA", "AS", "OC", "OC", "OC", "AS", "AF", "EU",
    "AF", "AF", "EU", "AF", "--", "--", "--", "EU", "EU", "EU", "EU", "NA", "NA", "NA", "AF",
];
