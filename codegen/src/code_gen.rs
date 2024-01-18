use crate::names::_name_to_enum_name;
use std::fs;

const NAMES_DATA: &str = include_str!("../countries-names.txt");
const CODES_2_DATA: &str = include_str!("../countries-two.txt");
const CODES_3_DATA: &str = include_str!("../countries-three.txt");
const COUNTRIES_TO_CONTINENTS: &str = include_str!("../countries-to-continents.txt");
const DMA: &str = include_str!("../dma.txt");

trait PushMut<T> {
    fn add(self, item: T) -> Self;
}

impl<T> PushMut<T> for Vec<T> {
    fn add(self, item: T) -> Self {
        let mut new = self;
        new.push(item);
        new
    }
}

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
        $data
            .split('\n')
            .enumerate()
            .map(|(i, code)| format!("\"{}\" => Some(Country::{})", code, $enum_names[i]))
            .collect::<Vec<String>>()
            .add("_ => None".to_string())
            .join(",\n")
    }};
}

macro_rules! _save_content {
    ($data: expr, $file_name: expr) => {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        fs::write(
            format!("{out_dir}/{}", $file_name),
            $data
        )
        .unwrap();
    };
}

macro_rules! save_content {
    (result $data: expr, $value: expr, $file_name: expr) => {
        _save_content!(
            format!(
                r#"Ok(match {} {{
                    {}
                }}?)"#,
                $value,
                $data
            ),
            $file_name
        )
    };

    ($data: expr, $value: expr, $file_name: expr) => {
        _save_content!(
            format!(
                r#"match {} {{
                    {}
                }}"#,
                $value,
                $data
            ),
            $file_name
        )
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
        .map(_name_to_enum_name)
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

    codes_with_fs_write!(CODES_2_DATA, enum_names, "countries-codes-2");
    codes_with_fs_write!(CODES_3_DATA, enum_names, "countries-codes-3");
    codes_with_fs_write!(reverse CODES_2_DATA, enum_names, "countries-codes-2-reverse");
    codes_with_fs_write!(reverse CODES_3_DATA, enum_names, "countries-codes-3-reverse");

    let match_pattern = names_by_line
        .clone()
        .enumerate()
        .map(|(i, name)| format!("Country::{} => f.write_str(\"{name}\")", enum_names[i]))
        .collect::<Vec<String>>()
        .join(",\n");
    save_content!(result match_pattern, "self", "countries-to-names");

    let match_pattern = enum_names
        .iter()
        .enumerate()
        .map(|(i, name)| format!("{} => Some(Country::{name})", i as u8 + OFFSET))
        .collect::<Vec<String>>()
        .add("_ => None".to_string())
        .join(",\n");
    save_content!(match_pattern, "value", "countries-from-buffer");

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
    save_content!(match_pattern, "value", "country-to-continent");

    let match_pattern = DMA.split('\n')
        .map(|dma| {
            let data: Vec<&str> = dma.split("; ").collect();
            format!("{} => f.write_str(\"{}\")", data[0], data[1])
        })
        .collect::<Vec<String>>()
        .add("_ => f.write_str(\"Unknown DMA\")".to_string())
        .join(",\n");
    save_content!(match_pattern, "value", "dma-code-to-name");
}
