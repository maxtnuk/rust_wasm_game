use crate::player::BasePlayer;
use crate::{BoardState,Board,Input};

pub struct Seed{
    dummy: u32
}
impl Seed{
    pub fn new() ->Self{
        Seed{
            dummy: 0
        }
    }
}
impl Board for Seed{
    fn input(&mut self, input: Input){

    }
    fn update(&mut self){

    }
    fn ready(&mut self){

    }
    fn gm_state(&self) -> BoardState{
        BoardState::Nothing
    }
}