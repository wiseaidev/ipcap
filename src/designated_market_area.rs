use crate::codegen;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DesignatedMarketArea(pub u32);

impl DesignatedMarketArea {
    pub fn dma_code(&self) -> u32 {
        self.0 / 1000
    }

    pub fn area_code(&self) -> u32 {
        self.0 % 1000
    }
}

impl Display for DesignatedMarketArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = self.dma_code();
        codegen!("dma-code-to-name")
    }
}
