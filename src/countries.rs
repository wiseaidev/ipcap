use std::fmt::{Display, Formatter};
use crate::codegen;
use crate::continents::Continent;

codegen!("countries-enum-values");

impl Country {
    pub fn from_buffer(value: u8) -> Option<Self> {
        codegen!("countries-from-buffer")
    }

    /// ```rust
    /// let country = Country::Poland;
    ///
    /// assert_eq!(country.alphabetic_code_2(), "PL")
    /// ```
    pub fn alphabetic_code_2(&self) -> &'static str {
        codegen!("countries-codes-2")
    }

    pub fn alphabetic_code_3(&self) -> &'static str {
        codegen!("countries-codes-3")
    }

    pub fn from_alphabetic_code_2(value: &str) -> Option<Self> {
        codegen!("countries-codes-2-reverse")
    }

    pub fn from_alphabetic_code_3(value: &str) -> Option<Self> {
        codegen!("countries-codes-3-reverse")
    }

    pub fn continent(&self) -> Option<Continent> {
        self.into()
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        codegen!("countries-to-names")
    }
}