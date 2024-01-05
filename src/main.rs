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
        // Parse command-line arguments
        let args = Cli::parse();

        // Initialize IP/DNS lookup client with the specified options
        let mut geo_ip = GeoIpReader::<File>::new().unwrap();

        // Perform IP lookup based on the provided target
        if !args.target.is_empty() {
            let record = geo_ip.get_record(&args.target);
            println!("{record:#?}");
        } else {
            // Print an error message and exit if the target is missing
            eprintln!("Error: Target is required!");
            std::process::exit(1);
        }
    }
    Ok(())
}
