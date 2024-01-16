use std::fmt::{Display, Formatter};
use crate::codegen;

struct DesignatedMarketArea(i32);

impl Display for DesignatedMarketArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        codegen!("dma-code-to-name")
    }
}
