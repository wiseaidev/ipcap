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
//! ipcap = "0.1.6"
//! ```
//!
//! 2. Use the `IpCap` struct to perform IP lookup without internet access:
//!
//! ```rust
//! use std::fs::File;
//! use ipcap::geo_ip_reader::{GeoIpReader, Record};
//! use ipcap::countries::Country;
//! use ipcap::designated_market_area::DesignatedMarketArea;
//!
//! let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();
//! let mut record = geo_ip.get_record("108.95.4.105");
//!
//! let mut expected_value = Record {
//!     dma: Some(DesignatedMarketArea(825858)),
//!     postal_code: Some("92109".into()),
//!     country: Country::UnitedStates,
//!     region_code: Some("CA".into()),
//!     city: Some("San Diego".into()),
//!     latitude: 32.79769999999999,
//!     longitude: -117.23349999999999,
//!     time_zone: "America/Los_Angeles"
//! };
//!
//! assert_eq!(record, expected_value);
//!
//! geo_ip = GeoIpReader::<File>::new("v6").unwrap();
//! record = geo_ip.get_record("2a08:1450:300f:900::1003");
//!
//! expected_value = Record {
//!     dma: None,
//!     postal_code: None,
//!     country: Country::UnitedKingdom,
//!     region_code: None,
//!     city: None,
//!     latitude: 54.0,
//!     longitude: -2.0,
//!     time_zone: "Europe/London"
//! };
//! assert_eq!(record, expected_value);
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
pub mod continents;
pub mod countries;
pub mod designated_market_area;
pub mod errors;
pub mod geo_ip_reader;
pub mod time_zones;
pub mod utils;
