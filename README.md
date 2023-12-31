# 🌍 IPCap

[![Crates.io](https://img.shields.io/crates/v/ipcap.svg)](https://crates.io/crates/ipcap)
[![docs](https://docs.rs/ipcap/badge.svg)](https://docs.rs/ipcap/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

```sh
ipcap 0.1.1

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
```

## 📖 Table of Contents

- [Installation](#-installation)
- [Features](#-features)
- [Usage](#-usage-as-cli)
- [Options](#-options)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install `ipcap`, use the following Cargo command:

```bash
cargo install --locked ipcap --all-features
```

## 📖 Download the dataset

Download the city database from the repository using this command:

```sh
curl -LS https://raw.githubusercontent.com/wiseaidev/ipcap/main/data/geo_ip_city.dat --create-dirs -o ~/ipcap/geo_ip_city.dat
```

This will download the `data/geo_ip_city.dat` from the repository and put it under `~/ipcap/geo_ip_city.dat`.

If, for some reason, you decide to change this file location, just set this environment variable to help the CLI read this file. To set the environment variable before running your Rust program, you can do something like:

```sh
export IPCAP_FILE_PATH=/your/custom/path/geo_ip_city.dat
```

Replace `/your/custom/path/geo_ip_city.dat` with the desired file path. If the environment variable is not set, the program will use the default path (`/home/username/ipcap/geo_ip_city.dat`).

> [!NOTE]
The dataset was shamelessly taken from the fedora website at [https://src.fedoraproject.org/repo/pkgs/GeoIP-GeoLite-data/GeoLiteCity.dat.gz](https://src.fedoraproject.org/repo/pkgs/GeoIP-GeoLite-data/GeoLiteCity.dat.gz/01968fd152251b98874ee0a8d254f4ab/).

## ✨ Features

- IP address lookup without internet access.
- Zero API calls for decoding IP addresses.
- Dataset download and customizable file path.

## ⌨ Usage as CLI

### Perform IP lookup:

```sh
ipcap -t 8.8.8.8
```

## 💻 Usage as Dep

```toml
[dependencies]
ipcap = "0.1.1"
```

```rust
use ipcap::geo_ip_reader::GeoIpReader;
use std::fs::File;

fn fn main() {
    let mut geo_ip = GeoIpReader::<File>::new().unwrap();
    let record = geo_ip.get_record("8.8.8.8");

    println!("{:?}", record);
}
```

## 🎨 Options

| Option                   | Default Value | Description                                              |
|--------------------------|---------------|----------------------------------------------------------|
| `--target`               | `""`          | Set the IP address to lookup with the --target option. |
  
## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/ipcap).
Your contributions help improve this CLI for the community.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
