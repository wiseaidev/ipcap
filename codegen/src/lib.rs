use crate::names::name_to_enum_name;
use std::fs;

mod names;

const NAMES_DATA: &str = include_str!("../countries-names.txt");
const CODES_2_DATA: &str = include_str!("../countries-two.txt");
const CODES_3_DATA: &str = include_str!("../countries-three.txt");
const COUNTRIES_TO_CONTINENTS: &str = include_str!("../countries-to-continents.txt");

macro_rules! codes {
    ($data: expr, $enum_names: expr) => {
        &$data
            .split('\n')
            .enumerate()
            .map(|(i, code)| format!("Country::{} => \"{}\"", $enum_names[i], code))
            .collect::<Vec<String>>()
            .join(",\n")
    };
}

macro_rules! codes_reverse {
    ($data: expr, $enum_names: expr) => {{
        let mut d = $data
            .split('\n')
            .enumerate()
            .map(|(i, code)| format!("\"{}\" => Some(Country::{})", code, $enum_names[i]))
            .collect::<Vec<String>>();
        d.push("_ => None".to_string());
        d.join(",\n")
    }};
}

macro_rules! save_content {
    ($data: expr, $value: expr, $file_name: expr) => {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        fs::write(
            format!("{out_dir}/{}", $file_name),
            format!(
                r#"match {} {{
                    {}
                }}"#,
                $value,
                $data
            ),
        )
        .unwrap();
    };
}

macro_rules! codes_with_fs_write {
    (reverse $data: expr, $enum_names: expr, $file_name: expr) => {
        save_content!(codes_reverse!($data, $enum_names), "value", $file_name);
    };

    ($data: expr, $enum_names: expr, $file_name: expr) => {
        save_content!(codes!($data, $enum_names), "self", $file_name);
    };
}

pub fn run() {
    const OFFSET: u8 = 1;
    let names_by_line = NAMES_DATA.split("\n");
    let enum_names = names_by_line
        .clone()
        .map(name_to_enum_name)
        .collect::<Vec<String>>();

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let enum_content = &*enum_names.join(", \n\t");
    fs::write(
        format!("{out_dir}/countries-enum-values"),
        format!(
            r#"
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub enum Country {{
                {enum_content}
            }}"#
        ),
    )
    .unwrap();

    codes_with_fs_write!(CODES_2_DATA, enum_names, "countries-code-2");
    codes_with_fs_write!(CODES_3_DATA, enum_names, "countries-code-3");
    codes_with_fs_write!(reverse CODES_2_DATA, enum_names, "countries-codes-2-reverse");
    codes_with_fs_write!(reverse CODES_3_DATA, enum_names, "countries-codes-3-reverse");

    let match_pattern = names_by_line
        .clone()
        .enumerate()
        .map(|(i, name)| format!("Country::{} => f.write_str(\"{name}\")", enum_names[i]))
        .collect::<Vec<String>>()
        .join(",\n");
    fs::write(
        format!("{out_dir}/countries-to-names"),
        format!(
            r"Ok(match self {{
                {match_pattern}
            }}?)"
        ),
    )
    .unwrap();

    let match_pattern = enum_names
        .iter()
        .enumerate()
        .map(|(i, name)| format!("{} => Some(Country::{name})", i as u8 + OFFSET))
        .collect::<Vec<String>>()
        .join(",\n");
    fs::write(
        format!("{out_dir}/countries-from-buffer"),
        format!(
            r#"match value {{
                {match_pattern},
                _ => None
            }}"#
        ),
    )
    .unwrap();

    let match_pattern = COUNTRIES_TO_CONTINENTS
        .split('\n')
        .enumerate()
        .map(|(i, continent)| {
            if continent.is_empty() {
                format!("Country::{} => None", enum_names[i])
            } else {
                format!("Country::{} => Some(Continent::{continent})", enum_names[i])
            }
        })
        .collect::<Vec<String>>()
        .join(",\n");
    fs::write(
        format!("{out_dir}/country-to-continent"),
        format!(
            r#"match value {{
               {match_pattern}
            }}"#
        ),
    )
    .unwrap();
}
