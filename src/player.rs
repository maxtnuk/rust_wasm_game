use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct BasePlayer{
    pub name: String,
    gamescore: GameScore,
}
impl BasePlayer{
    pub fn new(name: String) -> Self{
        BasePlayer{
            name: name,
            gamescore: HashMap::new()
        }
    }
}
impl PartialEq for BasePlayer {
    fn eq(&self, other: &BasePlayer) -> bool {
        self.name == other.name
    }
}

type GameScore=HashMap<String,GameBasic>;

#[derive(Serialize, Deserialize,Debug,PartialEq,Clone)]
pub struct GameBasic{
    wins: u32,
    looses: u32,
}

impl Default for BasePlayer{
    fn default() -> Self{
        BasePlayer{
            name: "bot".to_string(),
            gamescore: HashMap::new()
        }
    }
}
impl Default for GameBasic{
    fn default() -> Self{
        GameBasic{
            wins: 0,
            looses: 0
        }
    }
}