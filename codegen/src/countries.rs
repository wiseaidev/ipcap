use std::fmt::format;
use std::fs;
use std::path::Path;
use crate::names::name_to_enum_name;

const NAMES_DATA: &str = include_str!("../countries-names.txt");
const CODES_2_DATA: &str = include_str!("../countries-two.txt");
const CODES_3_DATA: &str = include_str!("../countries-three.txt");

macro_rules! codes {
    ($data: expr, $enum_names: expr) => {
        &$data.split('\n').enumerate().map(|(i, code)|
            format!("Countries::{} => \"{}\"", $enum_names[i], code)
        ).collect::<Vec<String>>().join(",\n")
    };
}

macro_rules! codes_reverse {
    ($data: expr, $enum_names: expr) => {
        {
            let mut d = $data.split('\n').enumerate().map(|(i, code)|
                format!("\"{}\" => Some(Countries::{})", code, $enum_names[i])
            ).collect::<Vec<String>>();
            d.push("_ => None".to_string());
            d.join(",\n")
        }
    };
}

pub fn run() {
    let offset = 1;
    let names_by_line = NAMES_DATA.split("\n");
    let enum_names = names_by_line.clone().map(name_to_enum_name).collect::<Vec<String>>();

    let out_dir = std::env::var("OUT_DIR").unwrap();

    fs::write(
        format!("{out_dir}/countries-enum-values"),
        "pub enum Countries {\n".to_string()
            + &*enum_names.join(", \n\t")
            + "}"
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

    fs::write(
        format!("{out_dir}/countries-to-names"),
        "Ok(match self {".to_string()
            + &*names_by_line
                .enumerate().map(|(i, name)|
                    format!("Countries::{} => f.write_str(\"{name}\")", enum_names[i])
                ).collect::<Vec<String>>().join(",\n")
            + "}?)"
    ).unwrap();
    // panic!("{:?}", enum_names);
}