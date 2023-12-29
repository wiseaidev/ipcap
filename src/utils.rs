use std::collections::HashMap;

/// Helper function to create a single-level hashmap.
///
/// This function takes a single value and wraps it in a hashmap with a default key "default".
///
/// # Arguments
///
/// * `value` - The value to be stored in the hashmap.
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
}
