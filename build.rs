#[path = "codegen/src/code_gen.rs"]
pub mod code_gen;

use codegen::run;

fn main() {
    run()
}
