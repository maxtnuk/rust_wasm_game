pub use crate::ggui::start_game;
use wasm_bindgen::prelude::*;
use wasm_games::*;

#[macro_use]
pub(crate) mod util;
pub mod ggui;
pub mod gl;

#[wasm_bindgen]
pub fn main() {}

#[wasm_bindgen]
pub fn render() {
    
}
