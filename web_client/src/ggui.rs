use wasm_games::GameList;
use wasm_bindgen::prelude::*;

pub mod gomoku;

#[wasm_bindgen]
pub fn start_game(js_objects: &JsValue){
    let gtype: GameList = js_objects.into_serde().unwrap();
    match gtype{
        GameList::Gomoku => {
            gomoku::start();        
        }
    }
}