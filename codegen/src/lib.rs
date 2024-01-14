use std::fs;
use crate::names::name_to_enum_name;

mod names;

const NAMES_DATA: &str = include_str!("../countries-names.txt");
const CODES_2_DATA: &str = include_str!("../countries-two.txt");
const CODES_3_DATA: &str = include_str!("../countries-three.txt");
const COUNTRIES_TO_CONTINENTS: &str = include_str!("../countries-to-continents.txt");

macro_rules! codes {
    ($data: expr, $enum_names: expr) => {
        &$data.split('\n').enumerate().map(|(i, code)|
            format!("Country::{} => \"{}\"", $enum_names[i], code)
        ).collect::<Vec<String>>().join(",\n")
    };
}

macro_rules! codes_reverse {
    ($data: expr, $enum_names: expr) => {
        {
            let mut d = $data.split('\n').enumerate().map(|(i, code)|
                format!("\"{}\" => Some(Country::{})", code, $enum_names[i])
            ).collect::<Vec<String>>();
            d.push("_ => None".to_string());
            d.join(",\n")
        }
    };
}

pub fn run() {
    const OFFSET: u8 = 1;
    let names_by_line = NAMES_DATA.split("\n");
    let enum_names = names_by_line.clone().map(name_to_enum_name).collect::<Vec<String>>();

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let enum_content = &*enum_names.join(", \n\t");
    fs::write(
        format!("{out_dir}/countries-enum-values"),
        format!(r#"
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub enum Country {{
                {enum_content}
            }}"#)
    ).unwrap();

    fs::write(
        format!("{out_dir}/countries-codes-2"),
        "match self {".to_string()
            + codes!(CODES_2_DATA, enum_names)
            + "}"
    ).unwrap();

    fs::write(
        format!("{out_dir}/countries-codes-3"),
        "match self {".to_string()
            + codes!(CODES_3_DATA, enum_names)
            + "}"
    ).unwrap();

    fs::write(
        format!("{out_dir}/countries-codes-2-reverse"),
        "match value {".to_string()
            + &*codes_reverse!(CODES_2_DATA, enum_names)
            + "}"
    ).unwrap();

    fs::write(
        format!("{out_dir}/countries-codes-3-reverse"),
        "match value {".to_string()
            + &*codes_reverse!(CODES_3_DATA, enum_names)
            + "}"
    ).unwrap();

    let match_pattern = names_by_line.clone().enumerate().map(|(i, name)|
        format!("Country::{} => f.write_str(\"{name}\")", enum_names[i])
    ).collect::<Vec<String>>().join(",\n");
    fs::write(
        format!("{out_dir}/countries-to-names"),
        format!(
            r"Ok(match self {{
                {match_pattern}
            }}?)"
        )
    ).unwrap();


    let match_pattern = enum_names.iter().enumerate().map(|(i, name)|
        format!("{} => Some(Country::{name})", i as u8 + OFFSET)
    ).collect::<Vec<String>>().join(",\n");
    fs::write(
        format!("{out_dir}/countries-from-buffer"),
        format!(
            r#"match value {{
                {match_pattern},
                _ => None
            }}"#
        )
    ).unwrap();

    let match_pattern = COUNTRIES_TO_CONTINENTS
        .split('\n').enumerate().map(|(i, continent)|
        if continent.is_empty() { format!("Country::{} => None", enum_names[i]) }
        else { format!("Country::{} => Some(Continent::{continent})", enum_names[i]) }
        ).collect::<Vec<String>>().join(",\n");
    fs::write(
        format!("{out_dir}/country-to-continent"),
        format!(
            r#"match value {{
               {match_pattern}
            }}"#
        )
    ).unwrap();
}
