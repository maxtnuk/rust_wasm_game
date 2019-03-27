use crate::gm::RoomController;
use crate::user::Player;
use std::collections::VecDeque;
use wasm_games::GameList;

pub struct LobbyManager {
    wating_user: VecDeque<Player>,
}
impl LobbyManager {
    pub fn new() -> Self {
        LobbyManager {
            wating_user: VecDeque::new(),
        }
    }
    pub fn check(&self, name: String) -> bool {
        if self
            .wating_user
            .iter()
            .find(|&x| x.to_string() == name)
            .is_some()
        {
            println!("user {} already exist", name);
            true
        } else {
            false
        }
    }
    pub fn push(&mut self, who: &Player) {
        let name = who.get_name();
        if !self.check(name) {
            self.wating_user.push_back(who.clone());
        }
    }
    pub fn dispatch(&mut self, rmmaker: &mut RoomController, gmlist: GameList) {
        while self.wating_user.len() > 1 {
            let player0 = self.wating_user.pop_front().unwrap();
            let player1 = self.wating_user.pop_front().unwrap();

            rmmaker.create(vec![player0, player1], gmlist.clone());
        }
    }
    pub fn kick(&mut self, who: &Player) {
        if let Some(index) = self.wating_user.iter().position(|r| *r == *who) {
            self.wating_user.remove(index).unwrap();
        }
    }
}
