use std::fmt::{Display, Formatter};
use crate::codegen;
use crate::countries::Country;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Continent {
    Africa,
    Antarctica,
    Asia,
    Europe,
    NorthAmerica,
    Oceania,
    SouthAmerica
}

impl Continent {
    pub fn alphabetic_code_2(&self) -> &'static str {
        match self {
            Continent::Africa => "AF",
            Continent::Antarctica => "AN",
            Continent::Asia => "AS",
            Continent::Europe => "EU",
            Continent::NorthAmerica => "NA",
            Continent::Oceania => "OC",
            Continent::SouthAmerica => "SA"
        }
    }
}

impl Display for Continent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Continent::Africa => f.write_str("Africa"),
            Continent::Antarctica => f.write_str("Antarctica"),
            Continent::Asia => f.write_str("Asia"),
            Continent::Europe => f.write_str("Europe"),
            Continent::NorthAmerica => f.write_str("North America"),
            Continent::Oceania => f.write_str("Oceania"),
            Continent::SouthAmerica => f.write_str("South America")
        }
    }
}

impl From<&Country> for Option<Continent> {
    fn from(value: &Country) -> Self {
        codegen!("country-to-continent")
    }
}