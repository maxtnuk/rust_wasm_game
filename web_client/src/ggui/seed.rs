use wasm_bindgen::prelude::*;
use crate::gl::get_canvas;

pub fn start() {
    let context = get_canvas("inner2").unwrap();
}
