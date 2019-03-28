#[allow(dead_code)]
pub mod player;
#[allow(dead_code)]
pub mod games;

use bincode::{deserialize as bin_de, serialize as bin_ser, Error};
use serde_derive::{Deserialize, Serialize};
pub use crate::player::BasePlayer;
use crate::games::*;

pub trait Board{
    fn input(&mut self, input: Input);
    fn update(&mut self);
    fn ready(&mut self);
    fn gm_state(&self) -> BoardState;
}
#[derive(PartialEq)]
pub enum BoardState{
    Nothing,
    Turn(BasePlayer),
    Ready,
    Relay,
    Win(BasePlayer),
    Etc(String)
}

pub struct Input{
    name: String,
    content: Message
}
impl Input{
    pub fn new(name: String, content: Message) -> Self{
        Input {
            name: name,
            content: content
        }
    }
}
#[derive(Serialize, Deserialize,Debug,PartialEq,Clone)]
pub enum GameList{
    Gomoku,
    Seed,
}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Message{
    Ready,
    Destroy,
    Success,
    Fail(String),
    Update,
    Login(String),
    Waiting(GameList),
    Disconnect,
    Click(u32,u32),
    Key(Arrow,(u32,u32)),
}
pub fn game_board(gm_mode: GameList) -> Box<dyn Board>{
    match gm_mode{
        GameList::Gomoku => {
            Box::new(Gomoku::new())
        },
        GameList::Seed => {
            Box::new(Seed::new())
        }
    }
}
#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Arrow{
    Up,
    Down,
    Left,
    Right
}

pub fn serialize(message: Message) -> Result<Vec<u8>, Error> {
    bin_ser(&message)
}

pub fn deserialize(buffer: &[u8]) -> Result<Message, Error> {
    bin_de(buffer)
}
