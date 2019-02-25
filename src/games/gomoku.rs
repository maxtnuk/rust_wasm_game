use crate::player::BasePlayer;
use crate::{BoardState,Board,Input};

pub struct Gomoku{
    sm: Space
}
impl Board for Gomoku{
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

pub struct Space{
    size: u32,
    space: Vec<CellState>,
    players: [BasePlayer;2],//0:black  1: white
    stage: u32,
    playerturn: [usize;2],
    point: u32
}

enum CellState{
    White,
    Black,
    Empty,
}

 //(row=x,col=y)
type Pos= (usize,usize);

impl Space{
    
    fn get_index(&self,pos: Pos) -> usize{
        pos.0 + pos.1 * self.size as usize  
    }
    
    fn cell_change(&mut self,pos: Pos,state: CellState){
        let index = self.get_index(pos);
        self.space[index]=state;
    }
    
    fn avail_place(&self,sel: Pos){
        
    }
    fn swap(&mut self){
        self.playerturn[0]=1;
        self.playerturn[1]=0;
    }
    fn cur_player(&self)->BasePlayer{
        let arridx=(self.stage as usize+1) % 2;
        let index = self.playerturn[arridx];
        self.players[index].clone()
    }
    
    fn place_stone(&mut self,who: BasePlayer,pos: Pos){
        
    }
}

impl Space{
     fn new(size: u32) ->Self{
        Space{
            size: 19,
            space: Vec::new(),
            players: Default::default(),
            stage: 1,
            playerturn: [0,1],
            point: 0 
        }
    }
    fn next_round(&mut self){
        let cur_player=self.cur_player();
        match self.stage{
            1 => {
                
            },
            2 => {
                
            },
            3 => {
                
            },
            4 => {
                
            },
            5 => {
                
            },
            _ => {
                
            },
        }
        self.stage+=1;
    }
}


