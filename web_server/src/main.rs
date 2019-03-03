pub mod gm;
pub mod user;
pub mod lobby;

use wasm_games::Message;
use wasm_games::games::Gomoku;
use wasm_games::{serialize, deserialize};
use wasm_games::Board;
use std::env;
use std::sync::{Arc, Mutex};
use ws::{listen, Handler, Sender, Result, Message as WsMsg, Handshake, CloseCode, Error};
use user::Player;
use gm::GameManager;
use lobby::LobbyManager;

pub struct WebMsg(Message);
impl Into<WsMsg> for WebMsg {
    fn into(self) -> WsMsg {
        WsMsg::Binary(serialize(self.0).unwrap())
    }
}
type ArcMut<T> = Arc<Mutex<T>>;
struct Server<T>
where
    T: Board,
{
    out: Sender,
    user: Option<Player>,
    gmm: ArcMut<GameManager<T>>,
    lbm: ArcMut<LobbyManager>,
}

impl<T> Handler for Server<T>
where
    T: Board,
{
    fn on_open(&mut self, _: Handshake) -> Result<()> {}

    fn on_message(&mut self, msg: WsMsg) -> Result<()> {
        if let ws::Message::Binary(buffer) = msg {
            if let Ok(msg) = deserialize(&buffer) {

                match msg {
                    Message::Ready => {
                        let gmm = self.gmm.lock().unwrap();
                        if let Some(user) = self.user {
                            if let Some(room) = gmm.room.get_room(user.to_string()) {
                                room.ready();
                            }
                        }
                    }
                    Message::Destroy => {
                        let gmm = self.gmm.lock().unwrap();
                        let lbm = self.lbm.lock().unwrap();

                        if let Some(user) = self.user {
                            if let Some(room) = gmm.room.get_room(user.to_string()) {
                                gmm.room.destroy(room.get_id());
                                lbm.kick(user);
                            }
                        }
                    }
                    Message::Update => {
                        let gmm = self.gmm.lock().unwrap();
                        if let Some(user) = self.user.unwrap() {
                            if let Some(room) = gmm.room.get_room(user.to_string()) {
                                room.update();
                            }
                        }
                    }
                    Message::Login(name) => {
                        let lbm = self.lbm.lock().unwrap();
                        if lbm.check(name) {
                            println!("already exist");
                            self.out.send(WebMsg(
                                Message::Fail("fail to make user".to_string()),
                            ))
                        } else {
                            let user = Player::new(name.clone(), self.out.clone());
                            user.send_msg(Message::Login(name));
                            self.user = Some(user);
                        }
                    }
                    Message::Waiting => {
                        let lbm = self.lbm.lock().unwrap();
                        let gmm = self.gmm.lock().unwrap();
                        if let Some(user) = self.user.unwrap() {
                            lbm.push(user.clone());
                            lbm.dispatch(gmm.room);
                        }
                    }
                    e @ _ => {
                        let gmm = self.gmm.lock().unwrap();
                        if let Some(user) = self.user.unwrap() {
                            if let Some(room) = gmm.room.get_room(user.to_string()) {
                                let user_input = user.make_input(e);
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
        let gmm = self.gmm.lock().unwrap();
        let lbm = self.lbm.lock().unwrap();

        match code {
            CloseCode::Away => {
                println!("The client is leaving the site.");
                {
                    let rmm = gmm.room;
                    if let Some(room) = gmm.room.get_room(self.user.to_string()) {
                        rmm.destroy(room.get_id());
                    }
                    lbm.kick(self.user.unwrap());
                }
            }
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    let gmm = Mutex::new(GameManager::new::<Gomoku>());
    let lbm = Mutex::new(LobbyManager::new());
    let rc_gmm = Arc::new(gmm);
    let rc_lbm = Arc::new(lbm);

    listen("127.0.0.1:3012", |out| {
        Server {
            out: out,
            gmm: Arc::clone(&rc_gmm),
            lbm: Arc::clone(&rc_lbm),
            user: None,
        }
    }).unwrap();
}
