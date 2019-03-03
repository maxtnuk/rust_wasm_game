use wasm_games::{BasePlayer,Input,Message};
use crate::WebMsg;
use ws::{Sender};
use std::hash::{Hash,Hasher};

pub struct Player{
    info: BasePlayer,
    soc: Option<Sender>
}
impl Player{
    fn new(name: String, soc: Sender) -> Self{
        Player{
            info: BasePlayer::new(name),
            soc: Some(soc)
        }
    }
    pub fn send_msg(&self,msg: Message){
        if let Some(soc) = self.soc{
            soc.send(WebMsg(msg));
        }
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
        if let Some(soc) = self.soc{
            soc.token().hash(state);
        }
    }
}
impl ToString for Player{
    fn to_string(&self) -> String{
        self.get_name()
    }
}
impl Default for Player{
    fn default() -> Player{
        Player{
            info: Default::default(),
            soc: None
        }
    }
}
