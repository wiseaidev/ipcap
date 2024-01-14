use std::fmt::{Display, Formatter};
use crate::codegen;
use crate::continents::Continent;

codegen!(statement; "countries-enum-values");

impl Country {
    pub fn from_buffer(value: u8) -> Option<Self> {
        codegen!("countries-from-buffer")
    }

    /// ```rust
    /// use ipcap::countries::Country;
    /// let country = Country::Poland;
    ///
    /// assert_eq!(country.alphabetic_code_2(), "PL")
    /// ```
    pub fn alphabetic_code_2(&self) -> &'static str {
        codegen!("countries-codes-2")
    }

    /// ```rust
    /// use ipcap::countries::Country;
    /// let country = Country::Poland;
    ///
    /// assert_eq!(country.alphabetic_code_3(), "POL")
    /// ```
    pub fn alphabetic_code_3(&self) -> &'static str {
        codegen!("countries-codes-3")
    }

    /// ```rust
    /// use ipcap::countries::Country;
    ///
    /// assert_eq!(Country::from_alphabetic_code_2("PL"), Some(Country::Poland))
    /// ```
    pub fn from_alphabetic_code_2(value: &str) -> Option<Self> {
        codegen!("countries-codes-2-reverse")
    }

    /// ```rust
    /// use ipcap::countries::Country;
    ///
    /// assert_eq!(Country::from_alphabetic_code_3("POL"), Some(Country::Poland))
    /// ```
    pub fn from_alphabetic_code_3(value: &str) -> Option<Self> {
        codegen!("countries-codes-3-reverse")
    }

    /// ```rust
    /// use ipcap::continents::Continent;
    /// use ipcap::countries::Country;
    ///
    /// assert_eq!(Country::Poland.continent(), Some(Continent::Europe))
    /// ```
    pub fn continent(&self) -> Option<Continent> {
        self.into()
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        codegen!("countries-to-names")
    }
}