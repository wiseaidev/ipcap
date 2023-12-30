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

/// Converts an IP address in string format to a 64-bit unsigned integer representation.
///
/// This function takes a string representing an IP address and converts it into a 64-bit
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
/// let ipv4_address = "192.168.1.1";
/// let ipv6_address = "2001:0db8:85a3:0000:0000:8a2e:0370:7334";
///
/// let ipv4_numeric = ip_to_number(ipv4_address);
/// let ipv6_numeric = ip_to_number(ipv6_address);
///
/// assert_eq!(ipv4_numeric, 3232235777);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_single_level() {
        // Test with a single-level entry
        let map: HashMap<&'static str, &'static str> = single_level("example_value");
        assert_eq!(map.get("default"), Some(&"example_value"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_multi_level() {
        // Test with multiple key-value pairs
        let entries = vec![("key1", "value1"), ("key2", "value2")];
        let map: HashMap<&'static str, &'static str> = multi_level(entries);
        assert_eq!(map.get("key1"), Some(&"value1"));
        assert_eq!(map.get("key2"), Some(&"value2"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_ip_to_number_ipv4() {
        // Test with a valid IPv4 address
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
}
