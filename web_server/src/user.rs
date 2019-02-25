use wasm_games::{BasePlayer,Input};
use crate::WebMsg;
use ws::{Sender,Message};
use std::hash::{Hash,Hasher};

#[derive(Default)]
pub struct Player{
    info: BasePlayer,
    soc: Sender
}
impl Player{
    fn new(name: String, soc: Sender) -> Self{
        Player{
            info: BasePlayer::new(name),
            soc: soc
        }
    }
    pub fn send_msg(&self,msg: Message){
        self.soc.send(WebMsg(msg));
    }
    pub fn get_name(&self)->String{
        self.info.name.clone()
    }
    pub fn make_input(&self,msg: Message)->Input{
        Input{
            name:self.info.name.clone(),
            content: msg
        }
    }
}
impl Hash for Player{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.soc.connection_id().hash(state);
    }
}
impl ToString for Player{
    fn to_string(&self) -> String{
        self.soc.connection_id().to_string()
    }
}