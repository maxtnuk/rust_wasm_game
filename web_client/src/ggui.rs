use wasm_bindgen::prelude::*;
use wasm_games::GameList;

pub mod gomoku;
pub mod seed;

#[wasm_bindgen]
pub fn start_game(js_objects: &JsValue) {
    let gtype: GameList = js_objects.into_serde().unwrap();
    match gtype {
        GameList::Gomoku => {
            gomoku::start().unwrap();
        }
        GameList::Seed =>{
            seed::start()
        }
    }
}
