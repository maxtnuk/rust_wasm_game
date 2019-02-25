use std::collections::{HashMap,VecDeque};
use crate::user::Player;
use crate::gm::RoomController;
use wasm_games::Board;

pub struct LobbyManager{
    wating_user: VecDeque<Player>,
}
impl LobbyManager{
    pub fn new() -> Self{
        LobbyManager{
            wating_user: VecDeque::new(),
        }
    }
    fn check(&self,name: String) -> bool{
        if self.userindex.contains(&name){
            println!("user {} already exist",name);
            true 
        }else{
            false
        }
    }
    pub fn push(&mut self,who: Player){
        let name=who.get_name();
        if let Some(user) = self.userindex[name]{
            self.wating_user.push_back(who);
        }
    }
    pub fn dispatch<T>(&mut self,rmmaker: RoomController<T>)
    where T: Board{
        while self.wating_user.len() > 1{
            let player0 = self.wating_user.pop_front();
            let player1 = self.wating_user.pop_front();
            
            rmmaker.create(vec![player0,player1]);
        }
    }
    pub fn kick(&mut self,who: Player){
        if let Some(index) = self.wating_user.iter().position(|&r| r == who.name.to_string()){
            self.wating_user.remove(index).unwrap();
        }
    }
} 