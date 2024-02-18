/// The main entry point of `ipcap`.
///
/// It parses command-line arguments using the `clap` crate, configures an IP lookup client based on
/// the provided command-line options, and performs an IP lookup using the specified target.
///
/// # Arguments
/// * `--target` - The IP address to be looked up.
///
/// # Examples
/// ```
/// // Run the `ipcap` CLI with an IP address and display results in concise format.
/// ipcap --target "8.8.8.8"
///
/// ```
///
/// # Errors
/// The function handles errors gracefully and prints out error messages if the IP lookup fails,
/// if the target is missing, etc.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "cli")]
    {
        use clap::Parser;
        use ipcap::cli::Cli;
        use ipcap::geo_ip_reader::GeoIpReader;
        use ipcap::utils::pretty_print_dict;
        use std::fs::File;
        use std::net::{Ipv4Addr, Ipv6Addr};
        // Parse command-line arguments
        let args = Cli::parse();

        // auto detect ip address type

        // Perform IP lookup based on the provided target
        if !args.target.is_empty() {
            match args.target.parse::<Ipv4Addr>() {
                Ok(_ipv4_addr) => {
                    let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();

                    let record = geo_ip.get_record(&args.target);
                    pretty_print_dict(record);
                }
                Err(_) => {
                    // Not an IPv4 address, try IPv6
                    match args.target.parse::<Ipv6Addr>() {
                        Ok(_ipv6_addr) => {
                            let mut geo_ip = GeoIpReader::<File>::new("v6").unwrap();

                            let record = geo_ip.get_record(&args.target);
                            pretty_print_dict(record);
                        }
                        Err(_) => {
                            // todo
                        }
                    }
                }
            }
        } else {
            // Print an error message and exit if the target is missing
            return Err("Target is required!".into());
        }
    }
    Ok(())
}
