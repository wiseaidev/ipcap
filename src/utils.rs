use crate::geo_ip_reader::Record;
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Helper function to create a single-level hashmap.
///
/// This function takes a single value and wraps it in a hashmap with a default key "default".
///
/// # Arguments
///
/// * `value` - The value to be stored in the hashmap.
///
/// # Returns
///
/// (`HashMap<&'static str, &'static str>`): A map with a single entry.
///
/// # Examples
///
/// ```
/// use ipcap::utils::single_level;
/// use std::collections::HashMap;
///
/// let map: HashMap<&'static str, &'static str> = single_level("example_value");
/// assert_eq!(map.get("default"), Some(&"example_value"));
/// assert_eq!(map.len(), 1);
/// ```
pub fn single_level(value: &'static str) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("default", value);
    map
}

/// Helper function to create a multi-level hashmap.
///
/// This function takes a vector of key-value pairs and converts them into a hashmap.
///
/// # Arguments
///
/// * `entries` - A vector of key-value pairs to be stored in the hashmap.
///
/// # Returns
///
/// (`HashMap<&'static str, &'static str>`): A map containing the provided key-value pairs.
///
/// # Examples
///
/// ```
/// use ipcap::utils::multi_level;
/// use std::collections::HashMap;
///
/// let entries = vec![
///     ("key1", "value1"),
///     ("key2", "value2"),
/// ];
///
/// let map: HashMap<&'static str, &'static str> = multi_level(entries);
/// assert_eq!(map.get("key1"), Some(&"value1"));
/// assert_eq!(map.get("key2"), Some(&"value2"));
/// assert_eq!(map.len(), 2);
/// ```
pub fn multi_level(
    entries: Vec<(&'static str, &'static str)>,
) -> HashMap<&'static str, &'static str> {
    entries.into_iter().collect()
}

/// Converts an IP address in string format to a 128-bit unsigned integer representation.
///
/// This function takes a string representing an IP address and converts it into a 128-bit
/// unsigned integer. It supports both IPv4 and IPv6 addresses. The result is the numeric
/// representation of the IP address.
///
/// # Arguments
///
/// * `ip` - A string slice containing the IP address.
///
/// # Returns
///
/// (`u128`): A 128-bit unsigned integer representation of the IP address.
///
/// # Panics
///
/// This function will panic if the input string does not represent a valid IPv4 or IPv6 address.
///
/// # Examples
///
/// ```
/// use ipcap::utils::ip_to_number;
///
/// let ipv4_address = "1.32.0.0";
/// let ipv6_address = "2001:0db8:85a3:0000:0000:8a2e:0370:7334";
///
/// let ipv4_numeric = ip_to_number(ipv4_address);
/// let ipv6_numeric = ip_to_number(ipv6_address);
///
/// assert_eq!(ipv4_numeric, 18874368);
/// assert_eq!(ipv6_numeric, 42540766411283223938465490629124161536);
/// ```
pub fn ip_to_number(ip: &str) -> u128 {
    match ip.parse::<Ipv4Addr>() {
        Ok(ipv4_addr) => {
            // IPv4 case
            let ipv4_u32: u32 = u32::from(ipv4_addr);
            u128::from(ipv4_u32)
        }
        Err(_) => {
            // Not an IPv4 address, try IPv6
            match ip.parse::<Ipv6Addr>() {
                Ok(ipv6_addr) => {
                    // IPv6 case
                    let segments = ipv6_addr.segments();
                    (u128::from(segments[0]) << 112)
                        | (u128::from(segments[1]) << 96)
                        | (u128::from(segments[2]) << 64)
                        | u128::from(segments[3])
                }
                Err(_) => {
                    // Invalid IP address
                    panic!("Invalid IP address: {}", ip);
                }
            }
        }
    }
}

/// Reads null-terminated string data from the given buffer starting at the specified position.
///
/// # Arguments
///
/// * `buffer` - The buffer containing the string data.
/// * `pos` - The starting position to read the string from.
///
/// # Returns
///
/// A tuple containing:
/// - The updated position after reading the string.
/// - An optional string representing the data read. `None` if no valid string is found.
///
/// # Examples
///
/// ```rust
/// use ipcap::utils::read_data;
///
/// let buffer = b"Hello\0World";
/// let pos = 0;
/// let (new_pos, data) = read_data(buffer, pos);
/// assert_eq!(new_pos, 5);
/// assert_eq!(data, Some("Hello".into()));
/// ```
pub fn read_data(buffer: &[u8], pos: usize) -> (usize, Option<Box<str>>) {
    let mut cur = pos;
    while buffer[cur] != 0 {
        cur += 1;
    }
    let data = if cur > pos {
        Some(
            String::from_utf8_lossy(&buffer[pos..cur])
                .to_string()
                .into_boxed_str(),
        )
    } else {
        None
    };
    (cur, data)
}

/// Pretty prints the fields of a Record struct by sorting them alphabetically and formatting the output.
///
/// # Arguments
///
/// * `record` - A reference to a Record struct.
///
/// # Example
///
/// ```rust
/// use ipcap::utils::pretty_print_dict;
/// use ipcap::geo_ip_reader::Record;
/// use ipcap::countries::Country;
/// use ipcap::designated_market_area::DesignatedMarketArea;
///
/// let record = Record {
///     dma: Some(DesignatedMarketArea(80700)),
///     postal_code: Some("94040".into()),
///     country: Country::UnitedStates,
///     region_code: Some("CA".into()),
///     city: Some("Mountain View".into()),
///     latitude: 37.3845,
///     longitude: -122.0881,
///     time_zone: "America/Los_Angeles",
/// };
///
/// pretty_print_dict(record);
/// ```
///
/// Output:
///
/// ```sh
/// {
///     "area_code": "650",
///     "city": "Mountain View",
///     "continent": "NA",
///     "country_code": "US",
///     "country_code3": "USA",
///     "country_name": "United States",
///     "dma_code": "807",
///     "latitude": "37.3845",
///     "longitude": "-122.0881",
///     "metro_code": "San Francisco, CA",
///     "postal_code": "94040",
///     "region_code": "CA",
///     "time_zone": "America/Los_Angeles",
/// }
/// ```
pub fn pretty_print_dict(record: Record) {
    let data: Vec<(&str, Option<String>)> = vec![
        ("dma_code", record.dma.map(|d| d.dma_code().to_string())),
        ("area_code", record.dma.map(|d| d.area_code().to_string())),
        ("metro_code", record.dma.map(|c| c.to_string())),
        (
            "postal_code",
            record.postal_code.as_ref().map(|d| d.to_string()),
        ),
        (
            "country_code",
            Some(record.country.alphabetic_code_2().to_string()),
        ),
        (
            "country_code3",
            Some(record.country.alphabetic_code_3().to_string()),
        ),
        ("country_name", Some(record.country.to_string())),
        (
            "continent",
            record.country.continent().map(|c| c.to_string()),
        ),
        ("region_code", record.region_code.map(|d| d.to_string())),
        ("city", record.city.map(|d| d.to_string())),
        ("latitude", Some(record.latitude.to_string())),
        ("longitude", Some(record.longitude.to_string())),
        ("time_zone", Some(record.time_zone.to_string())),
    ];

    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.0.cmp(b.0));

    println!("{{");

    for (key, value) in sorted_data {
        print!("    \"\u{1b}[1;32m{}\": ", key); // Green color for keys
        match value {
            Some(v) => print!("\u{1b}[1;37m\"{}\"\u{1b}[0m,", v), // Silver color for values
            None => print!("\u{1b}[1;30mnull\u{1b}[0m,"),         // Gray color for null values
        }
        println!();
    }

    println!("}}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_single_level() {
        let map: HashMap<&'static str, &'static str> = single_level("example_value");
        assert_eq!(map.get("default"), Some(&"example_value"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_multi_level() {
        let entries = vec![("key1", "value1"), ("key2", "value2")];
        let map: HashMap<&'static str, &'static str> = multi_level(entries);
        assert_eq!(map.get("key1"), Some(&"value1"));
        assert_eq!(map.get("key2"), Some(&"value2"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_ip_to_number_ipv4() {
        let ipv4_address = "192.168.1.1";
        let result = ip_to_number(ipv4_address);
        assert_eq!(result, 3232235777);
    }

    #[test]
    fn test_ip_to_number_ipv6() {
        // Test with a valid IPv6 address
        let ipv6_address = "2001:0db8:85a3:0000:0000:8a2e:0370:7334";
        let result = ip_to_number(ipv6_address);
        assert_eq!(result, 42540766411283223938465490629124161536);
    }

    #[test]
    #[should_panic(expected = "Invalid IP address")]
    fn test_ip_to_number_invalid() {
        let invalid_address = "invalid_ip";
        ip_to_number(invalid_address);
    }

    #[test]
    fn test_read_data_with_valid_string() {
        let buffer = b"Hello\0World";
        let pos = 0;
        let (new_pos, data) = read_data(buffer, pos);
        assert_eq!(new_pos, 5);
        assert_eq!(data, Some("Hello".into()));
    }

    #[test]
    fn test_read_data_with_empty_string() {
        let buffer = b"\0World";
        let pos = 0;
        let (new_pos, data) = read_data(buffer, pos);
        assert_eq!(new_pos, 0);
        assert_eq!(data, None);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 10 but the index is 10")]
    fn test_read_data_with_no_null_terminator() {
        let buffer = b"HelloWorld";
        let pos = 0;
        let (new_pos, data) = read_data(buffer, pos);
        assert_eq!(new_pos, buffer.len());
        assert_eq!(data, None);
    }
}

#[macro_export]
macro_rules! codegen {
    ($name: expr) => {
        include!(concat!(env!("OUT_DIR"), "/", $name))
    };

    (statement; $name: expr) => {
        include!(concat!(env!("OUT_DIR"), "/", $name));
    };
}
