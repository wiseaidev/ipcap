//! # IPCap
//!
//! Decode IP addresses into state, postal code, country, coordinates, etc without the
//! need to connect to the internet and zero API calls.
//!
//! ## Quick Start
//!
//! Get started with the `ipcap` library by following these simple steps:
//!
//! 1. Install the `ipcap` crate by adding the following line to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! ipcap = "0.1.2"
//! ```
//!
//! 2. Use the `IpCap` struct to perform IP lookup without internet access:
//!
//! ```rust
//! use ipcap::geo_ip_reader::GeoIpReader;
//! use ipcap::utils::pretty_print_dict;
//! use std::fs::File;
//! use std::collections::HashMap;
//!
//!
//! let mut geo_ip = GeoIpReader::<File>::new().unwrap();
//! let record = geo_ip.get_record("108.95.4.105");
//!
//! let mut expected_values = HashMap::new();
//! expected_values.insert("country_code3", Some("USA".to_string()));
//! expected_values.insert("longitude", Some("-117.23349999999999".to_string()));
//! expected_values.insert("country_code", Some("US".to_string()));
//! expected_values.insert("continent", Some("NA".to_string()));
//! expected_values.insert("postal_code", Some("92109".to_string()));
//! expected_values.insert("area_code", Some("858".to_string()));
//! expected_values.insert("country_name", Some("United States".to_string()));
//! expected_values.insert("region_code", Some("CA".to_string()));
//! expected_values.insert("dma_code", Some("825".to_string()));
//! expected_values.insert("city", Some("San Diego".to_string()));
//! expected_values.insert("latitude", Some("32.79769999999999".to_string()));
//! expected_values.insert("time_zone", Some("America/Los_Angeles".to_string()));
//! expected_values.insert("metro_code", Some("San Diego, CA".to_string()));
//!
//! for (key, expected_value) in expected_values.iter() {
//!     assert_eq!(record.get(key).cloned(), Some(expected_value).cloned());
//! }
//!
//! pretty_print_dict(&expected_values);
//! ```
//!
//! ## Options
//!
//! | Option                  | Description                                               |
//! |-------------------------|-----------------------------------------------------------|
//! | `--target`              | Set the IP address to lookup with the `lookup` method. |
//!
//! ## GitHub Repository
//!
//! You can access the source code for the `ipcap` crate on [GitHub](https://github.com/wiseaidev/ipcap).
//!
//! ## Contributing
//!
//! Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement,
//! please engage with the project on [GitHub](https://github.com/wiseaidev/ipcap).
//! Your contributions help improve this crate for the community.

#[cfg(feature = "cli")]
pub mod cli;
pub mod constants;
pub mod continent_names;
pub mod countries_codes_three;
pub mod countries_codes_two;
pub mod countries_names;
pub mod designated_market_area;
pub mod errors;
pub mod geo_ip_reader;
pub mod time_zones;
pub mod utils;
