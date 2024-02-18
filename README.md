# 🌍 IPCap

[![Crates.io](https://img.shields.io/crates/v/ipcap.svg)](https://crates.io/crates/ipcap)
[![docs](https://docs.rs/ipcap/badge.svg)](https://docs.rs/ipcap/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

```sh
ipcap 0.1.6

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

Download the city databases, v4 and v6 from the repository using this command:

```sh
# IPV4 database
curl -LS https://raw.githubusercontent.com/wiseaidev/ipcap/main/data/geo_ip_city_v4.dat --create-dirs -o ~/ipcap/geo_ip_city_v4.dat

# IPV6 database
curl -LS https://raw.githubusercontent.com/wiseaidev/ipcap/main/data/geo_ip_city_v6.dat --create-dirs -o ~/ipcap/geo_ip_city_v6.dat
```

This will download the `data/geo_ip_city_v4.dat` and or `data/geo_ip_city_v4.dat` database(s) from the repository and put it under `~/ipcap/`.

If, for some reason, you decide to change this file location, just set this environment variable to help the CLI read this file. To set the environment variable before running your Rust program, you can do something like:

```sh
# IPV4 database
export IPCAP_FILE_PATH=/your/custom/path/geo_ip_city_v4.dat

# IPV6 database
export IPCAP_FILE_PATH=/your/custom/path/geo_ip_city_v6.dat
```

Replace `/your/custom/path/geo_ip_city_v4.dat` with the desired file path. If the environment variable is not set, the program will use the default path (`/home/username/ipcap/geo_ip_city_v4.dat`).

> [!NOTE]
The databases were shamelessly taken from the fedora website at [https://src.fedoraproject.org/repo/pkgs/GeoIP-GeoLite-data/](https://src.fedoraproject.org/repo/pkgs/GeoIP-GeoLite-data/).

## About the IPV4 dataset

The last 600 bytes of this dataset:

```sh
  -46   29  -75  -66   42  103   50   56    0   75  111  108  107   97
  116   97    0   55   52   51   49   50   53    0  -31  -24   30   49
  -13   40  103   50   56    0   67  104   97  110  100   97  110  110
   97  103   97  114    0   55   49   50   49   51   55    0 -108  -12
   30  124  -13   40  103   50   56    0   72  111  111  103  104  108
  121    0   55   49   50   49   50   49    0 -100  -11   30  121  -12
   40  103   49   49    0   66  104   97  114  109  111  117  114    0
   49   55   54   51   49   53    0  -44  106   32  -43   36   39  103
   49   48    0   66   97  110  119   97  115   97    0    0  -84  -23
   31    4   38   39  103   49   49    0   68   97  100   97  104  117
    0   49   55   51   48   50   50    0   55   35   32   -3   71   39
  103   50   51    0   82  117  112  110   97  103   97  114    0    0
  -32   48   32  -45   36   39  103   49   57    0   78   97  103   97
  114    0    0  -99 -109   29 -105  -23   38  100   48   52    0   75
  101  109   97  110  103  103  105  115   97  110    0    0   90 -123
   26  -80  -62   43    0    0    0   71   69   79   45   53   51   51
   76   73   84   69   32   50   48   49   54   48   52   48   53   32
   66  117  105  108  100   32   49   32   67  111  112  121  114  105
  103  104  116   32   40   99   41   32   50   48   49   54   32   77
   97  120   77  105  110  100   32   73  110   99   32   65  108  108
   32   82  105  103  104  116  115   32   82  101  115  101  114   -1
   -1   -1    2   52  -67   41
```

can be interpreted as follows:

```sh
^00 Shek Kip Mei       
24 Phayakkhaphum Phisai 44110 _   6+ 70 Yala 95120 
Jt   * 05 Ban Hong   -   * 44 Bang Khla 24110      
* 68 Ban Na   v   * 77 Amnat Charoen 37000    
Mp+ 68 Ban Phru  _z   * 68 Chana 90160  ~   
* 46 Si Racha 20110  y   * 02 Chiang Mai 50250  
W   * 16 Nakhon Sawan 60130      *g28 Kolkata 743125    
1 (g28 Chandannagar 712137    | (g28 Hooghly 712121    
y (g11 Bharmour 176315  j  $'g10 Banwasa      &'g11 Dadahu 173022 
7#  G'g23 Rupnagar   0  $'g19 Nagar       &d04 Kemanggisan  Z    
+   GEO-533LITE 20160405 Build 1 Copyright (c) 2016 MaxMind Inc All Rights Reserved    4
```

The forth byte from the end of the file, '2', indicates the database type as the GeoLite City database `CITY_EDITION_REV1`. This dataset provides geolocation information, featuring diverse locations, numeric codes, and associated details. Place names, such as "Shek Kip Mei" and "Chiang Mai," and numeric codes, postal codes, contribute to the dataset's geographical context. Additionally, metadata elements like "GEO-533LITE" and a copyright statement suggest a connection to the MaxMind geolocation database. The dataset aims to offer insights into the geographical distribution of locations and is potentially valuable for geospatial analysis, like this project.

## ✨ Features

- Auto detect ipv4 and ipv6.
- IP address lookup without internet access.
- Zero API calls for decoding IP addresses.
- Dataset download and customizable file path.

## ⌨ Usage as CLI

### Perform IPV4 lookup:

```sh
ipcap -t 8.8.8.8
```

### Perform IPV6 lookup:

```sh
ipcap -t 2a08:1450:300f:900::1003
```

## 💻 Usage as Dep

```toml
[dependencies]
ipcap = "0.1.6"
```

```rust
use ipcap::geo_ip_reader::GeoIpReader;
use ipcap::utils::pretty_print_dict;
use std::fs::File;

fn main() {
    let mut geo_ip = GeoIpReader::<File>::new("v4").unwrap();
    let mut record = geo_ip.get_record("8.8.8.8");

    pretty_print_dict(record);

    geo_ip = GeoIpReader::<File>::new("v6").unwrap();
    record = geo_ip.get_record("2a08:1450:300f:900::1003");

    pretty_print_dict(record);
}
```

## 🎨 Options

| Option                   | Default Value | Description                                              |
|--------------------------|---------------|----------------------------------------------------------|
| `--target`               | `""`          | Set the IP address, v4 or v6, to lookup with the --target option. |
  
## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/ipcap).
Your contributions help improve this CLI for the community.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
