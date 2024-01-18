#[path = "src/codegen.rs"]
pub mod codegen;

#[path = "src/names.rs"]
pub mod names;

use codegen::run;

fn main() {
    run()
}
