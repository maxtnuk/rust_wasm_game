use wasm_bindgen::prelude::*;
use wasm_games::*;
pub use ggui::gomoku::start;

#[macro_use]
pub mod util;
pub mod gl;
pub mod ggui;

#[wasm_bindgen(module = "webchat_client")]
extern {
    fn send(msg: &[u8]);
    #[wasm_bindgen(js_name = addMessage)]
    fn add_message(msg: &str);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn send_message(msg: Message) {
    // Message serialization should ALWAYS succeed
    send(&serialize(msg).unwrap());
}
#[wasm_bindgen]
pub fn main() {
    
}

#[wasm_bindgen]
pub fn recv(buffer: &[u8]) {
    
}

#[wasm_bindgen]
pub fn input(msg: &str) {
    
}
