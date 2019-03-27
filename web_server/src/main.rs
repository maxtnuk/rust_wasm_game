pub mod gm;
pub mod lobby;
pub mod user;

use gm::GameManager;
use lobby::LobbyManager;
use std::sync::{Arc, Mutex};
use user::Player;
use wasm_games::Message;
use wasm_games::{deserialize, serialize};
use ws::{listen, CloseCode, Error, Handler, Handshake, Message as WsMsg, Result, Sender};

pub struct WebMsg(Message);
impl Into<WsMsg> for WebMsg {
    fn into(self) -> WsMsg {
        WsMsg::Binary(serialize(self.0).unwrap())
    }
}
type ArcMut<T> = Arc<Mutex<T>>;
struct Server {
    out: Sender,
    user: Option<Player>,
    gmm: ArcMut<GameManager>,
    lbm: ArcMut<LobbyManager>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: WsMsg) -> Result<()> {
        if let ws::Message::Binary(buffer) = msg {
            if let Ok(msg) = deserialize(&buffer) {
                match msg {
                    Message::Ready => {
                        let mut gmm = self.gmm.lock().unwrap();
                        if let Some(usr) = self.user.clone() {
                            if let Some(room) = gmm.room.get_room(usr.to_string()) {
                                room.ready();
                            }
                        }
                    }
                    Message::Destroy => {
                        let mut gmm = self.gmm.lock().unwrap();
                        let mut lbm = self.lbm.lock().unwrap();

                        if let Some(usr) = self.user.clone() {
                            if gmm.room.get_room(usr.to_string()).is_none() {
                                lbm.kick(&usr);
                            } else {
                                let room_name =
                                    gmm.room.get_room(usr.to_string()).unwrap().get_id();
                                gmm.room.destroy(room_name);
                            }
                        }
                    }
                    Message::Update => {
                        let mut gmm = self.gmm.lock().unwrap();
                        if let Some(usr) = self.user.clone() {
                            if let Some(room) = gmm.room.get_room(usr.to_string()) {
                                room.update();
                            }
                        }
                    }
                    Message::Login(name) => {
                        let lbm = self.lbm.lock().unwrap();
                        if lbm.check(name.clone()) {
                            println!("already exist");
                            self.out
                                .send(WebMsg(Message::Fail("fail to make user".to_string())))
                                .unwrap();
                        } else {
                            let user = Player::new(name.clone(), self.out.clone());
                            user.send_msg(Message::Login(name));
                            self.user = Some(user);
                        }
                    }
                    Message::Waiting(game) => {
                        let mut lbm = self.lbm.lock().unwrap();
                        let mut gmm = self.gmm.lock().unwrap();
                        if let Some(usr) = self.user.clone() {
                            lbm.push(&usr);
                            lbm.dispatch(&mut gmm.room, game);
                        }
                    }
                    e @ _ => {
                        let mut gmm = self.gmm.lock().unwrap();
                        if let Some(usr) = self.user.clone() {
                            if let Some(room) = gmm.room.get_room(self.get_name()) {
                                let user_input = usr.make_input(e);
                                room.input(user_input);
                            }
                        }
                    }
                }
                Ok(())
            } else {
                Err(ws::Error::new(
                    ws::ErrorKind::Internal,
                    "Deserialization error",
                ))
            }
        } else {
            Err(ws::Error::new(
                ws::ErrorKind::Internal,
                "Not a binary message",
            ))
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        let mut gmm = self.gmm.lock().unwrap();
        let mut lbm = self.lbm.lock().unwrap();

        match code {
            CloseCode::Away => {
                println!("The client is leaving the site.");
                {
                    let rmm = &mut gmm.room;
                    let room_name = if let Some(room) = rmm.get_room(self.get_name()) {
                        room.get_id()
                    } else {
                        "".to_string()
                    };
                    rmm.destroy(room_name);
                    lbm.kick(&self.user.clone().unwrap());
                }
            }
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}
impl Server {
    fn get_name(&self) -> String {
        match self.user.clone() {
            Some(who) => who.to_string(),
            None => "".to_string(),
        }
    }
}

fn main() {
    let gmm = Mutex::new(GameManager::new());
    let lbm = Mutex::new(LobbyManager::new());
    let rc_gmm = Arc::new(gmm);
    let rc_lbm = Arc::new(lbm);

    listen("0.0.0.0:8080", |out| Server {
        out: out,
        gmm: Arc::clone(&rc_gmm),
        lbm: Arc::clone(&rc_lbm),
        user: None,
    })
    .unwrap();
}
