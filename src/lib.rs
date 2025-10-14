mod framework;
mod shapes;
mod shell;

use sauron::{Program, wasm_bindgen};

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(crate::shell::Model::new());
}
