use crate::WebMsg;
use std::hash::{Hash, Hasher};
use wasm_games::{BasePlayer, Input, Message};
use ws::Sender;

#[derive(PartialEq, Clone)]
pub struct Player {
    info: BasePlayer,
    soc: Option<Sender>,
}

impl Player {
    pub fn new(name: String, soc: Sender) -> Self {
        Player {
            info: BasePlayer::new(name),
            soc: Some(soc),
        }
    }
    pub fn send_msg(&self, msg: Message) {
        if let Some(soc) = self.soc.clone() {
            soc.send(WebMsg(msg)).unwrap();
        }
    }
    pub fn get_name(&self) -> String {
        self.info.name.clone()
    }
    pub fn make_input(&self, msg: Message) -> Input {
        //need fix
        Input::new(self.info.get_name(), msg)
    }
}
impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(soc) = self.soc.clone() {
            soc.token().hash(state);
        }
    }
}
impl ToString for Player {
    fn to_string(&self) -> String {
        self.get_name()
    }
}
impl Default for Player {
    fn default() -> Player {
        Player {
            info: Default::default(),
            soc: None,
        }
    }
}
