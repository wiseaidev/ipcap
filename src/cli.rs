#[cfg(feature = "cli")]
use clap::builder::styling::{AnsiColor, Effects, Styles};
#[cfg(feature = "cli")]
use clap::Parser;

#[cfg(feature = "cli")]
fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    name = "ipcap",
    propagate_version = true,
    styles = styles(),
    help_template = r#"{before-help}{name} {version}
{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    about=r#"
 ▄█     ▄███████▄  ▄████████    ▄████████    ▄███████▄ 
███    ███    ███ ███    ███   ███    ███   ███    ███ 
███▌   ███    ███ ███    █▀    ███    ███   ███    ███ 
███▌   ███    ███ ███          ███    ███   ███    ███ 
███▌ ▀█████████▀  ███        ▀███████████ ▀█████████▀  
███    ███        ███    █▄    ███    ███   ███        
███    ███        ███    ███   ███    ███   ███        
█▀    ▄████▀      ████████▀    ███    █▀   ▄████▀      

🌍 IPCAP CLI
============

Perform IP lookup from the command line without internet access. Retrieve information
about IP addresses, including details such as city, region, country, location, etc.

FEATURES:
  - Target: Set the IP address to lookup with the --target option.
    
USAGE:
  iplookup [OPTIONS]

EXAMPLES:
  Perform IP lookup:
    ipcap -t 8.8.8.8

For more information, visit: https://github.com/wiseaidev/ipcap
"#
)]
#[cfg(feature = "cli")]
pub struct Cli {
    #[arg(global = true, short, long)]
    pub verbose: bool,

    /// IP address to lookup.
    #[arg(short = 't', long = "target")]
    pub target: String,
}
