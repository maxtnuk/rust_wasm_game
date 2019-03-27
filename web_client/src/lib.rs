pub use crate::ggui::start_game;
use wasm_bindgen::prelude::*;
use wasm_games::*;

#[macro_use]
pub(crate) mod util;
pub mod ggui;
pub mod gl;

#[wasm_bindgen(module = "webchat_client")]
extern "C" {
    fn send(msg: &[u8]);
    #[wasm_bindgen(js_name = addMessage)]
    fn add_message(msg: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn send_message(msg: Message) {
    // Message serialization should ALWAYS succeed
}
#[wasm_bindgen]
pub fn main() {}

#[wasm_bindgen]
pub fn recv(buffer: &[u8]) {}
#[wasm_bindgen]
pub fn gen_jsvalue() -> JsValue {
    let test = GameList::Gomoku;
    JsValue::from_serde(&test).unwrap()
}

#[wasm_bindgen]
pub fn input(msg: &str) {}
